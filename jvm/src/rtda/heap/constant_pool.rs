use crate::rtda::heap::class::Class;
use crate::rtda::heap::field::Field;
use crate::rtda::heap::method::Method;
use classfile::{get_str, ConstantPoolRef};
use std::ptr::NonNull;

#[derive(Debug, Clone)]
pub enum Constant {
    Placeholder,
    Integer(i32),
    Float(f32),
    Long(i64),
    Double(f64),
    String(String),
    Utf8(Vec<u8>),
    Class { class: NonNull<Class> },
    FieldRef { field: NonNull<Field> },
    MethodRef { method: NonNull<Method> },
    InterfaceMethodRef { method: NonNull<Method> },
    NameAndType { name: String, descriptor: String },
    MethodHandle,
    MethodType { descriptor: String },
    Module { name: String },
    Package { name: String },
}

#[derive(Debug, Clone)]
pub struct ConstantPool {
    consts: Vec<Constant>,
}

impl ConstantPool {
    pub fn new(cp: ConstantPoolRef) -> ConstantPool {
        let mut consts = Vec::with_capacity(cp.len());
        for constant in cp.iter() {
            match constant {
                classfile::Constant::Placeholder => {
                    consts.push(Constant::Placeholder);
                }
                classfile::Constant::Integer(i) => {
                    consts.push(Constant::Integer(*i));
                }
                classfile::Constant::Float(f) => {
                    consts.push(Constant::Float(*f));
                }
                classfile::Constant::Long(l) => {
                    consts.push(Constant::Long(*l));
                }
                classfile::Constant::Double(d) => {
                    consts.push(Constant::Double(*d));
                }
                classfile::Constant::String { string_index } => {
                    consts.push(Constant::String(
                        get_str(cp.clone(), *string_index as usize).to_string(),
                    ));
                }
                classfile::Constant::Utf8(utf8) => {
                    consts.push(Constant::Utf8(utf8.to_vec()));
                }
                classfile::Constant::MethodType { descriptor_index } => {
                    consts.push(Constant::MethodType {
                        descriptor: get_str(cp.clone(), *descriptor_index as usize).to_string(),
                    });
                }
                classfile::Constant::Module { name_index } => {
                    consts.push(Constant::Module {
                        name: get_str(cp.clone(), *name_index as usize).to_string(),
                    });
                }
                classfile::Constant::Package { name_index } => {
                    consts.push(Constant::Package {
                        name: get_str(cp.clone(), *name_index as usize).to_string(),
                    });
                }
                _ => {
                    consts.push(Constant::Placeholder);
                }
            }
        }
        ConstantPool { consts }
    }

    pub fn get(&self, index: usize) -> &Constant {
        &self.consts[index]
    }

    pub fn get_utf8(&self, index: usize) -> &Vec<u8> {
        match self.get(index as usize) {
            Constant::Utf8(utf8) => utf8,
            _ => panic!("java.lang.ClassFormatError"),
        }
    }

    pub fn get_str(&self, index: usize) -> String {
        match self.get(index as usize) {
            Constant::Utf8(utf8) => match String::from_utf8(utf8.clone()) {
                Ok(str) => str,
                _ => panic!("java.lang.ClassFormatError"),
            },
            Constant::String(string) => string.to_string(),
            _ => panic!("java.lang.ClassFormatError"),
        }
    }
}
