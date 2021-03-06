pub mod java_lang_instrument;

use crate::invoke_support::parameter::{Parameter, Parameters};
use crate::invoke_support::{invoke, ReturnType};
use crate::jvm::Jvm;
use crate::oops::class::Class;
use crate::oops::object::Object;
use std::cell::RefCell;
use std::rc::Rc;
use crate::utils::boxed;

const CONSTRUCTOR_DESC: &str = "(JZZ)V";

pub fn create_instrumentation() -> Rc<RefCell<Object>> {
    let boot_loader = Jvm::boot_class_loader();
    let class = boot_loader
        .find_or_create("sun/instrument/InstrumentationImpl")
        .expect("can't find sun.instrument.InstrumentationImpl");
    let constructor = Class::get_constructor(class.clone(), CONSTRUCTOR_DESC);
    let object = Some(boxed(Class::new_object(&class)));
    let parameters = vec![
        Parameter::Object(object.clone()),
        Parameter::Long(0),
        Parameter::Boolean(false),
        Parameter::Boolean(false),
    ];
   invoke(
        constructor.unwrap(),
        Some(Parameters::with_parameters(parameters)),
        ReturnType::Void,
    );
    return object.unwrap();
}
