use crate::instructions::comparisons::dcmp;
use crate::instructions::{InstructionExecutor, InstructionReader};
use crate::rtda::Frame;
use jvm_macros::NoOperand;

#[derive(NoOperand, Debug)]
#[allow(non_camel_case_types)]
pub struct DCMPG;

impl InstructionExecutor for DCMPG {
    fn execute(&self, frame: &mut Frame) {
        dcmp(frame, true);
    }
}

#[derive(NoOperand, Debug)]
#[allow(non_camel_case_types)]
pub struct DCMPL;

impl InstructionExecutor for DCMPL {
    fn execute(&self, frame: &mut Frame) {
        dcmp(frame, false);
    }
}
