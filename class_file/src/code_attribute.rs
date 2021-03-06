use crate::attribute_info::{read_attributes, Attribute, AttributeInfo};
use crate::class_reader::ClassReader;
use crate::constant_pool::ConstantPool;
use crate::line_number_table_attribute::LineNumberTableAttribute;
use std::cell::RefCell;
use std::rc::Rc;

pub struct CodeAttribute {
    cp: Rc<RefCell<ConstantPool>>,
    max_stack: u16,
    max_locals: u16,
    code: Vec<u8>,
    exception_table: Vec<ExceptionTableEntry>,
    attributes: Vec<Attribute>,
}

impl CodeAttribute {
    pub fn new() -> CodeAttribute {
        return CodeAttribute {
            cp: Rc::new(RefCell::new(ConstantPool::new())),
            max_stack: 0,
            max_locals: 0,
            code: vec![],
            exception_table: vec![],
            attributes: vec![],
        };
    }

    pub fn with_cp(cp: Rc<RefCell<ConstantPool>>) -> CodeAttribute {
        return CodeAttribute {
            cp,
            max_stack: 0,
            max_locals: 0,
            code: vec![],
            exception_table: vec![],
            attributes: vec![],
        };
    }

    #[inline]
    pub fn max_stack(&self) -> u32 {
        return self.max_stack as u32;
    }

    #[inline]
    pub fn max_locals(&self) -> u32 {
        return self.max_locals as u32;
    }

    #[inline]
    pub fn code(&self) -> &Vec<u8> {
        return &self.code;
    }

    #[inline]
    pub fn exception_table(&self) -> &Vec<ExceptionTableEntry> {
        return &self.exception_table;
    }

    pub fn line_number_table_attribute(&self) -> Option<LineNumberTableAttribute> {
        for attribute in &self.attributes {
            match attribute {
                Attribute::LineNumberTable(attr) => return Some(attr.unsafe_copy()),
                _ => {}
            }
        }
        return None;
    }

    pub fn display(&self) {
        println!("CodeAttribute:");
        println!("  max_stack:{}", self.max_stack);
        println!("  max_locals:{}", self.max_locals);
        println!("  code:{:?}", self.code.clone());
        println!("  exception_table_len:{}", self.exception_table.len());
        println!("  attributes_len:{}", self.attributes.len());
    }
}

impl AttributeInfo for CodeAttribute {
    fn read_info(&mut self, reader: &mut ClassReader) {
        self.max_stack = reader.read_u16();
        self.max_locals = reader.read_u16();
        let code_len = reader.read_u32();
        self.code = reader.read_bytes(code_len as usize);
        self.exception_table = ExceptionTableEntry::read_exception_table(reader);
        self.attributes = read_attributes(reader, self.cp.clone());
    }
}

pub struct ExceptionTableEntry {
    start_pc: u16,
    end_pc: u16,
    handler_pc: u16,
    catch_type: u16,
}

impl ExceptionTableEntry {
    pub fn read_exception_table(reader: &mut ClassReader) -> Vec<ExceptionTableEntry> {
        let exception_table_len = reader.read_u16();
        let mut exception_table = Vec::new();
        for _ in 0..exception_table_len {
            exception_table.push(ExceptionTableEntry {
                start_pc: reader.read_u16(),
                end_pc: reader.read_u16(),
                handler_pc: reader.read_u16(),
                catch_type: reader.read_u16(),
            })
        }
        return exception_table;
    }

    #[inline]
    pub fn start_pc(&self) -> u16 {
        return self.start_pc;
    }

    #[inline]
    pub fn end_pc(&self) -> u16 {
        return self.end_pc;
    }

    #[inline]
    pub fn handler_pc(&self) -> u16 {
        return self.handler_pc;
    }

    #[inline]
    pub fn catch_type(&self) -> u16 {
        return self.catch_type;
    }
}
