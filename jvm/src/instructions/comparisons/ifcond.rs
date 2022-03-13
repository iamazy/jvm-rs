use crate::instructions::{InstructionExecutor, InstructionReader};
use crate::rtda::Frame;
use bytes::Buf;
use jvm_macros::Branch;
use std::io::Cursor;

#[derive(Branch)]
#[allow(non_camel_case_types)]
pub struct IFEQ {
    offset: i32,
}

impl InstructionExecutor for IFEQ {
    fn execute(&self, frame: &mut Frame) {
        let val = frame.operand_stack().pop_int();
        if val == 0 {
            frame.branch(self.offset);
        }
    }
}

#[derive(Branch)]
#[allow(non_camel_case_types)]
pub struct IFNE {
    offset: i32,
}

impl InstructionExecutor for IFNE {
    fn execute(&self, frame: &mut Frame) {
        let val = frame.operand_stack().pop_int();
        if val != 0 {
            frame.branch(self.offset);
        }
    }
}

#[derive(Branch)]
#[allow(non_camel_case_types)]
pub struct IFLE {
    offset: i32,
}

impl InstructionExecutor for IFLE {
    fn execute(&self, frame: &mut Frame) {
        let val = frame.operand_stack().pop_int();
        if val <= 0 {
            frame.branch(self.offset);
        }
    }
}

#[derive(Branch)]
#[allow(non_camel_case_types)]
pub struct IFLT {
    offset: i32,
}

impl InstructionExecutor for IFLT {
    fn execute(&self, frame: &mut Frame) {
        let val = frame.operand_stack().pop_int();
        if val < 0 {
            frame.branch(self.offset);
        }
    }
}

#[derive(Branch)]
#[allow(non_camel_case_types)]
pub struct IFGE {
    offset: i32,
}

impl InstructionExecutor for IFGE {
    fn execute(&self, frame: &mut Frame) {
        let val = frame.operand_stack().pop_int();
        if val >= 0 {
            frame.branch(self.offset);
        }
    }
}

#[derive(Branch)]
#[allow(non_camel_case_types)]
pub struct IFGT {
    offset: i32,
}

impl InstructionExecutor for IFGT {
    fn execute(&self, frame: &mut Frame) {
        let val = frame.operand_stack().pop_int();
        if val > 0 {
            frame.branch(self.offset);
        }
    }
}
