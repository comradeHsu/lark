use crate::class_loader::app_class_loader::ClassLoader;
use crate::class_loader::bootstrap_class_loader::BootstrapClassLoader;
use crate::class_path::class_path::ClassPath;
use crate::cmd::Cmd;
use crate::instructions::base::class_init_logic::init_class;
use crate::interpreter::interpret;
use crate::invoke_support::parameter::{Parameter, Parameters};
use crate::invoke_support::{JavaCall, ReturnType};
use crate::oops::class::Class;
use crate::oops::object::Object;
use crate::oops::string_pool::StringPool;

use crate::runtime::frame::Frame;
use crate::runtime::thread::{JavaThread, thread_priority};
use crate::utils::{java_str_to_rust_str};
use chrono::Local;
use std::borrow::Borrow;
use std::cell::RefCell;
use std::rc::Rc;
use crate::invoke_support::ReturnType::Void;
use crate::oops::object::MetaData::Thread;
use crate::native;
use crate::universe::Universe;

pub struct Jvm {
    cmd: Cmd,
    boot_class_loader: BootstrapClassLoader,
    ext_class_loader: Option<Object>,
    app_class_loader: Option<Object>,
    main_thread: JavaThread,
}

pub static mut JVM: Option<Jvm> = None;

impl Jvm {
    pub fn new(mut cmd: Cmd) -> &'static mut Jvm {
        let mut cp = ClassPath::parse(&cmd.x_jre_option, &cmd.cp_option);
        if cmd.exec_jar_path().is_some() {
            cp.handle_jar(&mut cmd);
        }
        let class_path = Rc::new(cp);
        let class_loader = BootstrapClassLoader::new(class_path);
        let jvm = Jvm {
            cmd,
            boot_class_loader: class_loader,
            ext_class_loader: None,
            main_thread: JavaThread::new_main_thread(),
            app_class_loader: None,
        };
        jvm.main_thread.set();
        unsafe {
            JVM = Some(jvm);
            return JVM.as_mut().unwrap();
        }
    }

    fn main_thread_init(&self) {
        let thread_group = self.create_initial_thread_group();
        /// create main thread object
        let thread_class = self.boot_class_loader
            .find_or_create("java/lang/Thread")
            .unwrap();
        let thread_obj = Class::new_object(&thread_class);
        thread_obj.set_int_var("priority", "I", thread_priority::NORM_PRIORITY);
        let thread_constructor =
            thread_class.get_constructor("(Ljava/lang/ThreadGroup;Ljava/lang/String;)V");
        let parameters = Parameters::with_parameters(vec![
            Parameter::Object(Some(thread_obj.clone())),
            Parameter::Object(Some(thread_group)),
            Parameter::Object(Some(StringPool::java_string("main".to_string())))
        ]);
        thread_obj.set_meta_data(Thread(self.main_thread.clone()));
        self.main_thread.set_java_thread(Some(thread_obj));
        JavaCall::invoke(thread_constructor.unwrap(),Some(parameters),Void);
    }

    #[inline]
    pub fn main_thread(&self) -> JavaThread {
        return self.main_thread.clone();
    }

    #[inline]
    pub fn boot_class_loader() -> &'static BootstrapClassLoader {
        return &Self::instance().unwrap().boot_class_loader;
    }

    #[inline]
    pub fn instance() -> Option<&'static Self> {
        unsafe {
            return JVM.as_ref();
        }
    }

    pub fn start(&mut self) {
        //        let builder = (*self.main_thread).borrow_mut().std_thread();
        //        let join_handler = builder.spawn(move || {
        native::init();
        self.boot_class_loader.post_constructor();
        self.main_thread_init();
        self.init_vm();
        println!("init vm! {:?}", Local::now());
        self.exec_main();
        //        }).unwrap();
        //        join_handler.join().expect_err("thread::spawn failed");
    }

    fn init_vm(&mut self) {
        let vm_class = self
            .boot_class_loader
            .find_or_create("sun/misc/VM")
            .unwrap();
        init_class(vm_class);
        interpret(self.main_thread.clone());

        let ext_class = self
            .boot_class_loader
            .find_or_create("sun/misc/Launcher$ExtClassLoader")
            .unwrap();
        init_class(ext_class.clone());

        let app_class = self
            .boot_class_loader
            .find_or_create("sun/misc/Launcher$AppClassLoader")
            .unwrap();
        init_class(app_class.clone());

        interpret(self.main_thread.clone());
        self.ext_class_loader = self.create_ext_loader(&ext_class);
        self.app_class_loader = self.create_app_loader(&app_class, self.ext_class_loader.clone());
        display_loader_url(self.app_class_loader.clone());
    }

    fn exec_main(&self) {
        let class_name = self.cmd.class.clone().replace('.', "/");
        //let class_name = self.cmd.class.clone();

        let main_class =
            ClassLoader::load_class(self.app_class_loader.clone(), class_name.as_str());
        let main_method = main_class.get_main_method();
        if main_method.is_none() {
            println!("Main method not found in class {}", self.cmd.class.as_str());
            return;
        }
        let args_arr = self.create_args_array();
        let frame = Frame::new(main_method.unwrap());
        frame.set_ref(0, Some(args_arr));
        self.main_thread.push_frame(frame);
        interpret(self.main_thread.clone());
    }

    fn create_args_array(&self) -> Object {
        let string_class = self
            .boot_class_loader
            .find_or_create("java/lang/String")
            .unwrap();
        let args_arr_class =string_class.array_class();
        let args_arr = Class::new_array(&args_arr_class, self.cmd.args.len());
        args_arr.mut_references(|java_args| {
            for i in 0..java_args.len() {
                java_args[i] = Some(StringPool::java_string(self.cmd.args[i].clone()));
            }
        });
        return args_arr;
    }

    fn create_ext_loader(&self, ext_class: &Class) -> Option<Object> {
        let method = ext_class.get_static_method(
            "getExtClassLoader",
            "()Lsun/misc/Launcher$ExtClassLoader;",
        );
        let value = JavaCall::invoke(method.unwrap(), None, ReturnType::Object);
        return value.object();
    }

    fn create_app_loader(
        &self,
        app_class: &Class,
        parent: Option<Object>,
    ) -> Option<Object> {
        let method = app_class.get_static_method(
            "getAppClassLoader",
            "(Ljava/lang/ClassLoader;)Ljava/lang/ClassLoader;",
        );
        let params = Parameters::with_parameters(vec![Parameter::Object(parent)]);
        let value = JavaCall::invoke(method.unwrap(), Some(params), ReturnType::Object).object();
        return value;
    }

    fn create_initial_thread_group(&self) -> Object {
        let thread_group_class = self.boot_class_loader
            .find_or_create("java/lang/ThreadGroup")
            .unwrap();
        let system_instance = Class::new_object(&thread_group_class);
        let constructor =
            thread_group_class.get_constructor("()V");
        let parameters = Parameters::with_parameters(vec![
            Parameter::Object(Some(system_instance.clone())),
        ]);
        JavaCall::invoke(constructor.unwrap(),Some(parameters),Void);
        Universe::set_system_thread_group(Some(system_instance.clone()));

        let main_instance = Class::new_object(&thread_group_class);
        let constructor_with_param =
            thread_group_class.get_constructor("(Ljava/lang/ThreadGroup;Ljava/lang/String;)V");
        let parameters = Parameters::with_parameters(vec![
            Parameter::Object(Some(main_instance.clone())),
            Parameter::Object(Some(system_instance.clone())),
            Parameter::Object(Some(StringPool::java_string("main".to_string()))),
        ]);
        JavaCall::invoke(constructor_with_param.unwrap(),Some(parameters),Void);
        return main_instance
    }
}

fn display_loader_url(class_loader: Option<Object>) {
    let obj = class_loader.unwrap();
    let ucp = obj.get_ref_var("ucp", "Lsun/misc/URLClassPath;");

    let parent = obj.get_ref_var("parent", "Ljava/lang/ClassLoader;");
    if parent.is_some() {
        let parent = parent.unwrap().class();
        println!("parent:{}", parent.java_name());
    }

    let boot_loader = Jvm::boot_class_loader();
    let class = boot_loader.find_or_create("java/net/URL").unwrap();
    let method =class.get_instance_method( "toString", "()Ljava/lang/String;").unwrap();
    if ucp.is_some() {
        let ucp = ucp.unwrap();
        let path = ucp.get_ref_var("path", "Ljava/util/ArrayList;").unwrap();
        let data = path
            .get_ref_var("elementData", "[Ljava/lang/Object;")
            .unwrap();
        data.references(|objs| {
            for ob in objs {
                if ob.is_some() {
                    let param = Parameters::with_parameters(vec![Parameter::Object(ob.clone())]);
                    let string =
                        JavaCall::invoke(method.clone(), Some(param), ReturnType::Object).object();
                    let rust_str = java_str_to_rust_str(string.unwrap());
                    println!("URL:{}", rust_str);
                }
            }
        });
    }
}
