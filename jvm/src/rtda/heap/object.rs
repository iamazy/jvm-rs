use crate::rtda::heap::class::Class;
use crate::rtda::Slot;
use std::marker::PhantomData;
use std::ptr::NonNull;

#[derive(Debug, Clone)]
pub struct Object {
    pub(crate) class: NonNull<Class>,
    pub(crate) fields: Vec<Slot>,
    pub(crate) marker: PhantomData<Box<Class>>,
}

impl Object {
    pub fn new(class: NonNull<Class>) -> Self {
        unsafe {
            Self {
                class,
                fields: Vec::with_capacity(class.as_ref().instance_slot_count),
                marker: PhantomData,
            }
        }
    }
}
