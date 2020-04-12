use pegatexto_vm::bytecode::{Bytecode, OwnedBytecode};
use pegatexto_vm::bytecode::address::Address;
use pegatexto_vm::bytecode::builder::Builder as BytecodeBuilder;
use pegatexto_vm::bytecode::instruction::Instruction;
use pegatexto_vm::grammar::expression::Expression;

use std::collections::HashMap;
use std::fmt::Debug;
use std::vec::Vec;

struct RuleCompileInfo {
    index: Option<i32>,
    call_addresses: Vec<Address>,
    address: Address,
}
impl RuleCompileInfo {
    fn new() -> RuleCompileInfo {
        RuleCompileInfo {
            index: None,
            call_addresses: Vec::new(),
            address: Address::default(),
        }
    }
}

pub struct Compiler {
    builder: BytecodeBuilder,
    rulemap: HashMap<String, RuleCompileInfo>,
}

#[derive(Debug)]
pub enum CompileError {
    EmptyGrammar,
}

impl Compiler {
    pub fn new() -> Compiler {
        Compiler { builder: BytecodeBuilder::new(), rulemap: HashMap::new() }
    }

    pub fn emit(&self) -> Bytecode {
        self.builder.build()
    }

    pub fn emit_owned(self) -> OwnedBytecode {
        self.builder.build_owned()
    }

    pub fn compile_grammar(&mut self, grammar: &[(&str, Expression)]) -> Result<(), CompileError> {
        if grammar.len() == 0 {
            return Err(CompileError::EmptyGrammar)
        }
        for (i, (name, expr)) in grammar.iter().enumerate() {
            let current_address = self.builder.current_address();
            let mut rule_info = self.rule_info(name);
            rule_info.index = Some(i as i32);
            rule_info.address = current_address;
            self.compile_expr(expr);
            self.builder.push_instruction(&Instruction::Return);
        }
        for (name, _expr) in grammar.iter() {
            let rule_info = &self.rulemap[*name];
            let rule_address = rule_info.address;
            let iter = rule_info.call_addresses.iter();
            for call_addr in iter {
                self.builder.patch_jump(*call_addr, rule_address);
            }
        }
        Ok(())
    }

    fn rule_info(&mut self, name: &str) -> &mut RuleCompileInfo {
        self.rulemap.entry(name.to_string()).or_insert_with(RuleCompileInfo::new)
    }

    pub fn compile_expr(&mut self, expr: &Expression) {
        match expr {
            Expression::Char(c) => {
                let b = *c as u8;
                self.builder.push_instruction(&Instruction::Byte(b));
            },
            Expression::Literal(s) => {
                self.builder.push_instruction(&Instruction::Literal(s));
            },
            Expression::Class(c) => {
                self.builder.push_instruction(&Instruction::Class(*c));
            },
            Expression::Set(s) => {
                self.builder.push_instruction(&Instruction::Set(s));
            },
            Expression::InverseSet(s) => {
                self.builder.push_instruction(&Instruction::NotSet(s));
            },
            Expression::Range(min, max) => {
                let b_min = *min as u8;
                let b_max = *max as u8;
                self.builder.push_instruction(&Instruction::Range(b_min, b_max));
            },
            Expression::Any => {
                self.builder.push_instruction(&Instruction::Any);
            },
            Expression::NonTerminal(s) => {
                let addr = self.builder.current_address();
                self.builder.push_instruction(&Instruction::Call(Address::default()));
                self.rule_info(s).call_addresses.push(addr);
            },
            Expression::Quantifier(e, n) => {
                match n {
                    -1 => {
                        self.compile_expr(e);
                        self.builder.push_instruction(&Instruction::Succeed);
                    },
                    0 => {
                        let addr = self.builder.current_address();
                        self.compile_expr(e);
                        self.builder.push_instruction(&Instruction::JumpIfSuccess(addr));
                        self.builder.push_instruction(&Instruction::Succeed);
                    },
                    1 => {
                        self.builder.push_instruction(&Instruction::QuantifierInit);
                        self.compile_expr(e);
                        self.builder.push_instruction(&Instruction::QuantifierNext);
                        self.builder.push_instruction(&Instruction::FailIfLessThan(1));
                        self.builder.push_instruction(&Instruction::Pop);
                    },
                    _ => ()
                }
            },
            Expression::And(e) => {
                self.builder.push_instruction(&Instruction::Push);
                self.compile_expr(e);
                self.builder.push_instruction(&Instruction::Peek);
                self.builder.push_instruction(&Instruction::Pop);
            },
            Expression::Not(e) => {
                self.builder.push_instruction(&Instruction::Push);
                self.compile_expr(e);
                self.builder.push_instruction(&Instruction::ToggleSuccess);
                self.builder.push_instruction(&Instruction::Peek);
                self.builder.push_instruction(&Instruction::Pop);
            },
            Expression::Sequence(es) => {
                let n = es.len();
                match n {
                    0 => (),
                    1 => self.compile_expr(&es[0]),
                    _ => {
                        self.builder.push_instruction(&Instruction::Push);
                        self.compile_expr(&es[0]);
                        let mut jump_fail_patches = Vec::with_capacity(n - 1);
                        for e in es[1..].iter() {
                            let addr = self.builder.current_address();
                            self.builder.push_instruction(&Instruction::JumpIfFail(Address::default()));
                            jump_fail_patches.push(addr);
                            self.compile_expr(e);
                        }
                        let jump_success_patch = self.builder.current_address();
                        self.builder.push_instruction(&Instruction::JumpIfSuccess(Address::default()));
                        let fail_address = self.builder.current_address();
                        for patch_addr in jump_fail_patches.iter() {
                            self.builder.patch_jump(*patch_addr, fail_address);
                        }
                        self.builder.push_instruction(&Instruction::Peek);
                        let end_address = self.builder.current_address();
                        self.builder.patch_jump(jump_success_patch, end_address);
                        self.builder.push_instruction(&Instruction::Pop);
                    }
                }
            },
            Expression::Choice(es) => {
                let n = es.len();
                match n {
                    0 => (),
                    1 => self.compile_expr(&es[0]),
                    _ => {
                        let mut jump_success_patches = Vec::with_capacity(n - 1);
                        self.compile_expr(&es[0]);
                        for e in es[1..].iter() {
                            let addr = self.builder.current_address();
                            self.builder.push_instruction(&Instruction::JumpIfSuccess(Address::default()));
                            jump_success_patches.push(addr);
                            self.compile_expr(e);
                        }
                        let end_address = self.builder.current_address();
                        for patch_addr in jump_success_patches.iter() {
                            self.builder.patch_jump(*patch_addr, end_address);
                        }
                    }
                }
            },
        }
    }
}

