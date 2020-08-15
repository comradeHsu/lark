use crate::instructions::base::bytecode_reader::BytecodeReader;
use crate::runtime::frame::Frame;

pub trait Instruction {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader);

    fn execute(&mut self, frame: &Frame);
}

///没有操作数的指令
pub struct NoOperandsInstruction {}

impl NoOperandsInstruction {
    #[inline]
    pub const fn new() -> NoOperandsInstruction {
        return NoOperandsInstruction {};
    }
}

impl Instruction for NoOperandsInstruction {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {}

    fn execute(&mut self, frame: &Frame) {}
}

impl ToString for NoOperandsInstruction {
    fn to_string(&self) -> String {
        unimplemented!()
    }
}

///跳转指令
pub struct BranchInstruction {
    offset: i32,
}

impl BranchInstruction {
    #[inline]
    pub const fn new() -> BranchInstruction {
        return BranchInstruction { offset: 0 };
    }

    #[inline]
    pub fn get_offset(&self) -> i32 {
        return self.offset;
    }
}

impl Instruction for BranchInstruction {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        self.offset = reader.read_i16() as i32;
    }

    fn execute(&mut self, frame: &Frame) {
        unimplemented!()
    }
}

impl ToString for BranchInstruction {
    fn to_string(&self) -> String {
        unimplemented!()
    }
}

///存储和加载指令：本地变量表
pub struct LocalVarsInstruction {
    index: usize,
}

impl LocalVarsInstruction {
    #[inline]
    pub const fn new() -> LocalVarsInstruction {
        return LocalVarsInstruction { index: 0 };
    }

    #[inline]
    pub fn with_index(index: usize) -> LocalVarsInstruction {
        return LocalVarsInstruction { index };
    }

    #[inline]
    pub fn get_index(&self) -> usize {
        return self.index;
    }
}

impl Instruction for LocalVarsInstruction {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        self.index = reader.read_u8() as usize;
    }

    fn execute(&mut self, frame: &Frame) {
        unimplemented!()
    }
}

impl ToString for LocalVarsInstruction {
    fn to_string(&self) -> String {
        unimplemented!()
    }
}

///存储和加载指令：常量池
pub struct ConstantPoolInstruction {
    index: usize,
}

impl ConstantPoolInstruction {
    #[inline]
    pub fn new() -> ConstantPoolInstruction {
        return ConstantPoolInstruction { index: 0 };
    }

    #[inline]
    pub fn index(&self) -> usize {
        return self.index;
    }
}

impl Instruction for ConstantPoolInstruction {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        self.index = reader.read_u16() as usize;
    }

    fn execute(&mut self, frame: &Frame) {
        unimplemented!()
    }
}

impl ToString for ConstantPoolInstruction {
    fn to_string(&self) -> String {
        unimplemented!()
    }
}