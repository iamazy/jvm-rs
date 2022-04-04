use crate::rtda::heap::class::Class;
use crate::rtda::LocalVars;
use std::marker::PhantomData;
use std::ptr::NonNull;

#[derive(Debug, Clone)]
pub struct Object {
    pub(crate) class: NonNull<Class>,
    pub(crate) fields: LocalVars,
    pub(crate) marker: PhantomData<Box<Class>>,
}

impl Object {
    pub fn new(class: NonNull<Class>) -> Self {
        unsafe {
            Self {
                class,
                fields: LocalVars(Vec::with_capacity(class.as_ref().instance_slot_count)),
                marker: PhantomData,
            }
        }
    }

    pub fn is_instance_of(&self, class: NonNull<Class>) -> bool {
        unsafe { class.as_ref().is_assignable_from(self.class) }
    }
}
