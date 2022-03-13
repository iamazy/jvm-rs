use crate::instructions::{InstructionExecutor, InstructionReader};
use crate::rtda::Frame;
use jvm_macros::NoOperand;
use std::io::Cursor;

#[derive(NoOperand)]
#[allow(non_camel_case_types)]
pub struct D2F;

impl InstructionExecutor for D2F {
    fn execute(&self, frame: &mut Frame) {
        let val = frame.operand_stack().pop_double();
        frame.operand_stack().push_float(val as f32);
    }
}

#[derive(NoOperand)]
#[allow(non_camel_case_types)]
pub struct D2I;

impl InstructionExecutor for D2I {
    fn execute(&self, frame: &mut Frame) {
        let val = frame.operand_stack().pop_double();
        frame.operand_stack().push_int(val as i32);
    }
}

#[derive(NoOperand)]
#[allow(non_camel_case_types)]
pub struct D2L;

impl InstructionExecutor for D2L {
    fn execute(&self, frame: &mut Frame) {
        let val = frame.operand_stack().pop_double();
        frame.operand_stack().push_long(val as i64);
    }
}
