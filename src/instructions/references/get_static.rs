use crate::instructions::base::instruction::{ConstantPoolInstruction, Instruction};
use crate::runtime_data_area::frame::Frame;
use crate::instructions::base::bytecode_reader::BytecodeReader;
use crate::runtime_data_area::heap::constant_pool::Constant::FieldReference;

pub struct GetStatic(ConstantPoolInstruction);

impl GetStatic {
    #[inline]
    pub const fn new() -> GetStatic {
        return GetStatic(ConstantPoolInstruction::new());
    }
}

impl Instruction for GetStatic {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        self.0.fetch_operands(reader);
    }

    fn execute(&mut self, frame: &mut Frame) {
        let cp = (*frame.method().class()).borrow().constant_pool();
        let constant = cp.get_constant(self.0.index());
        let field_ref = match constant {
            FieldReference(c) => c,
            _ => {}
        };
        let field_option = field_ref.resolved_field();
        let field = (*field_option.unwrap()).borrow();
        let class = field.parent().class();
        if !field.parent().is_static() {
            panic!("java.lang.IncompatibleClassChangeError");
        }
        let desc = field.parent().descriptor();
        let slot_id = field.slot_id();
        let slots = (*class).borrow_mut().mut_static_vars().expect("slots is none");
        let stack = frame.operand_stack().expect("stack is none");
        let first_char = desc.chars().next().unwrap();
        match first_char {
            'Z'|'B'|'C'|'S'|'I' => stack.push_int(slots.get_int(slot_id)),
            'F' => stack.push_float(slots.get_float(slot_id)),
            'J' => stack.push_long(slots.get_long(slot_id)),
            'D' => stack.push_double(slots.get_double(slot_id)),
            'L' | '[' => stack.push_ref(slots.get_ref(slot_id)),
            _ => {}
        }
    }
}