use crate::instructions::{InstructionExecutor, InstructionReader};
use crate::rtda::{Constant, Frame};
use bytes::Buf;
use jvm_macros::Index16;

#[derive(Debug, Default, Index16)]
#[allow(non_camel_case_types)]
pub struct PUT_STATIC {
    pub index: usize,
}

impl InstructionExecutor for PUT_STATIC {
    fn execute(&self, frame: &mut Frame) {
        let cur_method = frame.method();
        let mut cur_class = cur_method.borrow_mut().class;
        unsafe {
            let mut constant_pool = cur_class.as_mut().constant_pool;
            if let Constant::FieldRef(field_ref) = constant_pool.as_mut().get_mut(self.index) {
                let field = field_ref.resolve_field();
                let mut class = field.borrow_mut().class;
                // TODO: initialize class

                if !field.borrow().is_static() {
                    panic!("java.lang.IncompatibleClassChangeError");
                }
                if field.borrow().is_final()
                    && (cur_class != class || cur_method.borrow().name != "<clinit>")
                {
                    panic!("java.lang.IllegalAccessError");
                }

                let descriptor = field.borrow().descriptor.clone();
                let slot_id = field.borrow().slot_id;
                let slots = class.as_mut().static_vars_mut();
                if let Some(char) = descriptor.chars().next() {
                    match char {
                        'Z' | 'B' | 'C' | 'S' | 'I' => {
                            slots.set_int(slot_id, frame.operand_stack_mut().pop_int());
                        }
                        'F' => {
                            slots.set_float(slot_id, frame.operand_stack_mut().pop_float());
                        }
                        'J' => {
                            slots.set_long(slot_id, frame.operand_stack_mut().pop_long());
                        }
                        'D' => {
                            slots.set_double(slot_id, frame.operand_stack_mut().pop_double());
                        }
                        'L' | '[' => {
                            slots.set_ref(slot_id, frame.operand_stack_mut().pop_ref());
                        }
                        _ => panic!("java.lang.ClassFormatError"),
                    }
                }
            }
        }
    }
}
