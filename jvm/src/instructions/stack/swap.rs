use crate::instructions::{InstructionExecutor, InstructionReader};
use crate::rtda::Frame;
use jvm_macros::NoOperand;
use std::io::Cursor;

#[derive(NoOperand, Debug)]
pub struct SWAP;

impl InstructionExecutor for SWAP {
    fn execute(&self, frame: &mut Frame) {
        let slot1 = frame.operand_stack_mut().pop_slot();
        let slot2 = frame.operand_stack_mut().pop_slot();
        frame.operand_stack_mut().push_slot(slot1);
        frame.operand_stack_mut().push_slot(slot2);
    }
}
