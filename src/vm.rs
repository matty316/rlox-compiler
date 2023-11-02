use crate::chunk::{Chunk, Value};
use crate::debug::{print_value, disassemble_instruction};
use crate::chunk::OpCode::*;

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
}

static DEBUG: bool = true;

impl VM {
    pub(crate) fn new() -> Self {
        VM {
            chunk: Chunk::new(),
            ip: 0,
            stack: vec![],
        }
    }    

    pub(crate) fn interpret(&mut self, chunk: Chunk) -> InterpretResult {
        self.chunk = chunk;
        self.ip = 0;
        self.run()
    }

    fn push(&mut self, value: Value) {
        self.stack.push(value);
    }

    fn pop(&mut self) -> Value {
        self.stack.pop().unwrap()
    }

    fn readByte(&mut self) -> u8 {
        let ip = self.ip;
        self.ip += 1;
        self.chunk.code[ip]
    }

    fn readConstant(&mut self) -> Value {
        let byte = self.readByte();
        self.chunk.constants[byte as usize]
    }

    fn binaryOp(&mut self, op: BinaryOp) -> Value {
        let b = self.pop();
        let a = self.pop();
        let result = match op {
            BinaryOp::Add => a + b,
            BinaryOp::Subtract => a - b,
            BinaryOp::Multiply => a * b,
            BinaryOp::Divide => a / b,
        };
        self.push(result);
        result
    } 

    fn run(&mut self) -> InterpretResult {
        
        loop {
            if DEBUG {
                print!("       ");
                for slot in &self.stack {
                    print!("[ ");
                    print_value(slot);
                    print!(" ]");
                }
                println!();
                disassemble_instruction(&self.chunk, self.ip);
            }
            let instruction = self.readByte();
            match instruction {
                0 => { // OpConstant
                    let constant = self.readConstant();
                    self.push(constant);
                    continue;
                }
                1 => { // OpReturn
                    let value = self.pop();
                    print_value(&value);
                    println!();
                    return InterpretResult::InterpretOk;
                }
                2 => {
                    let pop = self.pop();
                    self.push(-pop);
                    continue;
                }
                3 => {
                    self.binaryOp(BinaryOp::Add);
                    continue;
                }
                4 => {
                    self.binaryOp(BinaryOp::Subtract);
                    continue;
                }
                5 => {
                    self.binaryOp(BinaryOp::Multiply);
                    continue;
                }
                6 => {
                    self.binaryOp(BinaryOp::Divide);
                    continue;
                }
                _ => continue,
            }
        }
    }
}