use crate::instructions::{InstructionExecutor, InstructionReader};
use crate::rtda::{Constant, Frame};
use bytes::Buf;
use jvm_macros::Index16;

#[derive(Debug, Default, Index16)]
#[allow(non_camel_case_types)]
pub struct INSTANCE_OF {
    pub index: usize,
}

impl InstructionExecutor for INSTANCE_OF {
    fn execute(&self, frame: &mut Frame) {
        let object = frame.operand_stack_mut().pop_ref();
        unsafe {
            if object.is_null() {
                frame.operand_stack_mut().push_int(0);
                return;
            }

            let mut constant_pool = frame.method().borrow().class.as_ref().constant_pool;
            if let Constant::Class(class_ref) = constant_pool.as_mut().get_mut(self.index) {
                let class = class_ref.resolved_class().unwrap();
                if (*object).is_instance_of(class) {
                    frame.operand_stack_mut().push_int(1);
                } else {
                    frame.operand_stack_mut().push_int(0);
                }
            }
        }
    }
}
