use crate::attribute::Attribute;
use crate::{AttributeType, CodeAttribute};

#[derive(Debug, Clone)]
pub struct MethodInfo<'a> {
    pub access_flags: u16,
    pub name_index: u16,
    pub descriptor_index: u16,
    pub attributes: Vec<Attribute<'a>>,
    pub code_attr_index: Option<usize>,
}

impl<'a> MethodInfo<'a> {
    pub fn code_attribute(&self) -> Option<&CodeAttribute<'a>> {
        if let Some(index) = self.code_attr_index {
            if let Some(attr) = self.attributes.get(index) {
                if let AttributeType::Code { code } = &attr.attribute_type {
                    return Some(code);
                }
            }
        }
        None
    }
}
