use crate::instructions::{InstructionExecutor, InstructionReader};
use crate::rtda::Frame;
use jvm_macros::NoOperand;
use std::cmp::Ordering;

#[derive(NoOperand, Debug)]
#[allow(non_camel_case_types)]
pub struct LCMP;

impl InstructionExecutor for LCMP {
    fn execute(&self, frame: &mut Frame) {
        let val2 = frame.operand_stack_mut().pop_long();
        let val1 = frame.operand_stack_mut().pop_long();
        match val1.cmp(&val2) {
            Ordering::Greater => frame.operand_stack_mut().push_int(1),
            Ordering::Equal => frame.operand_stack_mut().push_int(0),
            _ => frame.operand_stack_mut().push_int(-1),
        }
    }
}
