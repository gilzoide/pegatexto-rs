use pegatexto_disassembler::dump_bytecode;

use pegatexto_vm::bytecode::opcode::Opcode;

fn main() {
    // 'a' [0-7] *
    let bytecode = [
        Opcode::Literal as u8, b'a', b'b', b'\n', b'b', b'c', 0,
        Opcode::Range as u8, b'0', b'7',
        Opcode::JumpIfSuccess as u8, 0, 0,
        Opcode::Succeed as u8,
    ];
    dump_bytecode(&bytecode);
}
