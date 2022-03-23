use crate::rtda::heap::class::Class;
use crate::rtda::heap::field::Field;
use crate::rtda::heap::method::Method;
use classfile::{get_str, ConstantPoolRef};

#[derive(Debug, Clone)]
pub enum Constant<'a> {
    Placeholder,
    Integer(i32),
    Float(f32),
    Long(i64),
    Double(f64),
    String(&'a str),
    Utf8(&'a [u8]),
    Class {
        class: &'a Class<'a>,
    },
    FieldRef {
        field: &'a Field<'a>,
        const_value_index: u16,
        slot_id: u16,
    },
    MethodRef {
        method: &'a Method<'a>,
    },
    InterfaceMethodRef {
        method: &'a Method<'a>,
    },
    NameAndType {
        name: &'a str,
        descriptor: &'a str,
    },
    MethodHandle,
    MethodType {
        descriptor: &'a str,
    },
    Module {
        name: &'a str,
    },
    Package {
        name: &'a str,
    },
}

#[derive(Debug, Clone)]
pub struct ConstantPool<'a> {
    consts: Vec<Constant<'a>>,
    class: &'a Class<'a>,
}

impl<'a, 'b: 'a> ConstantPool<'a> {
    pub fn new(class: &'a Class, cp: ConstantPoolRef<'b>) -> ConstantPool<'a> {
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
                    consts.push(Constant::String(get_str(
                        cp.clone(),
                        *string_index as usize,
                    )));
                }
                classfile::Constant::Utf8(utf8) => {
                    consts.push(Constant::Utf8(utf8));
                }
                classfile::Constant::MethodType { descriptor_index } => {
                    consts.push(Constant::MethodType {
                        descriptor: get_str(cp.clone(), *descriptor_index as usize),
                    });
                }
                classfile::Constant::Module { name_index } => {
                    consts.push(Constant::Module {
                        name: get_str(cp.clone(), *name_index as usize),
                    });
                }
                classfile::Constant::Package { name_index } => {
                    consts.push(Constant::Package {
                        name: get_str(cp.clone(), *name_index as usize),
                    });
                }
                _ => {
                    consts.push(Constant::Placeholder);
                }
            }
        }
        ConstantPool { consts, class }
    }

    pub fn get(&'a self, index: usize) -> &'a Constant<'a> {
        &self.consts[index]
    }

    pub fn get_utf8(&'a self, index: usize) -> &'a [u8] {
        match self.get(index as usize) {
            Constant::Utf8(utf8) => utf8,
            _ => panic!("java.lang.ClassFormatError"),
        }
    }

    pub fn get_str(&'a self, index: usize) -> &'a str {
        match self.get(index as usize) {
            Constant::Utf8(utf8) => std::str::from_utf8(utf8).unwrap(),
            Constant::String(string) => string,
            _ => panic!("java.lang.ClassFormatError"),
        }
    }
}
