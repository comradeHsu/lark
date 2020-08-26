use crate::instructions::base::class_init_logic::init_class;

use crate::invoke_support::parameter::{Parameter, Parameters};
use crate::invoke_support::{JavaCall, ReturnType};
use crate::native::registry::Registry;
use crate::oops::class::Class;
use crate::oops::method::Method;
use crate::oops::object::Object;
use crate::runtime::frame::Frame;
use crate::runtime::operand_stack::OperandStack;


use std::rc::Rc;

pub fn init() {
    Registry::register(
        "sun/reflect/NativeConstructorAccessorImpl",
        "newInstance0",
        "(Ljava/lang/reflect/Constructor;[Ljava/lang/Object;)Ljava/lang/Object;",
        new_instance0,
    );
}

// private static native Object newInstance0(Constructor<?> c, Object[] os)
// throws InstantiationException, IllegalArgumentException, InvocationTargetException;
// (Ljava/lang/reflect/Constructor;[Ljava/lang/Object;)Ljava/lang/Object;
pub fn new_instance0(frame: &Frame) {
    let (constructor_obj, arg_arr_obj) = frame.local_vars_get(|vars| {
        let constructor_obj = vars.get_ref(0).unwrap();
        let arg_arr_obj = vars.get_ref(1);
        (constructor_obj, arg_arr_obj)
    });

    let constructor = get_constructor(constructor_obj);
    let class = constructor.class();

    if !class.initialized() {
        frame.revert_next_pc();
        init_class(class);
        return;
    }
    let obj = Some(Class::new_object(&class));
    frame.push_ref(obj.clone());

    //    // call <init>
    //    let ops = convert_args(obj.unwrap(), arg_arr_obj, constructor.clone());
    //    let shim_frame = Frame::new_shim_frame(
    //        frame.thread(),
    //        ops.unwrap_or_else(|| OperandStack::new(0).unwrap()),
    //    );
    //    let thread = frame.thread();
    //    (*thread).borrow_mut().push_frame(shim_frame);
    //
    //    hack_invoke_method(thread, constructor);

    let mut params = Parameters::with_parameters(vec![Parameter::Object(obj.clone())]);
    if arg_arr_obj.is_some() {
        let arg_array = arg_arr_obj.unwrap();
        arg_array.references(|args| {
            args.iter()
                .for_each(|arg| params.append_parameter(Parameter::Object(arg.clone())))
        })
    }
    JavaCall::invoke(constructor, Some(params), ReturnType::Void);
}

fn get_method(method_obj: Object) -> Method {
    return _get_method(method_obj, false);
}

fn get_constructor(constructor_obj: Object) -> Method {
    return _get_method(constructor_obj, true);
}

fn _get_method(method_obj: Object, is_constructor: bool) -> Method {
    let extra = method_obj.meta_data();
    if extra.not_null() {
        return extra.method();
    }

    if is_constructor {
        let root = method_obj
            .get_ref_var("root", "Ljava/lang/reflect/Constructor;")
            .expect("the object hasn't root attribute");
        return root.meta_data().method();
    } else {
        let root = method_obj
            .get_ref_var("root", "Ljava/lang/reflect/Method;")
            .expect("the object hasn't root attribute");
        return root.meta_data().method();
    }
}

// Object[] -> []interface{}
fn convert_args(this: Object, _arg_arr: Object, method: Rc<Method>) -> Option<OperandStack> {
    if method.arg_slot_count() == 0 {
        return None;
    }

    //	argObjs := arg_arr.Refs()
    //	argTypes := method.ParsedDescriptor().ParameterTypes()

    let mut ops = OperandStack::new(method.arg_slot_count()).unwrap();
    if !method.is_static() {
        ops.push_ref(Some(this));
    }
    if method.arg_slot_count() == 1 && !method.is_static() {
        return Some(ops);
    }

    //	for i, argType := range argTypes {
    //		argObj := argObjs[i]
    //
    //		if len(argType) == 1 {
    //			// base type
    //			// todo
    //			unboxed := box.Unbox(argObj, argType)
    //			args[i+j] = unboxed
    //			if argType.isLongOrDouble() {
    //				j++
    //			}
    //		} else {
    //			args[i+j] = argObj
    //		}
    //	}

    return Some(ops);
}
