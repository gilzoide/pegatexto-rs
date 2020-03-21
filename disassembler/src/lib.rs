use pegatexto_vm::bytecode::instruction::InstructionIterator;

fn usize_digits(x: usize) -> usize {
    let mut digits = 0;
    let mut x = x;
    while x > 0 {
        x /= 10;
        digits += 1;
    }
    digits
}

pub fn dump_bytecode(bytecode: &[u8]) {
    let mut iter = InstructionIterator::new(bytecode);
    let address_digits = usize_digits(iter.bytes_len());
    let mut current = 0;
    while let Some(instruction) = iter.next() {
        println!("{:width$} | {}", current, instruction, width = address_digits);
        current = iter.current().into();
    }
}
