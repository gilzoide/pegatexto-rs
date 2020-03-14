use pegatexto_vm::bytecode::instruction::InstructionIterator;

pub fn dump_bytecode(bytecode: &[u8]) {
    let mut iter = InstructionIterator::new(bytecode);
    let mut current = 0;
    while let Some(instruction) = iter.next() {
        println!("{} | {}", current, instruction);
        current = iter.current();
    }
}
