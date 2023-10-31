use crate::chunk::{Chunk, Value};
use crate::debug::print_value;

#[repr(u8)]
enum BinaryOp {
    Add, Subtract, Multiply, Divide
}

pub(crate) enum InterpretResult {
    InterpretOk,
    InterpretCompileError,
    InterpretRuntimeError,
}

pub(crate) struct VM {
    chunk: Chunk,
    ip: usize,
    stack: Vec<Value>,
    stack_top: usize,
}

impl VM {
    pub(crate) fn new() -> Self {
        VM {
            chunk: Chunk::new(),
            ip: 0,
            stack: vec![],
            stack_top: 0,
        }
    }    

    fn reset_stack(&mut self) {
        self.stack_top = self.stack.len();
    }

    pub(crate) fn interpret(&mut self, chunk: Chunk) -> InterpretResult {
        self.chunk = chunk;
        self.ip = self.chunk.code.len();
        return self.run()
    }

    fn readByte(&mut self) -> usize {
        let ip = self.ip;
        self.ip += 1;
        return ip;
    }

    fn readConstant(&mut self) -> Value {
        let byte = self.readByte();
        return self.chunk.constants[byte];
    }

    fn binaryOp(&mut self, op: BinaryOp) -> Value {
        let b = self.stack.pop().unwrap();
        let a = self.stack.pop().unwrap();
        match op {
            BinaryOp::Add => a + b,
            BinaryOp::Subtract => a - b,
            BinaryOp::Multiply => a * b,
            BinaryOp::Divide => a / b,
        }
    } 

    fn run(&mut self) -> InterpretResult {
        
        loop {
            let instruction = self.readByte();
            match instruction {
                0 => { // OpConstant
                    let constant = self.readConstant();
                    self.stack.push(constant);
                    continue;
                },
                1 => { // OpReturn
                    let value = self.stack.pop().unwrap();
                    print_value(value);
                    println!();
                    return InterpretResult::InterpretOk;
                },
                _ => return InterpretResult::InterpretRuntimeError,
            }
        }
    }
}