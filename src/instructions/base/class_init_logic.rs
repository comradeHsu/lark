use std::rc::Rc;
use std::cell::RefCell;
use crate::runtime_data_area::thread::Thread;
use crate::runtime_data_area::heap::class::Class;

pub fn init_class(thread:Rc<RefCell<Thread>>,class:Rc<RefCell<Class>>) {
    (*class).borrow_mut().set_initialized();
    schedule_clinit(thread.clone(),class.clone());
    init_super_class(thread,class);
}

fn schedule_clinit(thread:Rc<RefCell<Thread>>,class:Rc<RefCell<Class>>) {
    let clinit = (*class).borrow().get_clinit_method();
    if clinit.is_some() {
        let new_frame = Thread::new_frame(thread.clone(),clinit.unwrap());
        (*thread).borrow_mut().push_frame(new_frame);
    }
}

fn init_super_class(thread:Rc<RefCell<Thread>>,class:Rc<RefCell<Class>>) {
    if !(*class).borrow().is_interface() {
        let super_class = (*class).borrow().super_class();
        if super_class.is_none() {
            return
        }
        let super_class = super_class.unwrap();
        if !(*super_class).borrow().initialized() {
            init_class(thread,super_class);
        }
    }
}