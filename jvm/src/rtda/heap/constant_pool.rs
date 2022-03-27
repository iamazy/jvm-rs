use crate::rtda::heap::class::Class;
use crate::rtda::heap::field::Field;
use crate::rtda::heap::method::Method;
use classfile::{get_str, ConstantPoolRef};
use std::marker::PhantomPinned;
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
    Class(ClassRef),
    FieldRef(FieldRef),
    MethodRef(MethodRef),
    InterfaceMethodRef(MethodRef),
    NameAndType { name: String, descriptor: String },
    MethodHandle,
    MethodType { descriptor: String },
    Module { name: String },
    Package { name: String },
}

#[derive(Debug, Clone)]
pub struct ClassRef {
    pub name: String,
    constant_pool: Option<NonNull<ConstantPool>>,
    class: Option<NonNull<Class>>,
}

#[derive(Debug, Clone)]
pub struct MethodRef {
    pub name: String,
    pub descriptor: String,
    pub class_name: String,
    constant_pool: Option<NonNull<ConstantPool>>,
    class: Option<NonNull<Class>>,
    method: Option<NonNull<Method>>,
}

pub type InterfaceMethodRef = MethodRef;

#[derive(Debug, Clone)]
pub struct FieldRef {
    pub name: String,
    pub descriptor: String,
    pub class_name: String,
    constant_pool: Option<NonNull<ConstantPool>>,
    class: Option<NonNull<Class>>,
    field: Option<NonNull<Field>>,
}

#[derive(Debug, Clone)]
pub struct ConstantPool {
    class: NonNull<Class>,
    consts: Vec<Constant>,
    _pin: PhantomPinned,
}

impl ConstantPool {
    pub fn new(capacity: usize) -> Self {
        Self {
            class: NonNull::dangling(),
            consts: Vec::with_capacity(capacity),
            _pin: PhantomPinned,
        }
    }

    pub fn len(&self) -> usize {
        self.consts.len()
    }

    pub fn fill(&mut self, class: &mut Class, cp: ConstantPoolRef) {
        self.class = NonNull::from(class);
        for constant in cp.iter() {
            match constant {
                classfile::Constant::Placeholder => {
                    self.consts.push(Constant::Placeholder);
                }
                classfile::Constant::Integer(i) => {
                    self.consts.push(Constant::Integer(*i));
                }
                classfile::Constant::Float(f) => {
                    self.consts.push(Constant::Float(*f));
                }
                classfile::Constant::Long(l) => {
                    self.consts.push(Constant::Long(*l));
                }
                classfile::Constant::Double(d) => {
                    self.consts.push(Constant::Double(*d));
                }
                classfile::Constant::String { string_index } => {
                    self.consts.push(Constant::String(
                        get_str(cp.clone(), *string_index as usize).to_string(),
                    ));
                }
                classfile::Constant::Utf8(utf8) => {
                    self.consts.push(Constant::Utf8(utf8.to_vec()));
                }
                classfile::Constant::NameAndType {
                    name_index,
                    descriptor_index,
                } => {
                    self.consts.push(Constant::NameAndType {
                        name: get_str(cp.clone(), *name_index as usize).to_string(),
                        descriptor: get_str(cp.clone(), *descriptor_index as usize).to_string(),
                    });
                }
                classfile::Constant::MethodType { descriptor_index } => {
                    self.consts.push(Constant::MethodType {
                        descriptor: get_str(cp.clone(), *descriptor_index as usize).to_string(),
                    });
                }
                classfile::Constant::Module { name_index } => {
                    self.consts.push(Constant::Module {
                        name: get_str(cp.clone(), *name_index as usize).to_string(),
                    });
                }
                classfile::Constant::Package { name_index } => {
                    self.consts.push(Constant::Package {
                        name: get_str(cp.clone(), *name_index as usize).to_string(),
                    });
                }
                classfile::Constant::Class { name_index } => {
                    self.consts.push(Constant::Class(ClassRef {
                        name: get_str(cp.clone(), *name_index as usize).to_string(),
                        constant_pool: None,
                        class: None,
                    }));
                }
                classfile::Constant::FieldRef {
                    class_index,
                    name_and_type_index,
                } => {
                    if let Some(classfile::Constant::NameAndType {
                        name_index,
                        descriptor_index,
                    }) = cp.get(*name_and_type_index as usize)
                    {
                        self.consts.push(Constant::FieldRef(FieldRef {
                            name: get_str(cp.clone(), *name_index as usize).to_string(),
                            descriptor: get_str(cp.clone(), *descriptor_index as usize).to_string(),
                            class_name: get_str(cp.clone(), *class_index as usize).to_string(),
                            constant_pool: None,
                            class: None,
                            field: None,
                        }));
                    }
                }
                classfile::Constant::MethodRef {
                    class_index,
                    name_and_type_index,
                } => {
                    if let Some(classfile::Constant::NameAndType {
                        name_index,
                        descriptor_index,
                    }) = cp.get(*name_and_type_index as usize)
                    {
                        self.consts.push(Constant::MethodRef(MethodRef {
                            name: get_str(cp.clone(), *name_index as usize).to_string(),
                            descriptor: get_str(cp.clone(), *descriptor_index as usize).to_string(),
                            class_name: get_str(cp.clone(), *class_index as usize).to_string(),
                            constant_pool: None,
                            class: None,
                            method: None,
                        }));
                    }
                }
                classfile::Constant::InterfaceMethodRef {
                    class_index,
                    name_and_type_index,
                } => {
                    if let Some(classfile::Constant::NameAndType {
                        name_index,
                        descriptor_index,
                    }) = cp.get(*name_and_type_index as usize)
                    {
                        self.consts
                            .push(Constant::InterfaceMethodRef(InterfaceMethodRef {
                                name: get_str(cp.clone(), *name_index as usize).to_string(),
                                descriptor: get_str(cp.clone(), *descriptor_index as usize)
                                    .to_string(),
                                class_name: get_str(cp.clone(), *class_index as usize).to_string(),
                                constant_pool: None,
                                class: None,
                                method: None,
                            }));
                    }
                }
                _ => {
                    self.consts.push(Constant::Placeholder);
                }
            }
        }
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

    pub fn get_str(&self, index: usize) -> &String {
        match self.get(index as usize) {
            Constant::String(string) => string,
            Constant::Class(ClassRef { name, .. }) => name,
            n => {
                println!("{:?}", n);
                panic!("java.lang.ClassFormatError")
            }
        }
    }
}
