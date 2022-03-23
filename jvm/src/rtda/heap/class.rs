use crate::rtda::heap::constant_pool::ConstantPool;
use crate::rtda::heap::field::Field;
use crate::rtda::heap::method::Method;
use crate::rtda::Slot;
use classfile::{get_utf8, ClassFile};
use std::sync::Arc;

#[derive(Debug)]
pub struct Class<'a> {
    pub access_flags: u16,
    pub name: &'a str,
    pub super_class_name: &'a str,
    pub interface_names: Vec<&'a str>,
    pub constant_pool: ConstantPool<'a>,
    pub fields: Vec<&'a Field<'a>>,
    pub methods: Vec<&'a Method<'a>>,
    // pub loader: Box<dyn ClassLoader>,
    pub super_class: Option<Arc<Class<'a>>>,
    pub interfaces: Vec<Arc<Class<'a>>>,
    pub instance_slot_count: usize,
    pub static_slot_count: usize,
    pub static_vars: Vec<&'a Slot>,
}

impl<'a> Class<'a> {
    // pub fn new(class_file: &ClassFile) -> Self {
    //     let access_flags = class_file.access_flags;
    //     let constant_pool = class_file.constant_pool.clone();
    //     let name = get_utf8(constant_pool.clone(), class_file.this_class as usize);
    //     let super_class_name = get_utf8(constant_pool.clone(), class_file.super_class as usize);
    //     let interface_names = class_file
    //         .interfaces
    //         .iter()
    //         .map(|interface_index| get_utf8(constant_pool.clone(), *interface_index as usize))
    //         .collect();
    //     let fields = class_file
    //         .fields
    //         .iter()
    //         .map(|field| field.clone())
    //         .collect();
    //     let methods = class_file
    //         .methods
    //         .iter()
    //         .map(|method| method.clone())
    //         .collect();
    // }
}
