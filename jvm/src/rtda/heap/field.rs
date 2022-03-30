use crate::rtda::heap::access_flags::AccessFlag;
use crate::rtda::heap::class::Class;
use classfile::{AttributeType, FieldInfo};
use std::marker::PhantomData;
use std::ptr::NonNull;

#[derive(Debug)]
pub struct Field {
    pub access_flags: u16,
    pub name: String,
    pub descriptor: String,
    pub const_value_index: u16,
    pub slot_id: usize,
    pub class: NonNull<Class>,
    marker: PhantomData<Box<Class>>,
}

impl Field {
    pub fn new(class: &mut Class, field_info: &FieldInfo) -> Self {
        let mut const_value_index = 0;
        for attr in &field_info.attributes {
            if let AttributeType::ConstantValue {
                constant_value_index,
            } = attr.attribute_type
            {
                const_value_index = constant_value_index;
            }
        }
        let name = unsafe {
            class
                .constant_pool
                .as_ref()
                .get_utf8(field_info.name_index as usize)
        };
        let descriptor = unsafe {
            class
                .constant_pool
                .as_ref()
                .get_utf8(field_info.descriptor_index as usize)
        };
        Self {
            access_flags: field_info.access_flags,
            name: String::from_utf8(name).unwrap(),
            descriptor: String::from_utf8(descriptor).unwrap(),
            const_value_index,
            slot_id: 0,
            class: NonNull::from(class),
            marker: PhantomData,
        }
    }

    pub fn is_public(&self) -> bool {
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

    pub fn is_static(&self) -> bool {
        self.access_flags & AccessFlag::ACC_STATIC.bits() != 0
    }

    pub fn is_long(&self) -> bool {
        self.descriptor == "J"
    }

    pub fn is_double(&self) -> bool {
        self.descriptor == "D"
    }

    pub fn is_accessible_to(&self, class: &Class) -> bool {
        if self.is_public() {
            return true;
        }
        unsafe {
            let this = self.class.as_ref();
            if self.is_protected() {
                return this.name == class.name
                    || class.is_sub_class_of(NonNull::from(this))
                    || this.package_name() == class.package_name();
            }
            if !self.is_private() {
                return this.package_name() == class.package_name();
            }
            this.name == class.name
        }
    }
}

pub fn new_fields(class: &mut Class, field_infos: &[FieldInfo]) -> Vec<Field> {
    field_infos
        .iter()
        .map(|field_info| Field::new(class, field_info))
        .collect()
}
