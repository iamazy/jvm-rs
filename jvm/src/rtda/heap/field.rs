use crate::rtda::heap::class::Class;
use classfile::{get_utf8, FieldInfo};

#[derive(Debug)]
pub struct Field<'a> {
    pub access_flags: u16,
    pub name: &'a [u8],
    pub descriptor: &'a [u8],
    pub class: &'a Class<'a>,
}

impl<'a> Field<'a> {
    pub fn new(class: &'a Class<'a>, field_info: &'a FieldInfo) -> Self {
        Self {
            access_flags: field_info.access_flags,
            name: get_utf8(class.constant_pool.clone(), field_info.name_index as usize),
            descriptor: get_utf8(
                class.constant_pool.clone(),
                field_info.descriptor_index as usize,
            ),
            class,
        }
    }
}

pub fn new_fields<'a>(class: &'a Class<'a>, field_infos: Vec<&'a FieldInfo>) -> Vec<Field<'a>> {
    field_infos
        .into_iter()
        .map(|field_info| Field::new(class, field_info))
        .collect()
}
