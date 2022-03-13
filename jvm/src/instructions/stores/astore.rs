use crate::instructions::{InstructionExecutor, InstructionReader};
use crate::rtda::Frame;
use bytes::Buf;
use jvm_macros::{Index8, NoOperand};
use std::io::Cursor;

fn astore(frame: &mut Frame, index: usize) {
    let val = frame.operand_stack().pop_ref();
    frame.local_vars().set_ref(index, val);
}

#[derive(Index8)]
#[allow(non_camel_case_types)]
pub struct ASTORE {
    index: u32,
}

impl ASTORE {
    #[inline]
    pub fn new(index: u32) -> ASTORE {
        ASTORE { index }
    }
}

impl InstructionExecutor for ASTORE {
    fn execute(&self, frame: &mut Frame) {
        astore(frame, self.index as usize);
    }
}

#[derive(NoOperand)]
#[allow(non_camel_case_types)]
pub struct ASTORE_0;

impl InstructionExecutor for ASTORE_0 {
    fn execute(&self, frame: &mut Frame) {
        astore(frame, 0);
    }
}

#[derive(NoOperand)]
#[allow(non_camel_case_types)]
pub struct ASTORE_1;

impl InstructionExecutor for ASTORE_1 {
    fn execute(&self, frame: &mut Frame) {
        astore(frame, 1);
    }
}

#[derive(NoOperand)]
#[allow(non_camel_case_types)]
pub struct ASTORE_2;

impl InstructionExecutor for ASTORE_2 {
    fn execute(&self, frame: &mut Frame) {
        astore(frame, 2);
    }
}

#[derive(NoOperand)]
#[allow(non_camel_case_types)]
pub struct ASTORE_3;

impl InstructionExecutor for ASTORE_3 {
    fn execute(&self, frame: &mut Frame) {
        astore(frame, 3);
    }
}
