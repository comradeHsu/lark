use crate::instructions::base::instruction::{LocalVarsInstruction, NoOperandsInstruction, Instruction};
use crate::runtime_data_area::frame::Frame;
use crate::instructions::base::bytecode_reader::BytecodeReader;

fn i_store(frame: &mut Frame,index:usize) {
    let val = frame.operand_stack().expect("operand_stack is empty").pop_int();
    frame.local_vars().expect("local_vars is empty").set_int(index,val);
}

///istore
pub struct IStore(LocalVarsInstruction);

impl IStore {

    #[inline]
    pub fn with_index(index:usize) -> IStore {
        return IStore(LocalVarsInstruction::with_index(index));
    }
}

impl Instruction for IStore {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        self.0.fetch_operands(reader);
    }

    fn execute(&mut self, frame: &mut Frame) {
        i_store(frame,self.0.get_index());
    }
}

///istore_0
pub struct IStore0(NoOperandsInstruction);

impl Instruction for IStore0 {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        self.0.fetch_operands(reader);
    }

    fn execute(&mut self, frame: &mut Frame) {
        i_store(frame,0);
    }
}

///istore_1
pub struct IStore1(NoOperandsInstruction);

impl Instruction for IStore1 {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        self.0.fetch_operands(reader);
    }

    fn execute(&mut self, frame: &mut Frame) {
        i_store(frame,1);
    }
}

///istore_2
pub struct IStore2(NoOperandsInstruction);

impl Instruction for IStore2 {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        self.0.fetch_operands(reader);
    }

    fn execute(&mut self, frame: &mut Frame) {
        i_store(frame,2);
    }
}

///istore_3
pub struct IStore3(NoOperandsInstruction);

impl Instruction for IStore3 {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        self.0.fetch_operands(reader);
    }

    fn execute(&mut self, frame: &mut Frame) {
        i_store(frame,3);
    }
}