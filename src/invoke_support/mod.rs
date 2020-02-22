use crate::instructions::base::bytecode_reader::BytecodeReader;
use crate::instructions::new_instruction;
use crate::interpreter::invoke_java_method;
use crate::invoke_support::parameter::{Parameter, Parameters};
use crate::invoke_support::return_value::ReturnValue;
use crate::jvm::JVM;
use crate::runtime_data_area::frame::Frame;
use crate::runtime_data_area::heap::method::Method;
use crate::runtime_data_area::heap::string_pool::StringPool;
use crate::runtime_data_area::thread::JavaThread;
use crate::utils::boxed;
use std::cell::RefCell;
use std::ops::DerefMut;
use std::rc::Rc;

pub mod parameter;
pub mod return_value;

pub fn invoke(method: Rc<Method>, params: Parameters, return_type: ReturnType) -> ReturnValue {
    let thread = create_execute_env(method, params);
    let return_value = executable(thread, return_type);
    return return_value;
}

fn create_execute_env(method: Rc<Method>, params: Parameters) -> Rc<RefCell<JavaThread>> {
    let thread = boxed(JavaThread::new_thread());
    let mut dummy_frame = JavaThread::new_frame(thread.clone(), method.clone());
    let mut frame = JavaThread::new_frame(thread.clone(), method);
    prepare_parameter(&mut frame, params);
    (*thread).borrow_mut().push_frame(dummy_frame);
    (*thread).borrow_mut().push_frame(frame);
    return thread;
}

fn prepare_parameter(frame: &mut Frame, params: Parameters) {
    let vars = frame.local_vars().expect("LocalVars is none");
    for index in 0..params.size() {
        let parameter = params.get_parameter(index);
        match parameter {
            Parameter::Boolean(value) => vars.set_boolean(index, *value),
            Parameter::Byte(value) => vars.set_int(index, *value as i32),
            Parameter::Short(value) => vars.set_int(index, *value as i32),
            Parameter::Int(value) => vars.set_int(index, *value),
            Parameter::Long(value) => vars.set_long(index, *value),
            Parameter::Float(value) => vars.set_float(index, *value),
            Parameter::Double(value) => vars.set_double(index, *value),
            Parameter::Char(value) => vars.set_int(index, *value as u8 as i32),
            Parameter::Object(value) => vars.set_ref(index, value.clone()),
        }
    }
}

fn executable(mut thread: Rc<RefCell<JavaThread>>, return_type: ReturnType) -> ReturnValue {
    let mut reader = BytecodeReader::new();
    loop {
        let current_frame = (*thread).borrow().current_frame();
        let pc = (*current_frame).borrow().next_pc();
        (*thread).borrow_mut().set_pc(pc);
        let method = (*current_frame).borrow().method_ptr();
        let bytecode = method.code();
        reader.reset(bytecode, pc);
        let opcode = reader.read_u8();
        let mut inst = new_instruction(opcode);
        inst.fetch_operands(&mut reader);
        (*current_frame).borrow_mut().set_next_pc(reader.pc());
        inst.execute((*current_frame).borrow_mut().deref_mut());
        if (*thread).borrow().stack_size() == 1 {
            break;
        }
    }
    let value_frame = (*thread).borrow_mut().pop_frame();
    let mut frame_borrow = (*value_frame).borrow_mut();
    let stack = frame_borrow.operand_stack().expect("stack is none");
    let value = match return_type {
        ReturnType::Void => ReturnValue::Void,
        ReturnType::Boolean => ReturnValue::Boolean(stack.pop_boolean()),
        ReturnType::Byte => ReturnValue::Byte(stack.pop_int() as i8),
        ReturnType::Short => ReturnValue::Short(stack.pop_int() as i16),
        ReturnType::Int => ReturnValue::Int(stack.pop_int()),
        ReturnType::Long => ReturnValue::Long(stack.pop_long()),
        ReturnType::Float => ReturnValue::Float(stack.pop_float()),
        ReturnType::Double => ReturnValue::Double(stack.pop_double()),
        ReturnType::Char => ReturnValue::Char(stack.pop_int() as u8 as char),
        ReturnType::Object => ReturnValue::Object(stack.pop_ref()),
    };
    return value;
}

pub enum ReturnType {
    Void,
    Boolean,
    Byte,
    Short,
    Int,
    Long,
    Float,
    Double,
    Char,
    Object,
}