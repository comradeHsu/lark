use crate::class_file::constant_pool::ConstantClassInfo;
use crate::runtime_data_area::heap::class::Class;
use crate::runtime_data_area::heap::constant_pool::ConstantPool;
use crate::runtime_data_area::heap::sym_ref::SymbolRef;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, Clone)]
pub struct ClassRef {
    symbol_ref: SymbolRef,
}

impl ClassRef {
    pub fn new_class_ref(info: &ConstantClassInfo) -> ClassRef {
        return ClassRef {
            symbol_ref: SymbolRef::with_info(info),
        };
    }

    #[inline]
    pub fn resolved_class(&mut self) -> Rc<RefCell<Class>> {
        return self.symbol_ref.resolved_class();
    }

    #[inline]
    pub fn set_holder(&mut self, holder: Rc<RefCell<Class>>) {
        self.symbol_ref.holder = Some(holder);
    }
}
