use crate::instructions::{InstructionExecutor, InstructionReader};
use crate::rtda::Frame;
use jvm_macros::NoOperand;
use std::io::Cursor;

#[derive(NoOperand)]
#[allow(non_camel_case_types)]
pub struct IOR;

impl InstructionExecutor for IOR {
    fn execute(&self, frame: &mut Frame) {
        let val2 = frame.operand_stack().pop_int();
        let val1 = frame.operand_stack().pop_int();
        frame.operand_stack().push_int(val1 | val2);
    }
}

#[derive(NoOperand)]
#[allow(non_camel_case_types)]
pub struct LOR;

impl InstructionExecutor for LOR {
    fn execute(&self, frame: &mut Frame) {
        let val2 = frame.operand_stack().pop_long();
        let val1 = frame.operand_stack().pop_long();
        frame.operand_stack().push_long(val1 | val2);
    }
}