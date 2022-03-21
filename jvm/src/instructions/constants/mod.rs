use crate::instructions::{InstructionExecutor, InstructionReader};
use crate::rtda::Frame;
use jvm_macros::NoOperand;
use std::io::Cursor;

pub(crate) mod r#const;
pub(crate) mod ipush;

#[derive(NoOperand, Debug)]
pub struct NOP;

impl InstructionExecutor for NOP {
    fn execute(&self, _frame: &mut Frame) {
        // do nothing
    }
}
