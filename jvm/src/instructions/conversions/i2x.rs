use crate::instructions::{InstructionExecutor, InstructionReader};
use crate::rtda::Frame;
use jvm_macros::NoOperand;
use std::io::Cursor;

#[derive(NoOperand)]
#[allow(non_camel_case_types)]
pub struct I2B;

impl InstructionExecutor for I2B {
    fn execute(&self, frame: &mut Frame) {
        let val = frame.operand_stack().pop_int() as i8;
        frame.operand_stack().push_int(val as i32);
    }
}

#[derive(NoOperand)]
#[allow(non_camel_case_types)]
pub struct I2C;

impl InstructionExecutor for I2C {
    fn execute(&self, frame: &mut Frame) {
        let val = frame.operand_stack().pop_int() as u16;
        frame.operand_stack().push_int(val as i32);
    }
}

#[derive(NoOperand)]
#[allow(non_camel_case_types)]
pub struct I2S;

impl InstructionExecutor for I2S {
    fn execute(&self, frame: &mut Frame) {
        let val = frame.operand_stack().pop_int() as i16;
        frame.operand_stack().push_int(val as i32);
    }
}

#[derive(NoOperand)]
#[allow(non_camel_case_types)]
pub struct I2D;

impl InstructionExecutor for I2D {
    fn execute(&self, frame: &mut Frame) {
        let val = frame.operand_stack().pop_int();
        frame.operand_stack().push_double(val as f64);
    }
}

#[derive(NoOperand)]
#[allow(non_camel_case_types)]
pub struct I2F;

impl InstructionExecutor for I2F {
    fn execute(&self, frame: &mut Frame) {
        let val = frame.operand_stack().pop_int();
        frame.operand_stack().push_float(val as f32);
    }
}

#[derive(NoOperand)]
#[allow(non_camel_case_types)]
pub struct I2L;

impl InstructionExecutor for I2L {
    fn execute(&self, frame: &mut Frame) {
        let val = frame.operand_stack().pop_int();
        frame.operand_stack().push_long(val as i64);
    }
}
