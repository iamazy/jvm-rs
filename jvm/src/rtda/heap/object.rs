use crate::rtda::heap::class::Class;
use crate::rtda::Slot;
use std::marker::PhantomData;
use std::ptr::NonNull;

#[derive(Debug, Clone)]
pub struct Object {
    class: NonNull<Class>,
    fields: Vec<Slot>,
    marker: PhantomData<Box<Class>>,
}

impl Object {
    pub fn new(fields: usize) -> Self {
        Self {
            class: NonNull::dangling(),
            fields: Vec::with_capacity(fields),
            marker: PhantomData,
        }
    }
}
