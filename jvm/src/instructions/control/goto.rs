use crate::instructions::{InstructionExecutor, InstructionReader};
use crate::rtda::Frame;
use bytes::Buf;
use jvm_macros::Branch;

#[derive(Branch, Default, Debug)]
#[allow(non_camel_case_types)]
pub struct GOTO {
    offset: i32,
}

impl InstructionExecutor for GOTO {
    fn execute(&self, frame: &mut Frame) {
        frame.branch(self.offset);
    }
}
