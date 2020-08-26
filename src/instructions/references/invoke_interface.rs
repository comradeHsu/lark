use crate::instructions::base::bytecode_reader::BytecodeReader;
use crate::instructions::base::instruction::Instruction;
use crate::instructions::base::method_invoke_logic::invoke_method;
use crate::instructions::references::{ResolveInterfaceMethodRef};

use crate::oops::method_ref::MethodRef;
use crate::runtime::frame::Frame;
use std::ops::Deref;

pub struct InvokeInterface {
    index: usize,
}

impl InvokeInterface {
    #[inline]
    pub const fn new() -> InvokeInterface {
        return InvokeInterface { index: 0 };
    }
}

impl Instruction for InvokeInterface {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        self.index = reader.read_u16() as usize;
        reader.read_u8();
        reader.read_u8();
    }

    fn execute(&mut self, frame: &Frame) {
        let current_class = frame.method().class();
        let (interface, resolved_method) = self.resolved_interface_method_ref_tuple(&current_class);
        if resolved_method.is_static() || resolved_method.is_private() {
            panic!("java.lang.IncompatibleClassChangeError")
        }

        let object = frame.get_ref_from_top(resolved_method.arg_slot_count() - 1);

        if object.is_none() {
            panic!("java.lang.NullPointerException") // todo
        }
        let object_class = object.unwrap().class();

        if !object_class
            .is_implements(&interface)
        {
            panic!("java.lang.IncompatibleClassChangeError")
        }
        let method_to_be_invoked = MethodRef::look_up_method_in_class(
            &object_class,
            resolved_method.name(),
            resolved_method.descriptor(),
        );
        if method_to_be_invoked.is_none() || method_to_be_invoked.as_ref().unwrap().is_abstract() {
            panic!("java.lang.AbstractMethodError")
        }
        if !method_to_be_invoked.as_ref().unwrap().is_public() {
            panic!("java.lang.IllegalAccessError")
        }

        invoke_method(frame, method_to_be_invoked.unwrap());
    }
}

impl ResolveInterfaceMethodRef for InvokeInterface {
    fn get_index(&self) -> usize {
        return self.index;
    }
}
