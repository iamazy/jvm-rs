use crate::rtda::heap::access_flags::AccessFlag;
use crate::rtda::heap::constant_pool::ConstantPool;
use crate::rtda::heap::field::{new_fields, Field};
use crate::rtda::heap::method::{new_methods, Method};
use crate::rtda::Slot;
use classfile::{get_str, ClassFile};
use std::ptr::NonNull;

#[derive(Debug)]
pub struct Class {
    pub access_flags: u16,
    pub name: String,
    pub super_class_name: String,
    pub interface_names: Vec<String>,
    pub constant_pool: ConstantPool,
    pub fields: Vec<Field>,
    pub methods: Vec<Method>,
    // pub loader: Box<dyn ClassLoader>,
    pub super_class: Option<NonNull<Class>>,
    pub interfaces: Vec<NonNull<Class>>,
    pub instance_slot_count: usize,
    pub static_slot_count: usize,
    pub static_vars: Vec<Slot>,
}

impl Class {
    pub fn new(class_file: &ClassFile) -> NonNull<Class> {
        let access_flags = class_file.access_flags;
        let constant_pool = class_file.constant_pool.clone();
        let name = get_str(constant_pool.clone(), class_file.this_class as usize).to_string();
        let super_class_name =
            get_str(constant_pool.clone(), class_file.super_class as usize).to_string();
        let interface_names = class_file
            .interfaces
            .iter()
            .map(|interface_index| {
                get_str(constant_pool.clone(), *interface_index as usize).to_string()
            })
            .collect();
        let mut class = Self {
            access_flags,
            name,
            super_class_name,
            interface_names,
            constant_pool: ConstantPool::new(class_file.constant_pool.clone()),
            fields: vec![],
            methods: vec![],
            super_class: None,
            interfaces: vec![],
            instance_slot_count: 0,
            static_slot_count: 0,
            static_vars: vec![],
        };
        let class = NonNull::from(&mut class);
        let fields = new_fields(class, &class_file.fields);
        for field in fields {
            unsafe {
                (*class.as_ptr()).fields.push(field);
            }
        }
        let methods = new_methods(class, &class_file.methods);
        for method in methods {
            unsafe {
                (*class.as_ptr()).methods.push(method);
            }
        }
        class
    }

    // access_flags
    pub fn is_publish(&self) -> bool {
        self.access_flags & AccessFlag::ACC_PUBLIC.bits() != 0
    }

    pub fn is_final(&self) -> bool {
        self.access_flags & AccessFlag::ACC_FINAL.bits() != 0
    }

    pub fn is_super(&self) -> bool {
        self.access_flags & AccessFlag::ACC_SUPER.bits() != 0
    }

    pub fn is_interface(&self) -> bool {
        self.access_flags & AccessFlag::ACC_INTERFACE.bits() != 0
    }

    pub fn is_abstract(&self) -> bool {
        self.access_flags & AccessFlag::ACC_ABSTRACT.bits() != 0
    }

    pub fn is_synthetic(&self) -> bool {
        self.access_flags & AccessFlag::ACC_SYNTHETIC.bits() != 0
    }

    pub fn is_annotation(&self) -> bool {
        self.access_flags & AccessFlag::ACC_ANNOTATION.bits() != 0
    }

    pub fn is_enum(&self) -> bool {
        self.access_flags & AccessFlag::ACC_ENUM.bits() != 0
    }
}
