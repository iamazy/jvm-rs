use crate::instructions::{InstructionExecutor, InstructionReader};
use crate::rtda::Frame;
use bytes::Buf;
use jvm_macros::{Index8, NoOperand};
use std::io::Cursor;

fn dstore(frame: &mut Frame, index: usize) {
    let val = frame.operand_stack().pop_double();
    frame.local_vars().set_double(index, val);
}

#[derive(Index8)]
#[allow(non_camel_case_types)]
pub struct DSTORE {
    index: u32,
}

impl DSTORE {
    #[inline]
    pub fn new(index: u32) -> DSTORE {
        DSTORE { index }
    }
}

impl InstructionExecutor for DSTORE {
    fn execute(&self, frame: &mut Frame) {
        dstore(frame, self.index as usize);
    }
}

#[derive(NoOperand)]
#[allow(non_camel_case_types)]
pub struct DSTORE_0;

impl InstructionExecutor for DSTORE_0 {
    fn execute(&self, frame: &mut Frame) {
        dstore(frame, 0);
    }
}

#[derive(NoOperand)]
#[allow(non_camel_case_types)]
pub struct DSTORE_1;

impl InstructionExecutor for DSTORE_1 {
    fn execute(&self, frame: &mut Frame) {
        dstore(frame, 1);
    }
}

#[derive(NoOperand)]
#[allow(non_camel_case_types)]
pub struct DSTORE_2;

impl InstructionExecutor for DSTORE_2 {
    fn execute(&self, frame: &mut Frame) {
        dstore(frame, 2);
    }
}

#[derive(NoOperand)]
#[allow(non_camel_case_types)]
pub struct DSTORE_3;

impl InstructionExecutor for DSTORE_3 {
    fn execute(&self, frame: &mut Frame) {
        dstore(frame, 3);
    }
}
