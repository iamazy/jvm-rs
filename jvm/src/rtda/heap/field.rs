use crate::rtda::heap::class::Class;
use classfile::FieldInfo;

#[derive(Debug)]
pub struct Field<'a> {
    pub access_flags: u16,
    pub name: &'a str,
    pub descriptor: &'a str,
    pub class: &'a Class<'a>,
}

impl<'a> Field<'a> {
    pub fn new(class: &'a Class, field_info: &'a FieldInfo) -> Self {
        let name = class.constant_pool.get_str(field_info.name_index as usize);
        let descriptor = class
            .constant_pool
            .get_str(field_info.descriptor_index as usize);
        Self {
            access_flags: field_info.access_flags,
            name,
            descriptor,
            class,
        }
    }
}

pub fn new_fields<'a>(class: &'a Class, field_infos: Vec<&'a FieldInfo>) -> Vec<Field<'a>> {
    field_infos
        .into_iter()
        .map(|field_info| Field::new(class, field_info))
        .collect()
}
