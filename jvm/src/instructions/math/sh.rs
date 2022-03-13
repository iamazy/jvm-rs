use crate::instructions::{InstructionExecutor, InstructionReader};
use crate::rtda::Frame;
use jvm_macros::NoOperand;
use std::io::Cursor;

#[derive(NoOperand)]
#[allow(non_camel_case_types)]
pub struct ISHL;

impl InstructionExecutor for ISHL {
    fn execute(&self, frame: &mut Frame) {
        let val2 = frame.operand_stack().pop_int();
        let val1 = frame.operand_stack().pop_int();
        let result = val1 << (val2 & 0x1f);
        frame.operand_stack().push_int(result);
    }
}

#[derive(NoOperand)]
#[allow(non_camel_case_types)]
pub struct ISHR;

impl InstructionExecutor for ISHR {
    fn execute(&self, frame: &mut Frame) {
        let val2 = frame.operand_stack().pop_int();
        let val1 = frame.operand_stack().pop_int();
        let result = val1 >> (val2 & 0x1f);
        frame.operand_stack().push_int(result);
    }
}

#[derive(NoOperand)]
#[allow(non_camel_case_types)]
pub struct IUSHR;

impl InstructionExecutor for IUSHR {
    fn execute(&self, frame: &mut Frame) {
        let val2 = frame.operand_stack().pop_int();
        let val1 = frame.operand_stack().pop_int();
        let result = val1 >> (val2 & 0x1f);
        frame.operand_stack().push_int(result);
    }
}

#[derive(NoOperand)]
#[allow(non_camel_case_types)]
pub struct LSHL;

impl InstructionExecutor for LSHL {
    fn execute(&self, frame: &mut Frame) {
        let val2 = frame.operand_stack().pop_int();
        let val1 = frame.operand_stack().pop_long();
        let result = val1 << (val2 & 0x3f);
        frame.operand_stack().push_long(result);
    }
}

#[derive(NoOperand)]
#[allow(non_camel_case_types)]
pub struct LSHR;

impl InstructionExecutor for LSHR {
    fn execute(&self, frame: &mut Frame) {
        let val2 = frame.operand_stack().pop_int();
        let val1 = frame.operand_stack().pop_long();
        let result = val1 >> (val2 & 0x3f);
        frame.operand_stack().push_long(result);
    }
}

#[derive(NoOperand)]
#[allow(non_camel_case_types)]
pub struct LUSHR;

impl InstructionExecutor for LUSHR {
    fn execute(&self, frame: &mut Frame) {
        let val2 = frame.operand_stack().pop_int();
        let val1 = frame.operand_stack().pop_long();
        let result = val1 >> (val2 & 0x3f);
        frame.operand_stack().push_long(result);
    }
}
