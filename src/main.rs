use chunk::Chunk;
use chunk::OpCode::*;
use debug::disasassemble_chunk;

mod chunk;
mod debug;

fn main() {
    let mut chunk = Chunk::new();
    let constant = chunk.add_constant(1.2);
    chunk.write(OpConstant as u8, 123);
    chunk.write(constant as u8, 123);
    chunk.write(OpReturn as u8, 123);
    disasassemble_chunk(chunk, "test chunk");
}

