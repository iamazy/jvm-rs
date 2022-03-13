use crate::instructions::{InstructionExecutor, InstructionReader};
use crate::rtda::Frame;
use bytes::Buf;
use jvm_macros::{Index8, NoOperand};
use std::io::Cursor;

fn iload(frame: &mut Frame, index: usize) {
    let val = frame.local_vars().get_int(index);
    frame.operand_stack().push_int(val);
}

#[derive(Index8)]
pub struct ILOAD {
    index: u32,
}

impl ILOAD {
    #[inline]
    pub const fn new(index: u32) -> Self {
        Self { index }
    }
}

impl InstructionExecutor for ILOAD {
    fn execute(&self, frame: &mut Frame) {
        iload(frame, self.index as usize);
    }
}

#[derive(NoOperand)]
#[allow(non_camel_case_types)]
pub struct ILOAD_0;

impl InstructionExecutor for ILOAD_0 {
    fn execute(&self, frame: &mut Frame) {
        iload(frame, 0);
    }
}

#[derive(NoOperand)]
#[allow(non_camel_case_types)]
pub struct ILOAD_1;

impl InstructionExecutor for ILOAD_1 {
    fn execute(&self, frame: &mut Frame) {
        iload(frame, 1);
    }
}

#[derive(NoOperand)]
#[allow(non_camel_case_types)]
pub struct ILOAD_2;

impl InstructionExecutor for ILOAD_2 {
    fn execute(&self, frame: &mut Frame) {
        iload(frame, 2);
    }
}

#[derive(NoOperand)]
#[allow(non_camel_case_types)]
pub struct ILOAD_3;

impl InstructionExecutor for ILOAD_3 {
    fn execute(&self, frame: &mut Frame) {
        iload(frame, 3);
    }
}
