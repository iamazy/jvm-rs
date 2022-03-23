use crate::rtda::heap::class::Class;
use classfile::MethodInfo;

#[derive(Debug)]
pub struct Method<'a> {
    pub access_flags: u16,
    pub name: &'a str,
    pub descriptor: &'a str,
    pub class: &'a Class<'a>,
    pub max_stack: Option<usize>,
    pub max_locals: Option<usize>,
    pub code: Option<&'a [u8]>,
}

impl<'a> Method<'a> {
    pub fn new(class: &'a Class, method_info: &'a classfile::MethodInfo) -> Self {
        let name = class.constant_pool.get_str(method_info.name_index as usize);
        let descriptor = class
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
            method.code = Some(code.code);
        }
        method
    }
}

pub fn new_methods<'a>(class: &'a Class, method_infos: Vec<&'a MethodInfo>) -> Vec<Method<'a>> {
    method_infos
        .into_iter()
        .map(|method_info| Method::new(class, method_info))
        .collect()
}
