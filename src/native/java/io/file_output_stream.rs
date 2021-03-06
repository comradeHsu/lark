use crate::native::registry::Registry;
use crate::runtime::frame::Frame;
use std::io;
use std::io::Write;

pub fn init() {
    Registry::register(
        "java/io/FileOutputStream",
        "writeBytes",
        "([BIIZ)V",
        write_bytes,
    );
    Registry::register("java/io/FileOutputStream", "initIDs", "()V", init_ids);
}

// private native void writeBytes(byte b[], int off, int len, boolean append) throws IOException;
// ([BIIZ)V
pub fn write_bytes(frame: &mut Frame) {
    let vars = frame.local_vars().expect("vars is none");
    let b = vars.get_ref(1).unwrap();
    let off = vars.get_int(2) as usize;
    let len = vars.get_int(3) as usize;
    let borrow = (*b).borrow();
    let java_bytes = borrow.bytes();
    let bytes = byte_change(java_bytes);
    let slice = &bytes[off..(off + len)];
    let mut out = io::stdout();
    let rs = out.write(slice);
    if rs.is_err() {
        println!("Error:{:?}", rs.err().unwrap())
    }
}

fn byte_change(java_bytes: &Vec<i8>) -> Vec<u8> {
    let mut vec = Vec::with_capacity(java_bytes.len());
    for java_byte in java_bytes {
        vec.push(*java_byte as u8);
    }
    return vec;
}

pub fn init_ids(frame: &mut Frame) {}

#[cfg(test)]
mod test {
    use std::io;
    use std::io::Write;

    #[test]
    fn test_stdout() {
        let str = "123456789".to_string();
        let bytes = str.as_bytes();
        let mut out = io::stdout();
        out.write(bytes);
    }

    #[test]
    fn test_char() {
        let bytes: Vec<u8> = vec![0; 8];
        let string = String::from_utf8(bytes).unwrap();
        print!("{},", string);
    }
}
