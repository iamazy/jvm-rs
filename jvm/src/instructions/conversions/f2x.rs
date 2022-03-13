use crate::instructions::{InstructionExecutor, InstructionReader};
use crate::rtda::Frame;
use jvm_macros::NoOperand;
use std::io::Cursor;

#[derive(NoOperand)]
#[allow(non_camel_case_types)]
pub struct F2D;

impl InstructionExecutor for F2D {
    fn execute(&self, frame: &mut Frame) {
        let val = frame.operand_stack().pop_float();
        frame.operand_stack().push_double(val as f64);
    }
}

#[derive(NoOperand)]
#[allow(non_camel_case_types)]
pub struct F2I;

impl InstructionExecutor for F2I {
    fn execute(&self, frame: &mut Frame) {
        let val = frame.operand_stack().pop_float();
        frame.operand_stack().push_int(val as i32);
    }
}

#[derive(NoOperand)]
#[allow(non_camel_case_types)]
pub struct F2L;

impl InstructionExecutor for F2L {
    fn execute(&self, frame: &mut Frame) {
        let val = frame.operand_stack().pop_float();
        frame.operand_stack().push_long(val as i64);
    }
}
