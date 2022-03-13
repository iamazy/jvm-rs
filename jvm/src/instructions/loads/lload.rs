use crate::instructions::{InstructionExecutor, InstructionReader};
use crate::rtda::Frame;
use bytes::Buf;
use jvm_macros::{Index8, NoOperand};
use std::io::Cursor;

fn lload(frame: &mut Frame, index: usize) {
    let val = frame.local_vars().get_long(index);
    frame.operand_stack().push_long(val);
}

#[derive(Index8)]
pub struct LLOAD {
    index: u32,
}

impl LLOAD {
    #[inline]
    pub const fn new(index: u32) -> Self {
        Self { index }
    }
}

impl InstructionExecutor for LLOAD {
    fn execute(&self, frame: &mut Frame) {
        lload(frame, self.index as usize);
    }
}

#[derive(NoOperand)]
#[allow(non_camel_case_types)]
pub struct LLOAD_0;

impl InstructionExecutor for LLOAD_0 {
    fn execute(&self, frame: &mut Frame) {
        lload(frame, 0);
    }
}

#[derive(NoOperand)]
#[allow(non_camel_case_types)]
pub struct LLOAD_1;

impl InstructionExecutor for LLOAD_1 {
    fn execute(&self, frame: &mut Frame) {
        lload(frame, 1);
    }
}

#[derive(NoOperand)]
#[allow(non_camel_case_types)]
pub struct LLOAD_2;

impl InstructionExecutor for LLOAD_2 {
    fn execute(&self, frame: &mut Frame) {
        lload(frame, 2);
    }
}

#[derive(NoOperand)]
#[allow(non_camel_case_types)]
pub struct LLOAD_3;

impl InstructionExecutor for LLOAD_3 {
    fn execute(&self, frame: &mut Frame) {
        lload(frame, 3);
    }
}
