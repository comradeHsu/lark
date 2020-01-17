use crate::runtime_data_area::heap::class::Class;
use std::rc::Rc;
use crate::class_file::constant_pool::{ConstantPool as Pool, ConstantInfoEnum};
use crate::runtime_data_area::heap::constant_pool::Constant::*;
use std::cell::RefCell;
use crate::runtime_data_area::heap::class_ref::ClassRef;
use core::mem;
use crate::runtime_data_area::heap::field_ref::FieldRef;
use crate::runtime_data_area::heap::method_ref::MethodRef;

#[derive(Debug)]
pub struct ConstantPool {
    class:Option<Rc<RefCell<Class>>>,
    constants:Vec<Constant>
}

impl ConstantPool {

    pub fn none() -> ConstantPool {
        return ConstantPool{
            class: Option::None,
            constants: vec![]
        };
    }

    pub fn new_constant_pool(class:Option<Rc<RefCell<Class>>>,pool:&Pool) -> Rc<RefCell<ConstantPool>> {
        let size = pool.len();
        let mut constants = Vec::with_capacity(size);
        let mut index = 0usize;
        let mut cp = Rc::new(RefCell::new(ConstantPool::none()));
        while index < size {
            let info_enum = pool.get_info(index).unwrap();
            let constant = match info_enum {
                ConstantInfoEnum::Integer(info) => Integer(info.val()),
                ConstantInfoEnum::Float(info) => Float(info.val()),
                ConstantInfoEnum::Long(info) => Long(info.val()),
                ConstantInfoEnum::Double(info) => Double(info.val()),
                ConstantInfoEnum::Str(info) => Str(info.string().to_string()),
                ConstantInfoEnum::Class(info) => {
                    ClassReference(ClassRef::new_class_ref(cp.clone(),info))
                },
                ConstantInfoEnum::FieldRef(info) => {
                    FieldReference(FieldRef::new_field_ref(cp.clone(),info))
                },
                ConstantInfoEnum::MethodRef(info) => {
                    MethodReference(MethodRef::new_method_ref(cp.clone(),info))
                },
                _ => panic!("Unknown constant type")
            };
            match constant {
                Long(_) | Double(_) => {
                    constants.push(constant);
                    constants.push(None);
                    index += 1;
                },
                _ => constants.push(constant)
            }
        }
        let mut pool = Rc::new(RefCell::new(
            ConstantPool{ class, constants }
        ));
        mem::swap(&mut pool,&mut cp);
        return cp;
    }

    pub fn get_constant(&mut self, index:usize) -> &mut Constant {
        let constant = self.constants.get_mut(index);
        if constant.is_none() {
            panic!("No constants at index {}", index);
        }
        return constant.unwrap();
    }

    pub fn get_constant_immutable(&self, index:usize) -> &Constant {
        let constant = self.constants.get(index);
        if constant.is_none() {
            panic!("No constants at index {}", index);
        }
        return constant.unwrap();
    }


    pub fn class(&self) -> Rc<RefCell<Class>> {
        let class = self.class.as_ref().unwrap();
        return class.clone();
    }

    #[inline]
    pub fn set_class(&mut self,class:Rc<RefCell<Class>>) {
        return self.class = Some(class);
    }
}

#[derive(Debug)]
pub enum Constant {
    None,
    Integer(i32),
    Float(f32),
    Long(i64),
    Double(f64),
    Str(String),
    ClassReference(ClassRef),
    FieldReference(FieldRef),
    MethodReference(MethodRef),
    InterfaceMethodRef()
}