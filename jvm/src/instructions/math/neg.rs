use crate::instructions::{InstructionExecutor, InstructionReader};
use crate::rtda::Frame;
use jvm_macros::NoOperand;
use std::io::Cursor;

#[derive(NoOperand, Debug)]
#[allow(non_camel_case_types)]
pub struct DNEG;

impl InstructionExecutor for DNEG {
    fn execute(&self, frame: &mut Frame) {
        let val = frame.operand_stack_mut().pop_double();
        frame.operand_stack_mut().push_double(-val);
    }
}

#[derive(NoOperand, Debug)]
#[allow(non_camel_case_types)]
pub struct FNEG;

impl InstructionExecutor for FNEG {
    fn execute(&self, frame: &mut Frame) {
        let val = frame.operand_stack_mut().pop_float();
        frame.operand_stack_mut().push_float(-val);
    }
}

#[derive(NoOperand, Debug)]
#[allow(non_camel_case_types)]
pub struct INEG;

impl InstructionExecutor for INEG {
    fn execute(&self, frame: &mut Frame) {
        let val = frame.operand_stack_mut().pop_int();
        frame.operand_stack_mut().push_int(-val);
    }
}

#[derive(NoOperand, Debug)]
#[allow(non_camel_case_types)]
pub struct LNEG;

impl InstructionExecutor for LNEG {
    fn execute(&self, frame: &mut Frame) {
        let val = frame.operand_stack_mut().pop_long();
        frame.operand_stack_mut().push_long(-val);
    }
}
