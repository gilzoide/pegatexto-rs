use pegatexto_compiler::Compiler;

use pegatexto_disassembler::dump_bytecode;
use pegatexto_vm::grammar::character_class::CharacterClass;
use pegatexto_vm::grammar::expression::Expression;
use pegatexto_vm::matcher::try_match;

fn test_expression() -> Expression<'static> {
    use Expression::*;

    Sequence(vec![
        Literal("hello"),
        Quantifier(Box::new(Class(CharacterClass::Whitespace)), 1),
        Literal("world"),
    ])
}

fn main() {
    let expr = test_expression();
    let mut compiler = Compiler::new();

    compiler.compile_expr(&expr);
    let bytecode = compiler.emit();
    dump_bytecode(&bytecode);

    let result = try_match(&bytecode, "hello\nworld, mfriend");
    println!("{:?}", result);
}
