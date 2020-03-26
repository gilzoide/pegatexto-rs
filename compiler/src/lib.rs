use pegatexto_vm::bytecode::Bytecode;
use pegatexto_vm::bytecode::address::Address;
use pegatexto_vm::bytecode::builder::Builder;
use pegatexto_vm::bytecode::instruction::Instruction;
use pegatexto_vm::grammar::Grammar;
use pegatexto_vm::grammar::expression::Expression;

pub struct Compiler {
    builder: Builder,
}

impl Compiler {
    pub fn new() -> Compiler {
        Compiler { builder: Builder::new() }
    }

    pub fn emit(self) -> Bytecode {
        self.builder.build()
    }

    pub fn compile_grammar(&mut self, grammar: &Grammar) {
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
            Expression::Range(min, max) => {
                let b_min = *min as u8;
                let b_max = *max as u8;
                self.builder.push_instruction(&Instruction::Range(b_min, b_max));
            },
            Expression::Any => {
                self.builder.push_instruction(&Instruction::NotByte(b'\0'));
            },
            Expression::NonTerminal(s) => {
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
                            let addr = self.builder.push_instruction(&Instruction::JumpIfFail(Address::zero()));
                            jump_fail_patches.push(addr);
                            self.compile_expr(e);
                        }
                        let jump_success_patch = self.builder.push_instruction(&Instruction::JumpIfSuccess(Address::zero()));
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
                            let addr = self.builder.push_instruction(&Instruction::JumpIfSuccess(Address::zero()));
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

