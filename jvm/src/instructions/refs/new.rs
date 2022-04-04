use crate::instructions::{InstructionExecutor, InstructionReader};
use crate::rtda::{Constant, Object};
use bytes::Buf;
use jvm_macros::Index8;

#[derive(Debug, Default, Index8)]
pub struct NEW {
    index: usize,
}

impl InstructionExecutor for NEW {
    fn execute(&self, frame: &mut crate::rtda::Frame) {
        unsafe {
            let mut constant_pool = frame.method().borrow().class.as_ref().constant_pool;
            if let Constant::Class(class_ref) = constant_pool.as_mut().get_mut(self.index) {
                let class = class_ref.resolved_class().unwrap();
                if class.as_ref().is_interface() || class.as_ref().is_abstract() {
                    panic!("java.lang.InstantiationError");
                }
                let mut object = Object::new(class);
                frame
                    .operand_stack_mut()
                    .push_ref(&mut object as *mut Object);
            }
        }
    }
}
