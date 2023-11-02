pub(crate) type Value = f64;

#[derive(Debug)]
pub(crate) enum OpCode {
    OpConstant,
    OpReturn,
    OpNegate,
    OpAdd,
    OpSubtract,
    OpMultiply,
    OpDivide,
}

pub(crate) struct Chunk {
    pub(crate) code: Vec<u8>,
    pub(crate) constants: Vec<Value>,
    pub(crate) lines: Vec<usize>
}

impl Chunk {
    pub(crate) fn new() -> Self {
        Chunk {
            code: vec![],
            constants: vec![],
            lines: vec![],
        }
    }

    pub(crate) fn write(&mut self, code: u8, line: usize) {
        self.code.push(code);
        self.lines.push(line);
    }

    pub(crate) fn add_constant(&mut self, value: Value) -> usize {
        self.constants.push(value);
        self.constants.len() - 1
    }
}


