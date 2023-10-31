use crate::chunk::Chunk;
use crate::chunk::Value;

pub(crate) fn disasassemble_chunk(chunk: Chunk, name: &str) {
    println!("== {} ==", name);
    let mut offset = 0;
    while offset < chunk.code.len() {
        offset = disassemble_instruction(&chunk, offset);
    }
}

fn disassemble_instruction(chunk: &Chunk, offset: usize) -> usize {
    print!("{:04} ", offset);
    if offset > 0 && chunk.lines[offset] == chunk.lines[offset - 1] {
        print!("   | ");
    } else {
        print!("{:04} ", chunk.lines[offset])
    }
    let instruction = chunk.code[offset];
    
    match instruction {
        0 => { // OpConstant
            return constant_instruction("OP_CONSTANT", chunk, offset);
        }
        1 => { // OpReturn
            return simple_instruction("OP_RETURN", offset);
        }
        _ => {
            println!("unknown opcode {}", instruction);
            return offset + 1;
        }
    }
}

fn simple_instruction(name: &str, offset: usize) -> usize {
    println!("{}", name);
    return offset + 1;
}

fn constant_instruction(name: &str, chunk: &Chunk, offset: usize) -> usize {
    let constant = chunk.code[offset + 1];
    print!("{} {} '", name, constant);
    print_value(chunk.constants[constant as usize]);
    print!("'\n");
    return offset + 2;
}

fn print_value(value: Value) {
    print!("{}", value)
}