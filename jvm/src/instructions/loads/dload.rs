use crate::instructions::{InstructionExecutor, InstructionReader};
use crate::rtda::Frame;
use bytes::Buf;
use jvm_macros::{Index8, NoOperand};
use std::io::Cursor;

fn dload(frame: &mut Frame, index: usize) {
    let val = frame.local_vars().get_double(index);
    frame.operand_stack().push_double(val);
}

#[derive(Index8)]
pub struct DLOAD {
    index: u32,
}

impl DLOAD {
    #[inline]
    pub const fn new(index: u32) -> Self {
        Self { index }
    }
}

impl InstructionExecutor for DLOAD {
    fn execute(&self, frame: &mut Frame) {
        dload(frame, self.index as usize);
    }
}

#[derive(NoOperand)]
#[allow(non_camel_case_types)]
pub struct DLOAD_0;

impl InstructionExecutor for DLOAD_0 {
    fn execute(&self, frame: &mut Frame) {
        dload(frame, 0);
    }
}

#[derive(NoOperand)]
#[allow(non_camel_case_types)]
pub struct DLOAD_1;

impl InstructionExecutor for DLOAD_1 {
    fn execute(&self, frame: &mut Frame) {
        dload(frame, 1);
    }
}

#[derive(NoOperand)]
#[allow(non_camel_case_types)]
pub struct DLOAD_2;

impl InstructionExecutor for DLOAD_2 {
    fn execute(&self, frame: &mut Frame) {
        dload(frame, 2);
    }
}

#[derive(NoOperand)]
#[allow(non_camel_case_types)]
pub struct DLOAD_3;

impl InstructionExecutor for DLOAD_3 {
    fn execute(&self, frame: &mut Frame) {
        dload(frame, 3);
    }
}
