use pegatexto_compiler::Compiler;

use pegatexto_disassembler::dump_bytecode;
use pegatexto_vm::grammar::character_class::CharacterClass;
use pegatexto_vm::grammar::expression::Expression;
use pegatexto_vm::matcher::try_match;

fn test_grammar() -> [(&'static str, Expression); 2] {
    use Expression::*;

    [
        ("axiom", Literal("hello".to_string()) + NonTerminal("s".to_string()) + Literal("world".to_string())),
        ("s", Class(CharacterClass::Whitespace)^1),
    ]
}

fn main() {
    let mut compiler = Compiler::new();

    compiler.compile_grammar(&test_grammar()).unwrap();
    let bytecode = compiler.emit();
    dump_bytecode(&bytecode);

    let result = try_match(&bytecode, "hello     world");
    println!("{:?}", result);
}
