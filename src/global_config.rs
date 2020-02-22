use std::env::consts::OS;
use crate::cmd::Cmd;

pub struct GlobalConfig {
    boot_lib_path:String,
    os:&'static str,

}

pub static mut GLOBAL_CONFIG:Option<GlobalConfig> = None;

impl GlobalConfig {

    fn new(cmd:&Cmd) -> GlobalConfig {
        return GlobalConfig{
            boot_lib_path: "".to_string(),
            os: OS
        };
    }

    pub fn init(cmd:&Cmd) {
        unsafe {
            if GLOBAL_CONFIG.is_none() {
                GLOBAL_CONFIG = Some(Self::new())
            }
        }
    }

    pub fn instance() -> &'static GlobalConfig {
        unsafe {
            return GLOBAL_CONFIG.as_ref().unwrap();
        }
    }
}