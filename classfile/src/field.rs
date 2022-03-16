use crate::attribute::Attribute;

#[derive(Debug, Clone)]
pub struct FieldInfo<'a> {
    pub access_flags: u16,
    pub name_index: u16,
    pub descriptor_index: u16,
    pub attributes: Vec<Attribute<'a>>,
}
