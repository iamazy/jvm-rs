use crate::rtda::heap::access_flags::AccessFlag;
use crate::rtda::heap::class::Class;
use classfile::{AttributeType, FieldInfo};
use std::ptr::NonNull;

#[derive(Debug, Clone)]
pub struct Field {
    pub access_flags: u16,
    pub name: String,
    pub descriptor: String,
    pub const_value_index: Option<u16>,
    pub slot_id: Option<u16>,
    pub class: NonNull<Class>,
}

impl Field {
    pub fn new(class: NonNull<Class>, field_info: &FieldInfo) -> Self {
        let class_ptr = unsafe { Box::from_raw(class.as_ptr()) };
        let name = class_ptr
            .constant_pool
            .get_str(field_info.name_index as usize);
        let descriptor = class_ptr
            .constant_pool
            .get_str(field_info.descriptor_index as usize);
        let mut const_value_index = None;
        for attr in &field_info.attributes {
            if let AttributeType::ConstantValue {
                constant_value_index,
            } = attr.attribute_type
            {
                const_value_index = Some(constant_value_index);
            }
        }
        Self {
            access_flags: field_info.access_flags,
            name,
            descriptor,
            const_value_index,
            slot_id: None,
            class,
        }
    }

    pub fn is_volatile(&self) -> bool {
        self.access_flags & AccessFlag::ACC_VOLATILE.bits() != 0
    }

    pub fn is_transient(&self) -> bool {
        self.access_flags & AccessFlag::ACC_TRANSIENT.bits() != 0
    }

    pub fn is_enum(&self) -> bool {
        self.access_flags & AccessFlag::ACC_ENUM.bits() != 0
    }

    pub fn is_long(&self) -> bool {
        self.descriptor == "J"
    }

    pub fn is_double(&self) -> bool {
        self.descriptor == "D"
    }
}

pub fn new_fields(class: NonNull<Class>, field_infos: &Vec<FieldInfo>) -> Vec<Field> {
    field_infos
        .iter()
        .map(|field_info| Field::new(class, field_info))
        .collect()
}
