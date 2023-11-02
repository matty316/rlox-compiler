use crate::chunk::Chunk;
use crate::chunk::Value;


pub(crate) fn disasassemble_chunk(chunk: &Chunk, name: &str) {
    println!("== {} ==", name);
    let mut offset = 0;
    while offset < chunk.code.len() {
        offset = disassemble_instruction(chunk, offset);
    }
}

pub(crate) fn disassemble_instruction(chunk: &Chunk, offset: usize) -> usize {
    print!("{:04} ", offset);
    if offset > 0 && chunk.lines[offset] == chunk.lines[offset - 1] {
        print!("   | ");
    } else {
        print!("{:04} ", chunk.lines[offset])
    }
    let instruction = &chunk.code[offset];
    
    match instruction {
        0 => constant_instruction("OpConstant", chunk, offset),
        1 => simple_instruction("OpReturn", offset),
        2 => simple_instruction("OpNegate", offset),
        3 => simple_instruction("OpAdd", offset),
        4 => simple_instruction("OpSubtract", offset),
        5 => simple_instruction("OpMultiply", offset),
        6 => simple_instruction("OpDivide", offset),
        _ => {
            println!("unknown opcode");
            offset + 1
        }
    }
}

fn simple_instruction(name: &str, offset: usize) -> usize {
    println!("{}", name);
    offset + 1
}

fn constant_instruction(name: &str, chunk: &Chunk, offset: usize) -> usize {
    let constant = chunk.code[offset + 1];
    print!("{}  {} '", name, constant);
    print_value(&chunk.constants[constant as usize]);
    println!("'");
    offset + 2
}

pub(crate) fn print_value(value: &Value) {
    print!("{}", value)
}