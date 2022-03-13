use crate::instructions::{InstructionExecutor, InstructionReader};
use crate::rtda::Frame;
use jvm_macros::NoOperand;
use std::io::Cursor;

#[derive(NoOperand)]
#[allow(non_camel_case_types)]
pub struct DREM;

impl InstructionExecutor for DREM {
    fn execute(&self, frame: &mut Frame) {
        let val2 = frame.operand_stack().pop_double();
        let val1 = frame.operand_stack().pop_double();
        let result = val1 % val2;
        frame.operand_stack().push_double(result);
    }
}

#[derive(NoOperand)]
#[allow(non_camel_case_types)]
pub struct FREM;

impl InstructionExecutor for FREM {
    fn execute(&self, frame: &mut Frame) {
        let val2 = frame.operand_stack().pop_float();
        let val1 = frame.operand_stack().pop_float();
        let result = val1 % val2;
        frame.operand_stack().push_float(result);
    }
}

#[derive(NoOperand)]
#[allow(non_camel_case_types)]
pub struct IREM;

impl InstructionExecutor for IREM {
    fn execute(&self, frame: &mut Frame) {
        let val2 = frame.operand_stack().pop_int();
        let val1 = frame.operand_stack().pop_int();
        if val2 == 0 {
            panic!("java.lang.ArithmeticException: / by zero");
        }
        let result = val1 % val2;
        frame.operand_stack().push_int(result);
    }
}

#[derive(NoOperand)]
#[allow(non_camel_case_types)]
pub struct LREM;

impl InstructionExecutor for LREM {
    fn execute(&self, frame: &mut Frame) {
        let val2 = frame.operand_stack().pop_long();
        let val1 = frame.operand_stack().pop_long();
        if val2 == 0 {
            panic!("java.lang.ArithmeticException: / by zero");
        }
        let result = val1 % val2;
        frame.operand_stack().push_long(result);
    }
}
