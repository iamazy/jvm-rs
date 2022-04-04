use crate::instructions::{InstructionExecutor, InstructionReader};
use crate::rtda::{Constant, Frame};
use bytes::Buf;
use jvm_macros::Index16;

#[derive(Debug, Default, Index16)]
#[allow(non_camel_case_types)]
pub struct CHECK_CAST {
    pub index: usize,
}

impl InstructionExecutor for CHECK_CAST {
    fn execute(&self, frame: &mut Frame) {
        let object = frame.operand_stack_mut().pop_ref();
        frame.operand_stack_mut().push_ref(object);
        if object.is_null() {
            return;
        }
        unsafe {
            let mut constant_pool = frame.method().borrow_mut().class.as_mut().constant_pool;
            if let Constant::Class(class_ref) = constant_pool.as_mut().get_mut(self.index) {
                let class = class_ref.resolved_class().unwrap();
                if !(*object).is_instance_of(class) {
                    panic!("java.lang.ClassCastException");
                }
            }
        }
    }
}
