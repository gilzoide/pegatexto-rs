use pegatexto_compiler::Compiler;

use pegatexto_disassembler::dump_bytecode;
use pegatexto_vm::grammar::character_class::CharacterClass;
use pegatexto_vm::grammar::expression::Expression;
use pegatexto_vm::matcher::*;

fn test_grammar() -> [(&'static str, Expression); 4] {
    use Expression::*;

	/* CSV <- Line*
	 * Line <- Field ("," Field)* (EOL / !.)
	 * Field <- [^\n,]+
	 * EOL <- \r? \n
	 */
    [
        ("CSV", NonTerminal("Line".to_string())^0),
        ("Line", (NonTerminal("Field".to_string()) + ((Char(',') + NonTerminal("Field".to_string()))^0) + (NonTerminal("EOL".to_string()) / !Any)) >> "line"),
        ("Field", (InverseSet("\n,".to_string())^1) >> "field"),
        ("EOL", (Char('\r')^(-1)) + Char('\n')),
    ]
}

fn main() {
    let mut compiler = Compiler::new();

    compiler.compile_grammar(&test_grammar()).unwrap();
    let bytecode = compiler.emit();
    dump_bytecode(&bytecode);

    let result = try_match_then(&bytecode, "oi,cábra\nda,peste", |s, i, args| {
        println!("!! ACTION {:?} {} {}", s, i, args.len());
        args.iter().max().copied().unwrap_or(s.len())
    });
    println!("{:?}", result);
}
