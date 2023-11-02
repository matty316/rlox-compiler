use crate::chunk::{Chunk, Value};
use crate::debug::{print_value, disassemble_instruction};


enum BinaryOp {
    Add, Subtract, Multiply, Divide
}

pub(crate) enum InterpretResult {
    Ok,
    CompileError,
    RuntimeError,
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

    fn read_byte(&mut self) -> u8 {
        let ip = self.ip;
        self.ip += 1;
        self.chunk.code[ip]
    }

    fn read_constant(&mut self) -> Value {
        let byte = self.read_byte();
        self.chunk.constants[byte as usize]
    }

    fn binary_op(&mut self, op: BinaryOp) -> Value {
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
            let instruction = self.read_byte();
            match instruction {
                0 => { // OpConstant
                    let constant = self.read_constant();
                    self.push(constant);
                    continue;
                }
                1 => { // OpReturn
                    let value = self.pop();
                    print_value(&value);
                    println!();
                    return InterpretResult::Ok;
                }
                2 => {
                    let pop = self.pop();
                    self.push(-pop);
                    continue;
                }
                3 => {
                    self.binary_op(BinaryOp::Add);
                    continue;
                }
                4 => {
                    self.binary_op(BinaryOp::Subtract);
                    continue;
                }
                5 => {
                    self.binary_op(BinaryOp::Multiply);
                    continue;
                }
                6 => {
                    self.binary_op(BinaryOp::Divide);
                    continue;
                }
                _ => continue,
            }
        }
    }
}