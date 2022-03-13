use crate::instructions::{InstructionExecutor, InstructionReader};
use crate::rtda::Frame;
use jvm_macros::NoOperand;
use std::io::Cursor;

#[derive(NoOperand)]
#[allow(non_camel_case_types)]
pub struct DSUB;

impl InstructionExecutor for DSUB {
    fn execute(&self, frame: &mut Frame) {
        let val1 = frame.operand_stack().pop_double();
        let val2 = frame.operand_stack().pop_double();
        frame.operand_stack().push_double(val1 - val2);
    }
}

#[derive(NoOperand)]
#[allow(non_camel_case_types)]
pub struct FSUB;

impl InstructionExecutor for FSUB {
    fn execute(&self, frame: &mut Frame) {
        let val1 = frame.operand_stack().pop_float();
        let val2 = frame.operand_stack().pop_float();
        frame.operand_stack().push_float(val1 - val2);
    }
}

#[derive(NoOperand)]
#[allow(non_camel_case_types)]
pub struct ISUB;

impl InstructionExecutor for ISUB {
    fn execute(&self, frame: &mut Frame) {
        let val1 = frame.operand_stack().pop_int();
        let val2 = frame.operand_stack().pop_int();
        frame.operand_stack().push_int(val1 - val2);
    }
}

#[derive(NoOperand)]
#[allow(non_camel_case_types)]
pub struct LSUB;

impl InstructionExecutor for LSUB {
    fn execute(&self, frame: &mut Frame) {
        let val1 = frame.operand_stack().pop_long();
        let val2 = frame.operand_stack().pop_long();
        frame.operand_stack().push_long(val1 - val2);
    }
}
