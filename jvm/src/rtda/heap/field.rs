use crate::rtda::heap::access_flags::AccessFlag;
use crate::rtda::heap::class::Class;
use classfile::{AttributeType, FieldInfo};
use std::marker::{PhantomData, PhantomPinned};
use std::ptr::NonNull;

#[derive(Debug)]
pub struct Field {
    pub access_flags: u16,
    pub name_index: u16,
    pub descriptor_index: u16,
    pub const_value_index: Option<u16>,
    pub slot_id: Option<u16>,
    pub class: NonNull<Class>,
    marker: PhantomData<Box<Class>>,
    _pin: PhantomPinned,
}

impl Field {
    pub fn new(class: &mut Class, field_info: &FieldInfo) -> Self {
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
            name_index: field_info.name_index,
            descriptor_index: field_info.descriptor_index,
            const_value_index,
            slot_id: None,
            class: NonNull::from(class),
            marker: PhantomData,
            _pin: PhantomPinned,
        }
    }

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
        self.descriptor() == "J"
    }

    pub fn is_double(&self) -> bool {
        self.descriptor() == "D"
    }

    pub fn name(&self) -> &String {
        unsafe {
            self.class
                .as_ref()
                .constant_pool
                .as_ref()
                .get_str(self.name_index as usize)
        }
    }

    pub fn descriptor(&self) -> &String {
        unsafe {
            self.class
                .as_ref()
                .constant_pool
                .as_ref()
                .get_str(self.descriptor_index as usize)
        }
    }
}

pub fn new_fields(class: &mut Class, field_infos: &[FieldInfo]) -> Vec<Field> {
    field_infos
        .iter()
        .map(|field_info| Field::new(class, field_info))
        .collect()
}
