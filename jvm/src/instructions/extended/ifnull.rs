use crate::instructions::{InstructionExecutor, InstructionReader};
use crate::rtda::Frame;
use bytes::Buf;
use jvm_macros::Branch;
use std::io::Cursor;

#[derive(Branch)]
#[allow(non_camel_case_types)]
pub struct IFNULL {
    offset: i32,
}

impl InstructionExecutor for IFNULL {
    fn execute(&self, frame: &mut Frame) {
        let r#ref = frame.operand_stack().pop_ref();
        if r#ref.is_null() {
            frame.branch(self.offset);
        }
    }
}

#[derive(Branch)]
#[allow(non_camel_case_types)]
pub struct IFNONNULL {
    offset: i32,
}

impl InstructionExecutor for IFNONNULL {
    fn execute(&self, frame: &mut Frame) {
        let r#ref = frame.operand_stack().pop_ref();
        if !r#ref.is_null() {
            frame.branch(self.offset);
        }
    }
}
