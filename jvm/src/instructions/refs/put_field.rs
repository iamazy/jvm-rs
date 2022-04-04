use crate::instructions::{InstructionExecutor, InstructionReader};
use crate::rtda::{Constant, Frame};
use bytes::Buf;
use jvm_macros::Index16;

#[derive(Debug, Default, Index16)]
#[allow(non_camel_case_types)]
pub struct PUT_FIELD {
    pub index: usize,
}

impl InstructionExecutor for PUT_FIELD {
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
                if field.borrow().is_final()
                    && (cur_class != field.borrow().class || cur_method.borrow().name != "<init>")
                {
                    panic!("java.lang.IllegalAccessError");
                }
                let descriptor = field.borrow().descriptor.clone();
                let slot_id = field.borrow().slot_id;
                if let Some(char) = descriptor.chars().next() {
                    match char {
                        'Z' | 'B' | 'C' | 'S' | 'I' => {
                            let val = frame.operand_stack_mut().pop_int();
                            let object = frame.operand_stack_mut().pop_ref();
                            if object.is_null() {
                                panic!("java.lang.NullPointerException");
                            }
                            (*object).fields.set_int(slot_id, val)
                        }
                        'F' => {
                            let val = frame.operand_stack_mut().pop_float();
                            let object = frame.operand_stack_mut().pop_ref();
                            if object.is_null() {
                                panic!("java.lang.NullPointerException");
                            }
                            (*object).fields.set_float(slot_id, val)
                        }
                        'J' => {
                            let val = frame.operand_stack_mut().pop_long();
                            let object = frame.operand_stack_mut().pop_ref();
                            if object.is_null() {
                                panic!("java.lang.NullPointerException");
                            }
                            (*object).fields.set_long(slot_id, val)
                        }
                        'D' => {
                            let val = frame.operand_stack_mut().pop_double();
                            let object = frame.operand_stack_mut().pop_ref();
                            if object.is_null() {
                                panic!("java.lang.NullPointerException");
                            }
                            (*object).fields.set_double(slot_id, val)
                        }
                        'L' | '[' => {
                            let val = frame.operand_stack_mut().pop_ref();
                            let object = frame.operand_stack_mut().pop_ref();
                            if object.is_null() {
                                panic!("java.lang.NullPointerException");
                            }
                            (*object).fields.set_ref(slot_id, val)
                        }
                        _ => panic!("java.lang.ClassFormatError"),
                    }
                }
            }
        }
    }
}
