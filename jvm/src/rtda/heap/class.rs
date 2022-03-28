use crate::rtda::heap::access_flags::AccessFlag;
use crate::rtda::heap::class_loader::ClassLoader;
use crate::rtda::heap::constant_pool::ConstantPool;
use crate::rtda::heap::field::{new_fields, Field};
use crate::rtda::heap::method::{new_methods, Method};
use crate::rtda::Slot;
use classfile::{ClassFile, get_str};
use std::ptr::NonNull;

#[derive(Debug)]
pub struct Class {
    pub access_flags: u16,
    // index of Constant::Class in constant pool
    pub name: String,
    pub super_class_name: Option<String>,
    pub interface_names: Vec<String>,
    pub constant_pool: NonNull<ConstantPool>,
    pub fields: Vec<Field>,
    pub methods: Vec<Method>,
    pub loader: NonNull<ClassLoader>,
    pub super_class: Option<NonNull<Class>>,
    pub interfaces: Vec<NonNull<Class>>,
    pub instance_slot_count: usize,
    pub static_slot_count: usize,
    pub static_vars: Vec<Slot>,
}

impl Class {
    pub fn new(class_file: &ClassFile) -> Class {
        let access_flags = class_file.access_flags;
        // initialize constant pool
        let constant_pool = ConstantPool::new(class_file.constant_pool.clone());
        // this class
        let name = get_str(class_file.constant_pool.clone(), class_file.this_class as usize);

        //  super class
        let mut super_class_name = None;
        if class_file.super_class > 0 {
            super_class_name = Some(get_str(class_file.constant_pool.clone(), class_file.super_class as usize));
        }

        // interface names
        let mut interface_names = Vec::with_capacity(class_file.interfaces.len());
        for interface in &class_file.interfaces {
            interface_names.push(get_str(class_file.constant_pool.clone(), *interface as usize));
        }

        let mut class = Self {
            access_flags,
            name,
            super_class_name,
            interface_names,
            constant_pool: NonNull::dangling(),
            loader: NonNull::dangling(),
            fields: Vec::with_capacity(class_file.fields.len()),
            methods: Vec::with_capacity(class_file.methods.len()),
            super_class: None,
            interfaces: Vec::with_capacity(class_file.interfaces.len()),
            instance_slot_count: 0,
            static_slot_count: 0,
            static_vars: vec![],
        };

        class.constant_pool = Box::leak(Box::new(constant_pool)).into();

        // initialize fields
        let fields = new_fields(&mut class, &class_file.fields);
        for field in fields {
            class.fields.push(field);
        }

        // initialize methods
        let methods = new_methods(&mut class, &class_file.methods);
        for method in methods {
            class.methods.push(method);
        }
        class
    }

    // access_flags
    pub fn is_publish(&self) -> bool {
        self.access_flags & AccessFlag::ACC_PUBLIC.bits() != 0
    }

    pub fn is_private(&self) -> bool {
        self.access_flags & AccessFlag::ACC_PRIVATE.bits() != 0
    }

    pub fn is_protected(&self) -> bool {
        self.access_flags & AccessFlag::ACC_PROTECTED.bits() != 0
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

#[cfg(test)]
mod tests {
    use crate::classpath::{ClassPath, Entry};
    use crate::rtda::heap::class::Class;

    #[test]
    fn test_read_class() {
        let class_path = ClassPath::new("".to_string(), "../data/jvm8".to_string());
        if let Ok(class_bytes) = class_path.read_class("User") {
            if let Ok((_, ref class_file)) = classfile::parse(class_bytes.as_slice()) {
                let class = Class::new(class_file);
                assert_eq!(class.name, "User");
                assert_eq!(class.super_class_name.unwrap(), "java/lang/Object");
                assert_eq!(class.fields.len(), 3);
                for field in class.fields.iter().as_ref() {
                    println!("{}", field.name);
                }
            }
        }
    }
}
