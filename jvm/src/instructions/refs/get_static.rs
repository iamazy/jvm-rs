use crate::instructions::{InstructionExecutor, InstructionReader};
use crate::rtda::{Constant, Frame};
use bytes::Buf;
use jvm_macros::Index16;

#[derive(Debug, Default, Index16)]
#[allow(non_camel_case_types)]
pub struct GET_STATIC {
    pub index: usize,
}

impl InstructionExecutor for GET_STATIC {
    fn execute(&self, frame: &mut Frame) {
        unsafe {
            let mut constant_pool = frame.method().borrow_mut().class.as_mut().constant_pool;
            if let Constant::FieldRef(field_ref) = constant_pool.as_mut().get_mut(self.index) {
                let field = field_ref.resolve_field();
                let mut class = field.borrow_mut().class;
                if !field.borrow().is_static() {
                    panic!("java.lang.IncompatibleClassChangeError");
                }
                let descriptor = field.borrow().descriptor.clone();
                let slot_id = field.borrow().slot_id;
                let slots = class.as_mut().static_vars_mut();
                if let Some(char) = descriptor.chars().next() {
                    match char {
                        'Z' | 'B' | 'C' | 'S' | 'I' => {
                            frame.operand_stack_mut().push_int(slots.get_int(slot_id));
                        }
                        'F' => {
                            frame
                                .operand_stack_mut()
                                .push_float(slots.get_float(slot_id));
                        }
                        'J' => {
                            frame.operand_stack_mut().push_long(slots.get_long(slot_id));
                        }
                        'D' => {
                            frame
                                .operand_stack_mut()
                                .push_double(slots.get_double(slot_id));
                        }
                        'L' | '[' => {
                            frame.operand_stack_mut().push_ref(slots.get_ref(slot_id));
                        }
                        _ => {
                            panic!("java.lang.ClassFormatError");
                        }
                    }
                }
            }
        }
    }
}
