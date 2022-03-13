use crate::instructions::{InstructionExecutor, InstructionReader};
use crate::rtda::Frame;
use bytes::Buf;
use jvm_macros::Branch;
use std::io::Cursor;

#[derive(Branch)]
#[allow(non_camel_case_types)]
pub struct IF_ICMPEQ {
    offset: i32,
}

impl InstructionExecutor for IF_ICMPEQ {
    fn execute(&self, frame: &mut Frame) {
        let val2 = frame.operand_stack().pop_int();
        let val1 = frame.operand_stack().pop_int();
        if val1 == val2 {
            frame.branch(self.offset)
        }
    }
}

#[derive(Branch)]
#[allow(non_camel_case_types)]
pub struct IF_ICMPNE {
    offset: i32,
}

impl InstructionExecutor for IF_ICMPNE {
    fn execute(&self, frame: &mut Frame) {
        let val2 = frame.operand_stack().pop_int();
        let val1 = frame.operand_stack().pop_int();
        if val1 != val2 {
            frame.branch(self.offset)
        }
    }
}

#[derive(Branch)]
#[allow(non_camel_case_types)]
pub struct IF_ICMPLE {
    offset: i32,
}

impl InstructionExecutor for IF_ICMPLE {
    fn execute(&self, frame: &mut Frame) {
        let val2 = frame.operand_stack().pop_int();
        let val1 = frame.operand_stack().pop_int();
        if val1 <= val2 {
            frame.branch(self.offset)
        }
    }
}

#[derive(Branch)]
#[allow(non_camel_case_types)]
pub struct IF_ICMPLT {
    offset: i32,
}

impl InstructionExecutor for IF_ICMPLT {
    fn execute(&self, frame: &mut Frame) {
        let val2 = frame.operand_stack().pop_int();
        let val1 = frame.operand_stack().pop_int();
        if val1 < val2 {
            frame.branch(self.offset)
        }
    }
}

#[derive(Branch)]
#[allow(non_camel_case_types)]
pub struct IF_ICMPGE {
    offset: i32,
}

impl InstructionExecutor for IF_ICMPGE {
    fn execute(&self, frame: &mut Frame) {
        let val2 = frame.operand_stack().pop_int();
        let val1 = frame.operand_stack().pop_int();
        if val1 >= val2 {
            frame.branch(self.offset)
        }
    }
}

#[derive(Branch)]
#[allow(non_camel_case_types)]
pub struct IF_ICMPGT {
    offset: i32,
}

impl InstructionExecutor for IF_ICMPGT {
    fn execute(&self, frame: &mut Frame) {
        let val2 = frame.operand_stack().pop_int();
        let val1 = frame.operand_stack().pop_int();
        if val1 > val2 {
            frame.branch(self.offset)
        }
    }
}
