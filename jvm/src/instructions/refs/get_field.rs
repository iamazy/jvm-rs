use crate::instructions::{InstructionExecutor, InstructionReader};
use crate::rtda::{Constant, Frame};
use bytes::Buf;
use jvm_macros::Index16;

#[derive(Debug, Default, Index16)]
#[allow(non_camel_case_types)]
pub struct GET_FIELD {
    pub index: usize,
}

impl InstructionExecutor for GET_FIELD {
    fn execute(&self, frame: &mut Frame) {
        let cur_method = frame.method();
        let mut cur_class = cur_method.borrow().class;
        unsafe {
            let mut constant_pool = cur_class.as_mut().constant_pool;
            if let Constant::FieldRef(field_ref) = constant_pool.as_mut().get_mut(self.index) {
                let field = field_ref.resolve_field();
                if field.borrow().is_static() {
                    panic!("java.lang.IncompatibleClassChangeError");
                }
                let object = frame.operand_stack_mut().pop_ref();
                if object.is_null() {
                    panic!("java.lang.NullPointerException");
                }
                let descriptor = field.borrow().descriptor.clone();
                let slot_id = field.borrow().slot_id;
                if let Some(char) = descriptor.chars().next() {
                    match char {
                        'Z' | 'B' | 'C' | 'S' | 'I' => {
                            frame
                                .operand_stack_mut()
                                .push_int((*object).fields.get_int(slot_id));
                        }
                        'F' => {
                            frame
                                .operand_stack_mut()
                                .push_float((*object).fields.get_float(slot_id));
                        }
                        'J' => {
                            frame
                                .operand_stack_mut()
                                .push_long((*object).fields.get_long(slot_id));
                        }
                        'D' => {
                            frame
                                .operand_stack_mut()
                                .push_double((*object).fields.get_double(slot_id));
                        }
                        'L' | '[' => {
                            frame
                                .operand_stack_mut()
                                .push_ref((*object).fields.get_ref(slot_id));
                        }
                        _ => {
                            panic!("java.lang.IllegalAccessError");
                        }
                    }
                }
            }
        }
    }
}
