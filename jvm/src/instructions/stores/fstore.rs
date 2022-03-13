use crate::instructions::{InstructionExecutor, InstructionReader};
use crate::rtda::Frame;
use bytes::Buf;
use jvm_macros::{Index8, NoOperand};
use std::io::Cursor;

fn fstore(frame: &mut Frame, index: usize) {
    let val = frame.operand_stack().pop_double();
    frame.local_vars().set_double(index, val);
}

#[derive(Index8)]
#[allow(non_camel_case_types)]
pub struct FSTORE {
    index: u32,
}

impl FSTORE {
    #[inline]
    pub fn new(index: u32) -> FSTORE {
        FSTORE { index }
    }
}

impl InstructionExecutor for FSTORE {
    fn execute(&self, frame: &mut Frame) {
        fstore(frame, self.index as usize);
    }
}

#[derive(NoOperand)]
#[allow(non_camel_case_types)]
pub struct FSTORE_0;

impl InstructionExecutor for FSTORE_0 {
    fn execute(&self, frame: &mut Frame) {
        fstore(frame, 0);
    }
}

#[derive(NoOperand)]
#[allow(non_camel_case_types)]
pub struct FSTORE_1;

impl InstructionExecutor for FSTORE_1 {
    fn execute(&self, frame: &mut Frame) {
        fstore(frame, 1);
    }
}

#[derive(NoOperand)]
#[allow(non_camel_case_types)]
pub struct FSTORE_2;

impl InstructionExecutor for FSTORE_2 {
    fn execute(&self, frame: &mut Frame) {
        fstore(frame, 2);
    }
}

#[derive(NoOperand)]
#[allow(non_camel_case_types)]
pub struct FSTORE_3;

impl InstructionExecutor for FSTORE_3 {
    fn execute(&self, frame: &mut Frame) {
        fstore(frame, 3);
    }
}
