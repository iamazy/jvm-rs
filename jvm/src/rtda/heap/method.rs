use crate::rtda::heap::class::Class;
use classfile::MethodInfo;
use std::ptr::NonNull;

#[derive(Debug)]
pub struct Method {
    pub access_flags: u16,
    pub name: String,
    pub descriptor: String,
    pub class: NonNull<Class>,
    pub max_stack: Option<usize>,
    pub max_locals: Option<usize>,
    pub code: Option<Vec<u8>>,
}

impl Method {
    pub fn new(class: NonNull<Class>, method_info: &classfile::MethodInfo) -> Self {
        let class_ptr = unsafe { Box::from_raw(class.as_ptr()) };
        let name = class_ptr
            .constant_pool
            .get_str(method_info.name_index as usize);
        let descriptor = class_ptr
            .constant_pool
            .get_str(method_info.descriptor_index as usize);
        let mut method = Method {
            access_flags: method_info.access_flags,
            name,
            descriptor,
            class,
            max_stack: None,
            max_locals: None,
            code: None,
        };
        if let Some(code) = method_info.code_attribute() {
            method.max_stack = Some(code.max_stack as usize);
            method.max_locals = Some(code.max_locals as usize);
            method.code = Some(code.code.to_vec());
        }
        method
    }
}

pub fn new_methods(class: NonNull<Class>, method_infos: &Vec<MethodInfo>) -> Vec<Method> {
    method_infos
        .iter()
        .map(|method_info| Method::new(class, method_info))
        .collect()
}
