use crate::instructions::{InstructionExecutor, InstructionReader};
use crate::rtda::Frame;
use jvm_macros::NoOperand;

#[derive(NoOperand, Debug)]
#[allow(non_camel_case_types)]
pub struct POP;

impl InstructionExecutor for POP {
    fn execute(&self, frame: &mut Frame) {
        frame.operand_stack_mut().pop_slot();
    }
}

#[derive(NoOperand, Debug)]
#[allow(non_camel_case_types)]
pub struct POP2;

impl InstructionExecutor for POP2 {
    fn execute(&self, frame: &mut Frame) {
        frame.operand_stack_mut().pop_slot();
        frame.operand_stack_mut().pop_slot();
    }
}
