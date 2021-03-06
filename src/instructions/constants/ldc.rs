use crate::instructions::base::bytecode_reader::BytecodeReader;
use crate::instructions::base::instruction::{
    ConstantPoolInstruction, Instruction, LocalVarsInstruction,
};
use crate::runtime::frame::Frame;
use crate::oops::constant_pool::Constant::{
    ClassReference, Double, Float, Integer, Long, Str,
};
use crate::oops::string_pool::StringPool;
use crate::oops::constant_pool::Constant;

pub struct LDC(LocalVarsInstruction);

impl LDC {
    #[inline]
    pub fn new() -> LDC {
        return LDC(LocalVarsInstruction::new());
    }
}

impl Instruction for LDC {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        self.0.fetch_operands(reader);
    }

    fn execute(&mut self, frame: &mut Frame) {
        ldc(frame, self.0.get_index());
    }
}

pub struct LDCw(ConstantPoolInstruction);

impl LDCw {
    #[inline]
    pub fn new() -> LDCw {
        return LDCw(ConstantPoolInstruction::new());
    }
}

impl Instruction for LDCw {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        self.0.fetch_operands(reader);
    }

    fn execute(&mut self, frame: &mut Frame) {
        ldc(frame, self.0.index());
    }
}

pub struct LDC2w(ConstantPoolInstruction);

impl LDC2w {
    #[inline]
    pub fn new() -> LDC2w {
        return LDC2w(ConstantPoolInstruction::new());
    }
}

impl Instruction for LDC2w {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        self.0.fetch_operands(reader);
    }

    fn execute(&mut self, frame: &mut Frame) {
        //        let stack = frame.operand_stack().expect("stack is none");
        let class = frame.method().class();
        let borrow_class = (*class).borrow();
        let cp = borrow_class.constant_pool();
        let constant = cp.get_constant_immutable(self.0.index());
        match constant {
            Long(v) => frame.operand_stack().expect("stack is none").push_long(*v),
            Double(v) => frame
                .operand_stack()
                .expect("stack is none")
                .push_double(*v),
            _ => panic!("java.lang.ClassFormatError"),
        }
    }
}

fn ldc(frame: &mut Frame, index: usize) {
    //    let stack = frame.operand_stack().expect("stack is none");
    let class = frame.method().class();
    let mut constant = (*class).borrow_mut().mut_constant_pool().take_constant(index);
    match &mut constant {
        Integer(v) => frame.operand_stack().expect("stack is none").push_int(*v),
        Float(v) => frame.operand_stack().expect("stack is none").push_float(*v),
        Str(v) => {
            let string = StringPool::java_string(v.clone());
            frame
                .operand_stack()
                .expect("stack is none")
                .push_ref(Some(string))
        }
        ClassReference(v) => {
            let class = v.resolved_class(class.clone());
            let borrow = (*class).borrow();
            let obj = borrow.java_class();
            frame
                .operand_stack()
                .expect("stack is none")
                .push_ref(Some(obj.unwrap().clone()));
        }
        _ => panic!("todo: ldc!"),
    }
    (*class)
        .borrow_mut()
        .mut_constant_pool()
        .restoration_constant(index,constant);
}
