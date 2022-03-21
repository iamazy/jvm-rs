use crate::rtda::heap::class::Class;
use classfile::get_utf8;

#[derive(Debug)]
pub struct Method<'a> {
    pub access_flags: u16,
    pub name: &'a [u8],
    pub descriptor: &'a [u8],
    pub class: &'a Class<'a>,
    pub max_stack: Option<usize>,
    pub max_locals: Option<usize>,
    pub code: Option<&'a [u8]>,
}

impl<'a> Method<'a> {
    pub fn new(class: &'a Class<'a>, method_info: &'a classfile::MethodInfo) -> Self {
        let mut method = Method {
            access_flags: method_info.access_flags,
            name: get_utf8(class.constant_pool.clone(), method_info.name_index as usize),
            descriptor: get_utf8(
                class.constant_pool.clone(),
                method_info.descriptor_index as usize,
            ),
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
