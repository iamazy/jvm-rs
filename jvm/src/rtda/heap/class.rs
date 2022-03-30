use crate::rtda::heap::access_flags::AccessFlag;
use crate::rtda::heap::class_loader::ClassLoader;
use crate::rtda::heap::constant_pool::{Constant, ConstantPool};
use crate::rtda::heap::field::{new_fields, Field};
use crate::rtda::heap::method::{new_methods, Method};
use crate::rtda::LocalVars;
use classfile::{get_str, ClassFile};
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
    pub static_vars: Option<LocalVars>,
}

impl Class {
    pub fn new(class_file: &ClassFile) -> Class {
        let access_flags = class_file.access_flags;
        // initialize constant pool
        let constant_pool = ConstantPool::new(class_file.constant_pool.clone());
        // this class
        let name = get_str(
            class_file.constant_pool.clone(),
            class_file.this_class as usize,
        );

        //  super class
        let mut super_class_name = None;
        if class_file.super_class > 0 {
            super_class_name = Some(get_str(
                class_file.constant_pool.clone(),
                class_file.super_class as usize,
            ));
        }

        // interface names
        let mut interface_names = Vec::with_capacity(class_file.interfaces.len());
        for interface in &class_file.interfaces {
            interface_names.push(get_str(
                class_file.constant_pool.clone(),
                *interface as usize,
            ));
        }

        let mut class = Self {
            access_flags,
            name,
            super_class_name,
            interface_names,
            constant_pool: Box::leak(Box::new(constant_pool)).into(),
            loader: NonNull::dangling(),
            fields: Vec::with_capacity(class_file.fields.len()),
            methods: Vec::with_capacity(class_file.methods.len()),
            super_class: None,
            interfaces: Vec::with_capacity(class_file.interfaces.len()),
            instance_slot_count: 0,
            static_slot_count: 0,
            static_vars: None,
        };

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

    pub fn calc_instance_field_slot_ids(&mut self) {
        let mut slot_id: usize = 0;
        if self.super_class.is_some() {
            slot_id = unsafe { self.super_class.unwrap().as_ref().instance_slot_count };
        }
        for field in self.fields.iter_mut() {
            if !field.is_static() {
                field.slot_id = slot_id;
                slot_id += 1;
                if field.is_long() || field.is_double() {
                    slot_id += 1;
                }
            }
        }
        self.instance_slot_count = slot_id;
    }

    pub fn calc_static_field_slot_ids(&mut self) {
        let mut slot_id: usize = 0;
        for field in self.fields.iter_mut() {
            if field.is_static() {
                field.slot_id = slot_id;
                slot_id += 1;
                if field.is_long() || field.is_double() {
                    slot_id += 1;
                }
            }
        }
        self.static_slot_count = slot_id;
    }

    pub fn alloc_init_static_vars(&mut self) {
        self.static_vars = Some(LocalVars::new(self.static_slot_count));
        for field in self.fields.iter_mut().as_ref() {
            if field.is_static() && field.is_final() && field.const_value_index > 0 {
                match field.descriptor.as_str() {
                    "Z" | "B" | "C" | "S" | "I" => {
                        if let Constant::Integer(int) = unsafe {
                            self.constant_pool
                                .as_ref()
                                .get(field.const_value_index as usize)
                        } {
                            self.static_vars
                                .as_mut()
                                .unwrap()
                                .set_int(field.slot_id, *int);
                        }
                    }
                    "J" => {
                        if let Constant::Long(long) = unsafe {
                            self.constant_pool
                                .as_ref()
                                .get(field.const_value_index as usize)
                        } {
                            self.static_vars
                                .as_mut()
                                .unwrap()
                                .set_long(field.slot_id, *long);
                        }
                    }
                    "F" => {
                        if let Constant::Float(float) = unsafe {
                            self.constant_pool
                                .as_ref()
                                .get(field.const_value_index as usize)
                        } {
                            self.static_vars
                                .as_mut()
                                .unwrap()
                                .set_float(field.slot_id, *float);
                        }
                    }
                    "D" => {
                        if let Constant::Double(double) = unsafe {
                            self.constant_pool
                                .as_ref()
                                .get(field.const_value_index as usize)
                        } {
                            self.static_vars
                                .as_mut()
                                .unwrap()
                                .set_double(field.slot_id, *double);
                        }
                    }
                    _ => {
                        unimplemented!("please implement me")
                    }
                }
            }
        }
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

    pub fn is_accessible_to(&self, class: &Class) -> bool {
        self.is_publish() || self.package_name() == class.package_name()
    }

    pub fn is_sub_class_of(&self, class: NonNull<Class>) -> bool {
        let mut child = self;
        loop {
            if child.super_class.is_some() {
                if child.super_class.unwrap() == class {
                    return true;
                }
                child = unsafe { self.super_class.unwrap().as_ref() };
            } else {
                return false;
            }
        }
    }

    pub fn package_name(&self) -> Option<&str> {
        if let Some(pos) = self.name.rfind('/') {
            return Some(self.name[..pos].as_ref());
        }
        None
    }

    pub fn look_up_field(&self, name: &str, descriptor: &str) -> Option<&Field> {
        for field in self.fields.iter() {
            if field.name == name && field.descriptor == descriptor {
                return Some(field);
            }
        }
        unsafe {
            for interface in self.interfaces.iter() {
                if let Some(field) = interface.as_ref().look_up_field(name, descriptor) {
                    return Some(field);
                }
            }
            if self.super_class.is_some() {
                return self
                    .super_class
                    .unwrap()
                    .as_ref()
                    .look_up_field(name, descriptor);
            }
        }
        return None;
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
                    println!("field: {}", field.name);
                }
                for method in class.methods.iter().as_ref() {
                    println!("method: {}", method.name);
                }
            }
        }
    }
}
