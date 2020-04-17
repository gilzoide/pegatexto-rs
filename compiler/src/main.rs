use pegatexto_compiler::Compiler;

use pegatexto_disassembler::dump_bytecode;
use pegatexto_vm::grammar::character_class::CharacterClass;
use pegatexto_vm::grammar::expression::Expression;
use pegatexto_vm::matcher::*;

fn test_grammar() -> [(&'static str, Expression); 7] {
    use Expression::*;

	// 
        // Sp <- \s*
        // Number <- '-'? \d+ Sp
        // TermOp <- [+-] Sp
        // FactorOp <- [*/] Sp
        // 
        // Exp <- Term (TermOp Term)*
        // Term <- Factor (FactorOp Factor)*
        // Factor <- Number / '(' Sp Exp ')' Sp
        //
    [
        ("Exp", NonTerminal("Term".to_string()) + ((NonTerminal("TermOp".to_string()) + NonTerminal("Term".to_string()))^0)),
        ("Term", NonTerminal("Factor".to_string()) + ((NonTerminal("FactorOp".to_string()) + NonTerminal("Factor".to_string()))^0)),
        ("Factor", NonTerminal("Number".to_string()) / (Char('(') + NonTerminal("Sp".to_string()) + NonTerminal("Exp".to_string()) + Char(')') + NonTerminal("Sp".to_string()))),

        ("Sp", Class(CharacterClass::Whitespace)^0),
        ("Number", (Char('-')^(-1)) + (Class(CharacterClass::Digit)^1) + NonTerminal("Sp".to_string())),
        ("TermOp", Set("+-".to_string()) + NonTerminal("Sp".to_string())),
        ("FactorOp", Set("*/".to_string()) + NonTerminal("Sp".to_string())),
    ]
}

fn main() {
    let mut compiler = Compiler::new();

    compiler.compile_grammar(&test_grammar()).unwrap();
    let bytecode = compiler.emit();
    dump_bytecode(&bytecode);

    let result = try_match_then(&bytecode, "3 + 5*9 / (1+1) - 12", |s, i, args| {
        println!("!! ACTION {:?} {} {}", s, i, args.len());
        args.iter().max().copied().unwrap_or(s.len())
    });
    println!("{:?}", result);
}
