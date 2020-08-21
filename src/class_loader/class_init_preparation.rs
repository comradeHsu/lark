use crate::oops::class::Class;

use crate::oops::field::Field;
use crate::oops::slots::Slots;

use std::cell::RefCell;
use std::rc::Rc;

pub struct ClassPreparation();

impl ClassPreparation {
    pub fn prepare(class: &Class) {
        Self::calc_instance_field_slot_ids(&class);
        Self::calc_static_field_slot_ids(&class);
        Self::alloc_and_init_static_vars(&class);
    }

    fn calc_instance_field_slot_ids(class: &Class) {
        let mut slot_id = 0usize;
        {
            let super_class = class.super_class();
            if super_class.is_some() {
                slot_id = (*super_class.unwrap()).instance_slot_count() as usize;
            }
        }
        for field in class.fields() {
            let field = field.clone();
            if !(*field).borrow().parent().is_static() {
                (*field).borrow_mut().set_slot(slot_id);
                slot_id += 1;
                if (*field).borrow().is_long_or_double() {
                    slot_id += 1;
                }
            }
        }
       class.set_instance_slot_count(slot_id as u32);
    }

    fn calc_static_field_slot_ids(class: &Class) {
        let mut slot_id = 0usize;
        for field in class.fields() {
            let field = field.clone();
            if (*field).borrow().parent().is_static() {
                (*field).borrow_mut().set_slot(slot_id);
                slot_id += 1;
                if (*field).borrow().is_long_or_double() {
                    slot_id += 1;
                }
            }
        }
        class.set_static_slot_count(slot_id as u32);
    }

    fn alloc_and_init_static_vars(class: &Class) {
        let count = class.static_slot_count() as usize;
        class.set_static_vars(Slots::with_capacity(count));
        let mut static_final_fields = Vec::new();
        for field in class.fields() {
            let is_static = field.borrow_mut().parent().is_static();
            if is_static && field.borrow_mut().parent().is_final() {
                //                ClassLoader::init_static_final_var(class.clone(), field.clone())
                static_final_fields.push(field.clone());
            }
        }
        for field in static_final_fields {
            Self::init_static_final_var(&class, field)
        }
    }

    fn init_static_final_var(class: &Class, field: Rc<RefCell<Field>>) {
        class.init_static_final_variable(field);
    }
}
