use crate::instructions::{InstructionExecutor, InstructionReader};
use crate::rtda::Frame;
use bytes::Buf;
use jvm_macros::{Index8, NoOperand};
use std::io::Cursor;

fn fload(frame: &mut Frame, index: usize) {
    let val = frame.local_vars().get_float(index);
    frame.operand_stack().push_float(val);
}

#[derive(Index8)]
pub struct FLOAD {
    index: u32,
}

impl FLOAD {
    #[inline]
    pub const fn new(index: u32) -> Self {
        Self { index }
    }
}

impl InstructionExecutor for FLOAD {
    fn execute(&self, frame: &mut Frame) {
        fload(frame, self.index as usize);
    }
}

#[derive(NoOperand)]
#[allow(non_camel_case_types)]
pub struct FLOAD_0;

impl InstructionExecutor for FLOAD_0 {
    fn execute(&self, frame: &mut Frame) {
        fload(frame, 0);
    }
}

#[derive(NoOperand)]
#[allow(non_camel_case_types)]
pub struct FLOAD_1;

impl InstructionExecutor for FLOAD_1 {
    fn execute(&self, frame: &mut Frame) {
        fload(frame, 1);
    }
}

#[derive(NoOperand)]
#[allow(non_camel_case_types)]
pub struct FLOAD_2;

impl InstructionExecutor for FLOAD_2 {
    fn execute(&self, frame: &mut Frame) {
        fload(frame, 2);
    }
}

#[derive(NoOperand)]
#[allow(non_camel_case_types)]
pub struct FLOAD_3;

impl InstructionExecutor for FLOAD_3 {
    fn execute(&self, frame: &mut Frame) {
        fload(frame, 3);
    }
}
