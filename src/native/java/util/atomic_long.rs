use crate::native::registry::Registry;
use crate::runtime::frame::Frame;

pub fn init() {
    Registry::register(
        "java/util/concurrent/atomic/AtomicLong",
        "VMSupportsCS8",
        "()Z",
        vm_supports_cs8,
    )
}

/// java/util/concurrent/atomic/AtomicLong.VMSupportsCS8()Z
pub fn vm_supports_cs8(frame: &Frame) {
    frame.push_boolean(false);
}
