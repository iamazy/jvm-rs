use crate::rtda::heap::class::Class;
use crate::rtda::heap::field::Field;
use crate::rtda::heap::method::Method;
use anyhow::anyhow;
use classfile::{get_str, ConstantPoolRef};
use jvm_macros::SymbolRef;
use std::cell::RefCell;
use std::ptr::NonNull;
use std::sync::Arc;

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

pub trait SymbolicRef {
    fn resolved_class_ref(&mut self) -> anyhow::Result<()>;
}

#[derive(Debug, Clone, SymbolRef)]
pub struct ClassRef {
    pub name: String,
    constant_pool: NonNull<ConstantPool>,
    class: Option<NonNull<Class>>,
}

#[derive(Debug, Clone, SymbolRef)]
pub struct MethodRef {
    pub name: String,
    pub descriptor: String,
    pub class_name: String,
    constant_pool: NonNull<ConstantPool>,
    class: Option<NonNull<Class>>,
    method: Option<NonNull<Method>>,
}

pub type InterfaceMethodRef = MethodRef;

#[derive(Debug, Clone, SymbolRef)]
pub struct FieldRef {
    pub name: String,
    pub descriptor: String,
    pub class_name: String,
    pub constant_pool: NonNull<ConstantPool>,
    class: Option<NonNull<Class>>,
    field: Option<Arc<RefCell<Field>>>,
}

impl FieldRef {
    pub fn resolve_field(&mut self) -> anyhow::Result<Arc<RefCell<Field>>> {
        if self.field.is_none() {
            let _ = self.resolve_field_ref();
        }
        Ok(self.field.clone().unwrap())
    }

    pub fn resolve_field_ref(&mut self) -> anyhow::Result<()> {
        unsafe {
            let cp_class = self.constant_pool.as_ref().class.as_ref();
            let self_class = self.resolved_class()?;
            let field = self_class
                .as_ref()
                .look_up_field(self.name.as_str(), self.descriptor.as_str());
            if field.is_none() {
                panic!("java.lang.NoSuchFieldError");
            }
            if !field.clone().unwrap().borrow().is_accessible_to(cp_class) {
                panic!("java.lang.IllegalAccessError");
            }
            self.field = field;
            Ok(())
        }
    }
}

#[derive(Debug, Clone)]
pub struct ConstantPool {
    pub class: NonNull<Class>,
    pub consts: Vec<Constant>,
}

impl ConstantPool {
    pub fn new(cp: ConstantPoolRef) -> Self {
        let mut constant_pool = Self {
            class: NonNull::dangling(),
            consts: Vec::with_capacity(cp.len()),
        };
        for constant in cp.iter() {
            match constant {
                classfile::Constant::Integer(i) => {
                    constant_pool.consts.push(Constant::Integer(*i));
                }
                classfile::Constant::Float(f) => {
                    constant_pool.consts.push(Constant::Float(*f));
                }
                classfile::Constant::Long(l) => {
                    constant_pool.consts.push(Constant::Long(*l));
                }
                classfile::Constant::Double(d) => {
                    constant_pool.consts.push(Constant::Double(*d));
                }
                classfile::Constant::String { string_index } => {
                    constant_pool.consts.push(Constant::String(get_str(
                        cp.clone(),
                        *string_index as usize,
                    )));
                }
                classfile::Constant::Utf8(utf8) => {
                    constant_pool.consts.push(Constant::Utf8(utf8.to_vec()));
                }
                classfile::Constant::NameAndType {
                    name_index,
                    descriptor_index,
                } => {
                    constant_pool.consts.push(Constant::NameAndType {
                        name: get_str(cp.clone(), *name_index as usize),
                        descriptor: get_str(cp.clone(), *descriptor_index as usize),
                    });
                }
                classfile::Constant::MethodType { descriptor_index } => {
                    constant_pool.consts.push(Constant::MethodType {
                        descriptor: get_str(cp.clone(), *descriptor_index as usize),
                    });
                }
                classfile::Constant::Module { name_index } => {
                    constant_pool.consts.push(Constant::Module {
                        name: get_str(cp.clone(), *name_index as usize),
                    });
                }
                classfile::Constant::Package { name_index } => {
                    constant_pool.consts.push(Constant::Package {
                        name: get_str(cp.clone(), *name_index as usize),
                    });
                }
                classfile::Constant::Class { name_index } => {
                    let constant = Constant::Class(ClassRef {
                        name: get_str(cp.clone(), *name_index as usize),
                        constant_pool: NonNull::from(&mut constant_pool),
                        class: None,
                    });
                    constant_pool.consts.push(constant);
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
                        let constant = Constant::FieldRef(FieldRef {
                            name: get_str(cp.clone(), *name_index as usize),
                            descriptor: get_str(cp.clone(), *descriptor_index as usize),
                            class_name: get_str(cp.clone(), *class_index as usize),
                            constant_pool: NonNull::from(&mut constant_pool),
                            class: None,
                            field: None,
                        });
                        constant_pool.consts.push(constant);
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
                        let constant = Constant::MethodRef(MethodRef {
                            name: get_str(cp.clone(), *name_index as usize),
                            descriptor: get_str(cp.clone(), *descriptor_index as usize),
                            class_name: get_str(cp.clone(), *class_index as usize),
                            constant_pool: NonNull::from(&mut constant_pool),
                            class: None,
                            method: None,
                        });
                        constant_pool.consts.push(constant);
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
                        let interface_method_ref =
                            Constant::InterfaceMethodRef(InterfaceMethodRef {
                                name: get_str(cp.clone(), *name_index as usize),
                                descriptor: get_str(cp.clone(), *descriptor_index as usize),
                                class_name: get_str(cp.clone(), *class_index as usize),
                                constant_pool: NonNull::from(&mut constant_pool),
                                class: None,
                                method: None,
                            });
                        constant_pool.consts.push(interface_method_ref);
                    }
                }
                _ => {
                    constant_pool.consts.push(Constant::Placeholder);
                }
            }
        }
        constant_pool
    }

    pub fn len(&self) -> usize {
        self.consts.len()
    }

    pub fn get(&self, index: usize) -> &Constant {
        &self.consts[index]
    }

    pub fn get_mut(&mut self, index: usize) -> &mut Constant {
        &mut self.consts[index]
    }

    pub fn get_utf8(&self, index: usize) -> Vec<u8> {
        match self.get(index as usize) {
            Constant::Utf8(utf8) => utf8.clone(),
            _ => panic!("java.lang.ClassFormatError"),
        }
    }
}
