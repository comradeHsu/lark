use std::path::Path;
use std::fs::File;
use std::io::Read;
use crate::class_path::class_path::{Entry, FindClassError};

#[derive(Debug)]
pub struct DirEntry {

    pub abs_dir:String

}

impl DirEntry {
    pub fn new(class_path: &String) -> DirEntry {
        let path = Path::new(class_path);
        if !path.exists() {
            panic!("error")
        }
        return DirEntry{
            abs_dir: String::from(class_path)
        };
    }
}

impl Entry for DirEntry {
    fn read_class(&self,class_name: &str) -> Result<(Vec<u8>,Box<dyn Entry>),FindClassError>{
        let mut file_path = self.abs_dir.clone();
        file_path.push_str("/");
        file_path.push_str(class_name);
        let path = Path::new(&file_path);
        let result = File::open(path);
        let file = match result {
            Err(e) => return Err(FindClassError(e.to_string())),
            Ok(f) => f
        };
        let mut bytes = Vec::new();
        file.bytes().for_each(|x| bytes.push(x.unwrap()));
        return Ok((bytes,Box::new(DirEntry{
            abs_dir: String::from(&self.abs_dir)
        })));
    }

    fn to_string(&self) -> String {
        return String::from(&self.abs_dir);
    }
}

#[cfg(test)]
mod tests{
    use std::path::Path;
    use std::fs::File;
    use crate::class_path::class_path::FindClassError;
    use std::fs;

    #[test]
    fn open_file() {
        let path = Path::new("D:\\test");
        let result = fs::read_dir(path);
        let file = match result {
            Err(e) => panic!("打不开"),
            Ok(f) => f
        };
    }
}