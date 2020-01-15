use std::rc::Rc;
use crate::runtime_data_area::heap::class::Class;
use crate::class_file::member_info::MemberInfo;
use crate::runtime_data_area::heap::access_flags::{PUBLIC, FINAL, PRIVATE, PROTECTED, STATIC, SYNTHETIC};

pub struct ClassMember {
    access_flags:u16,
    name:String,
    descriptor:String,
    class:Rc<Class>
}

impl ClassMember {

    #[inline]
    pub fn new() -> ClassMember {
        return ClassMember{
            access_flags: 0,
            name: "".to_string(),
            descriptor: "".to_string(),
            class: Rc::new(Class::new())
        };
    }

    pub fn copy_member_info(&mut self,info:&MemberInfo) {
        self.access_flags = info.access_flags();
        self.name = info.name().to_string();
        self.descriptor = info.descriptor().to_string();
    }

    #[inline]
    pub fn set_class(&mut self,class:Rc<Class>) {
        self.class = class;
    }

    #[inline]
    pub fn descriptor(&self) -> &str{
        return self.descriptor.as_str();
    }

    #[inline]
    pub fn name(&self) -> &str{
        return self.name.as_str();
    }

    #[inline]
    pub fn is_public(&self) -> bool {
        return 0 != self.access_flags & PUBLIC;
    }

    #[inline]
    pub fn is_private(&self) -> bool {
        return 0 != self.access_flags & PRIVATE;
    }

    #[inline]
    pub fn is_protected(&self) -> bool {
        return 0 != self.access_flags & PROTECTED;
    }

    #[inline]
    pub fn is_static(&self) -> bool {
        return 0 != self.access_flags & STATIC;
    }

    #[inline]
    pub fn is_final(&self) -> bool {
        return 0 != self.access_flags & FINAL;
    }

    #[inline]
    pub fn is_synthetic(&self) -> bool {
        return 0 != self.access_flags & SYNTHETIC;
    }

    pub fn is_accessible_to(&self, class:&Class) -> bool {
        if self.is_public() {
            return true;
        }
        let other = self.class.clone();
        if self.is_protected() {
            return class == other.as_ref() || class.is_sub_class_of(other.as_ref()) ||
                other.package_name() == class.package_name();
        }
        if !self.is_private() {
            return other.package_name() == class.package_name();
        }
        return class == other;
    }
}