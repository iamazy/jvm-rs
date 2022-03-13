use crate::instructions::{InstructionExecutor, InstructionReader};
use crate::rtda::Frame;
use jvm_macros::NoOperand;
use std::io::Cursor;

#[derive(NoOperand)]
#[allow(non_camel_case_types)]
pub struct L2D;

impl InstructionExecutor for L2D {
    fn execute(&self, frame: &mut Frame) {
        let val = frame.operand_stack().pop_long();
        frame.operand_stack().push_double(val as f64);
    }
}

#[derive(NoOperand)]
#[allow(non_camel_case_types)]
pub struct L2F;

impl InstructionExecutor for L2F {
    fn execute(&self, frame: &mut Frame) {
        let val = frame.operand_stack().pop_long();
        frame.operand_stack().push_float(val as f32);
    }
}

#[derive(NoOperand)]
#[allow(non_camel_case_types)]
pub struct L2I;

impl InstructionExecutor for L2I {
    fn execute(&self, frame: &mut Frame) {
        let val = frame.operand_stack().pop_long();
        frame.operand_stack().push_int(val as i32);
    }
}
