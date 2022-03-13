use crate::instructions::{InstructionExecutor, InstructionReader};
use crate::rtda::Frame;
use bytes::Buf;
use jvm_macros::{Index8, NoOperand};
use std::io::Cursor;

fn lstore(frame: &mut Frame, index: usize) {
    let val = frame.operand_stack().pop_long();
    frame.local_vars().set_long(index, val);
}

#[derive(Index8)]
#[allow(non_camel_case_types)]
pub struct LSTORE {
    index: u32,
}

impl LSTORE {
    #[inline]
    pub fn new(index: u32) -> LSTORE {
        LSTORE { index }
    }
}

impl InstructionExecutor for LSTORE {
    fn execute(&self, frame: &mut Frame) {
        lstore(frame, self.index as usize);
    }
}

#[derive(NoOperand)]
#[allow(non_camel_case_types)]
pub struct LSTORE_0;

impl InstructionExecutor for LSTORE_0 {
    fn execute(&self, frame: &mut Frame) {
        lstore(frame, 0);
    }
}

#[derive(NoOperand)]
#[allow(non_camel_case_types)]
pub struct LSTORE_1;

impl InstructionExecutor for LSTORE_1 {
    fn execute(&self, frame: &mut Frame) {
        lstore(frame, 1);
    }
}

#[derive(NoOperand)]
#[allow(non_camel_case_types)]
pub struct LSTORE_2;

impl InstructionExecutor for LSTORE_2 {
    fn execute(&self, frame: &mut Frame) {
        lstore(frame, 2);
    }
}

#[derive(NoOperand)]
#[allow(non_camel_case_types)]
pub struct LSTORE_3;

impl InstructionExecutor for LSTORE_3 {
    fn execute(&self, frame: &mut Frame) {
        lstore(frame, 3);
    }
}
