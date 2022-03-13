use crate::instructions::{InstructionExecutor, InstructionReader};
use crate::rtda::Frame;
use bytes::Buf;
use jvm_macros::{Index8, NoOperand};
use std::io::Cursor;

fn istore(frame: &mut Frame, index: usize) {
    let val = frame.operand_stack().pop_double();
    frame.local_vars().set_double(index, val);
}

#[derive(Index8)]
#[allow(non_camel_case_types)]
pub struct ISTORE {
    index: u32,
}

impl ISTORE {
    #[inline]
    pub const fn new(index: u32) -> Self {
        Self { index }
    }
}

impl InstructionExecutor for ISTORE {
    fn execute(&self, frame: &mut Frame) {
        istore(frame, self.index as usize);
    }
}

#[derive(NoOperand)]
#[allow(non_camel_case_types)]
pub struct ISTORE_0;

impl InstructionExecutor for ISTORE_0 {
    fn execute(&self, frame: &mut Frame) {
        istore(frame, 0);
    }
}

#[derive(NoOperand)]
#[allow(non_camel_case_types)]
pub struct ISTORE_1;

impl InstructionExecutor for ISTORE_1 {
    fn execute(&self, frame: &mut Frame) {
        istore(frame, 1);
    }
}

#[derive(NoOperand)]
#[allow(non_camel_case_types)]
pub struct ISTORE_2;

impl InstructionExecutor for ISTORE_2 {
    fn execute(&self, frame: &mut Frame) {
        istore(frame, 2);
    }
}

#[derive(NoOperand)]
#[allow(non_camel_case_types)]
pub struct ISTORE_3;

impl InstructionExecutor for ISTORE_3 {
    fn execute(&self, frame: &mut Frame) {
        istore(frame, 3);
    }
}
