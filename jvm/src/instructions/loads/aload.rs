use crate::instructions::{InstructionExecutor, InstructionReader};
use crate::rtda::Frame;
use bytes::Buf;
use jvm_macros::{Index8, NoOperand};
use std::io::Cursor;

fn aload(frame: &mut Frame, index: usize) {
    let val = frame.local_vars().get_ref(index);
    frame.operand_stack().push_ref(val);
}

#[derive(Index8)]
#[allow(non_camel_case_types)]
pub struct ALOAD {
    index: u32,
}

impl ALOAD {
    #[inline]
    pub const fn new(index: u32) -> Self {
        Self { index }
    }
}

impl InstructionExecutor for ALOAD {
    fn execute(&self, frame: &mut Frame) {
        aload(frame, self.index as usize)
    }
}

#[derive(NoOperand)]
#[allow(non_camel_case_types)]
pub struct ALOAD_0;

impl InstructionExecutor for ALOAD_0 {
    fn execute(&self, frame: &mut Frame) {
        aload(frame, 0)
    }
}

#[derive(NoOperand)]
#[allow(non_camel_case_types)]
pub struct ALOAD_1;

impl InstructionExecutor for ALOAD_1 {
    fn execute(&self, frame: &mut Frame) {
        aload(frame, 1)
    }
}

#[derive(NoOperand)]
#[allow(non_camel_case_types)]
pub struct ALOAD_2;

impl InstructionExecutor for ALOAD_2 {
    fn execute(&self, frame: &mut Frame) {
        aload(frame, 2)
    }
}

#[derive(NoOperand)]
#[allow(non_camel_case_types)]
pub struct ALOAD_3;

impl InstructionExecutor for ALOAD_3 {
    fn execute(&self, frame: &mut Frame) {
        aload(frame, 3)
    }
}
