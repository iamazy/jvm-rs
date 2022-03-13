use crate::instructions::{InstructionExecutor, InstructionReader};
use crate::rtda::Frame;
use jvm_macros::NoOperand;
use std::io::Cursor;

fn fcmp(frame: &mut Frame, flag: bool) {
    let v2 = frame.operand_stack().pop_float();
    let v1 = frame.operand_stack().pop_float();
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
pub struct FCMPG;

impl InstructionExecutor for FCMPG {
    fn execute(&self, frame: &mut Frame) {
        fcmp(frame, true);
    }
}

#[derive(NoOperand)]
#[allow(non_camel_case_types)]
pub struct FCMPL;

impl InstructionExecutor for FCMPL {
    fn execute(&self, frame: &mut Frame) {
        fcmp(frame, false);
    }
}
