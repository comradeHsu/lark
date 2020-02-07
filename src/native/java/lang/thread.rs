use crate::runtime_data_area::frame::Frame;
use crate::runtime_data_area::heap::class_loader::ClassLoader;
use crate::runtime_data_area::heap::class::Class;
use crate::runtime_data_area::heap::string_pool::StringPool;
use crate::utils::boxed;
use crate::native::registry::Registry;

pub fn init() {
    Registry::register("java/lang/Thread", "currentThread",
                       "()Ljava/lang/Thread;", current_thread);
    Registry::register("java/lang/Thread", "setPriority0",
                       "(I)V", set_priority0);
    Registry::register("java/lang/Thread", "isAlive",
                       "()Z", is_alive);
    Registry::register("java/lang/Thread", "start0",
                       "()V", start0);
}

pub fn current_thread(frame:&mut Frame) {
    let class = frame.method().class();
    let loader = (*class).borrow().loader();
    let thread_class = ClassLoader::load_class(loader.clone(),"java/lang/Thread");
    let mut java_thread = Class::new_object(&thread_class);
    java_thread.set_ref_var("name","Ljava/lang/String;",
                            StringPool::java_string(loader.clone(),"Main".to_string()));

    let thread_group_class = ClassLoader::load_class(loader.clone(),"java/lang/ThreadGroup");
    let mut java_thread_group = Class::new_object(&thread_group_class);
    java_thread.set_ref_var("group","Ljava/lang/ThreadGroup;",
                            boxed(java_thread_group));
    java_thread.set_int_var("priority", "I", 1);

    frame.operand_stack().expect("stack is none").push_ref(Some(boxed(java_thread)));
}

// private native void setPriority0(int newPriority);
// (I)V
pub fn set_priority0(frame:&mut Frame) {
    // vars := frame.LocalVars()
    // this := vars.GetThis()
    // newPriority := vars.GetInt(1))
    // todo
}

// public final native boolean isAlive();
// ()Z
pub fn is_alive(frame:&mut Frame) {
    frame.operand_stack().expect("stack is none").push_boolean(false);
}

// private native void start0();
// ()V
pub fn start0(frame:&mut Frame) {
// todo
}