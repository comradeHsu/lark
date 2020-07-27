use crate::instructions::base::bytecode_reader::BytecodeReader;
use crate::instructions::base::instruction::{ConstantPoolInstruction, Instruction};
use crate::runtime_data_area::frame::Frame;
use crate::runtime_data_area::heap::class::Class;
use crate::runtime_data_area::heap::constant_pool::Constant::ClassReference;
use crate::utils::boxed;
use crate::instructions::references::ResolveClassRef;

pub struct ANewArray(ConstantPoolInstruction);

impl ANewArray {
    #[inline]
    pub fn new() -> ANewArray {
        return ANewArray(ConstantPoolInstruction::new());
    }
}

impl Instruction for ANewArray {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        self.0.fetch_operands(reader);
    }

    fn execute(&mut self, frame: &mut Frame) {
        let class = frame.method().class();
        let component_class = self.resolve_class_ref(class);
        let stack = frame.operand_stack().expect("stack is none");
        let count = stack.pop_int();
        if count < 0 {
            panic!("java.lang.NegativeArraySizeException")
        }
        let array_class = (*component_class).borrow().array_class();
        let array = Class::new_array(&array_class, count as usize);
        stack.push_ref(Some(boxed(array)));
    }
}

impl ResolveClassRef for ANewArray {
    fn get_index_in_constant_pool(&self) -> usize {
        return self.0.index();
    }
}
