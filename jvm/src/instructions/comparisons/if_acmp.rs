use crate::instructions::{InstructionExecutor, InstructionReader};
use crate::rtda::Frame;
use bytes::Buf;
use jvm_macros::Branch;
use std::io::Cursor;

#[derive(Branch)]
#[allow(non_camel_case_types)]
pub struct IF_ACMPEQ {
    offset: i32,
}

impl InstructionExecutor for IF_ACMPEQ {
    fn execute(&self, frame: &mut Frame) {
        let ref2 = frame.operand_stack().pop_ref();
        let ref1 = frame.operand_stack().pop_ref();
        if ref2 == ref1 {
            frame.branch(self.offset);
        }
    }
}

#[derive(Branch)]
#[allow(non_camel_case_types)]
pub struct IF_ACMPNE {
    offset: i32,
}

impl InstructionExecutor for IF_ACMPNE {
    fn execute(&self, frame: &mut Frame) {
        let ref2 = frame.operand_stack().pop_ref();
        let ref1 = frame.operand_stack().pop_ref();
        if ref2 != ref1 {
            frame.branch(self.offset);
        }
    }
}
