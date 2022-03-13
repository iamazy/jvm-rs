use crate::instructions::{InstructionExecutor, InstructionReader};
use crate::rtda::Frame;
use jvm_macros::NoOperand;
use std::io::Cursor;

#[derive(NoOperand)]
#[allow(non_camel_case_types)]
pub struct POP;

impl InstructionExecutor for POP {
    fn execute(&self, frame: &mut Frame) {
        frame.operand_stack().pop_slot();
    }
}

#[derive(NoOperand)]
#[allow(non_camel_case_types)]
pub struct POP2;

impl InstructionExecutor for POP2 {
    fn execute(&self, frame: &mut Frame) {
        frame.operand_stack().pop_slot();
        frame.operand_stack().pop_slot();
    }
}
