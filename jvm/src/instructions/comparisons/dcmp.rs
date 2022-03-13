use crate::instructions::{InstructionExecutor, InstructionReader};
use crate::rtda::Frame;
use jvm_macros::NoOperand;
use std::io::Cursor;

fn dcmp(frame: &mut Frame, flag: bool) {
    let v2 = frame.operand_stack().pop_double();
    let v1 = frame.operand_stack().pop_double();
    let value = if v1 > v2 {
        1
    } else if v1 == v2 {
        0
    } else if v1 < v2 {
        -1
    } else if flag {
        1
    } else {
        -1
    };
    frame.operand_stack().push_int(value);
}

#[derive(NoOperand)]
#[allow(non_camel_case_types)]
pub struct DCMPG;

impl InstructionExecutor for DCMPG {
    fn execute(&self, frame: &mut Frame) {
        dcmp(frame, true);
    }
}

#[derive(NoOperand)]
#[allow(non_camel_case_types)]
pub struct DCMPL;

impl InstructionExecutor for DCMPL {
    fn execute(&self, frame: &mut Frame) {
        dcmp(frame, false);
    }
}
