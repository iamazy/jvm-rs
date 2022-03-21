use crate::instructions::comparisons::fcmp;
use crate::instructions::{InstructionExecutor, InstructionReader};
use crate::rtda::Frame;
use jvm_macros::NoOperand;
use std::io::Cursor;

#[derive(NoOperand, Debug)]
#[allow(non_camel_case_types)]
pub struct FCMPG;

impl InstructionExecutor for FCMPG {
    fn execute(&self, frame: &mut Frame) {
        fcmp(frame, true);
    }
}

#[derive(NoOperand, Debug)]
#[allow(non_camel_case_types)]
pub struct FCMPL;

impl InstructionExecutor for FCMPL {
    fn execute(&self, frame: &mut Frame) {
        fcmp(frame, false);
    }
}
