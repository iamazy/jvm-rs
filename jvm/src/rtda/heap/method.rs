use crate::rtda::heap::access_flags::AccessFlag;
use crate::rtda::heap::class::Class;
use classfile::MethodInfo;
use std::marker::PhantomData;
use std::ptr::NonNull;

#[derive(Debug)]
pub struct Method {
    pub access_flags: u16,
    pub name: String,
    pub descriptor: String,
    pub class: NonNull<Class>,
    max_stack: usize,
    max_locals: usize,
    code: Option<Vec<u8>>,
    marker: PhantomData<Box<Class>>,
}

impl Method {
    pub fn new(class: &mut Class, method_info: &classfile::MethodInfo) -> Self {
        let name = unsafe {
            class
                .constant_pool
                .as_ref()
                .get_utf8(method_info.name_index as usize)
        };
        let descriptor = unsafe {
            class
                .constant_pool
                .as_ref()
                .get_utf8(method_info.descriptor_index as usize)
        };
        let mut method = Method {
            access_flags: method_info.access_flags,
            name: String::from_utf8(name).unwrap(),
            descriptor: String::from_utf8(descriptor).unwrap(),
            class: NonNull::from(class),
            max_stack: 0,
            max_locals: 0,
            code: None,
            marker: PhantomData,
        };
        if let Some(code) = method_info.code_attribute() {
            method.max_stack = code.max_stack as usize;
            method.max_locals = code.max_locals as usize;
            method.code = Some(code.code.to_vec());
        }
        method
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

    pub fn is_static(&self) -> bool {
        self.access_flags & AccessFlag::ACC_STATIC.bits() != 0
    }

    pub fn is_final(&self) -> bool {
        self.access_flags & AccessFlag::ACC_FINAL.bits() != 0
    }

    pub fn is_synchronized(&self) -> bool {
        self.access_flags & AccessFlag::ACC_SYNCHRONIZED.bits() != 0
    }

    pub fn is_bridge(&self) -> bool {
        self.access_flags & AccessFlag::ACC_BRIDGE.bits() != 0
    }

    pub fn is_varargs(&self) -> bool {
        self.access_flags & AccessFlag::ACC_VARARGS.bits() != 0
    }

    pub fn is_native(&self) -> bool {
        self.access_flags & AccessFlag::ACC_NATIVE.bits() != 0
    }

    pub fn is_abstract(&self) -> bool {
        self.access_flags & AccessFlag::ACC_ABSTRACT.bits() != 0
    }

    pub fn is_strict(&self) -> bool {
        self.access_flags & AccessFlag::ACC_STRICT.bits() != 0
    }

    pub fn is_accessible_to(&self, class: &Class) -> bool {
        if self.is_public() {
            return true;
        }
        unsafe {
            let this = self.class.as_ref();
            if self.is_protected() {
                return this.name == class.name
                    || class.is_sub_class_of(this)
                    || this.package_name() == class.package_name();
            }
            if !self.is_private() {
                return this.package_name() == class.package_name();
            }
            this.name == class.name
        }
    }

    pub fn max_locals(&self) -> usize {
        self.max_locals
    }

    pub fn max_stack(&self) -> usize {
        self.max_stack
    }

    pub fn code(&self) -> Option<&[u8]> {
        self.code.as_deref()
    }
}

pub fn new_methods(class: &mut Class, method_infos: &[MethodInfo]) -> Vec<Method> {
    method_infos
        .iter()
        .map(|method_info| Method::new(class, method_info))
        .collect()
}
