use crate::instructions::{InstructionExecutor, InstructionReader};
use crate::rtda::Frame;
use jvm_macros::NoOperand;

#[derive(NoOperand, Debug)]
#[allow(non_camel_case_types)]
pub struct ISHL;

impl InstructionExecutor for ISHL {
    fn execute(&self, frame: &mut Frame) {
        let val2 = frame.operand_stack_mut().pop_int();
        let val1 = frame.operand_stack_mut().pop_int();
        let result = val1 << (val2 as u32 & 0x1f);
        frame.operand_stack_mut().push_int(result);
    }
}

#[derive(NoOperand, Debug)]
#[allow(non_camel_case_types)]
pub struct ISHR;

impl InstructionExecutor for ISHR {
    fn execute(&self, frame: &mut Frame) {
        let val2 = frame.operand_stack_mut().pop_int();
        let val1 = frame.operand_stack_mut().pop_int();
        let result = val1 >> (val2 as u32 & 0x1f);
        frame.operand_stack_mut().push_int(result);
    }
}

#[derive(NoOperand, Debug)]
#[allow(non_camel_case_types)]
pub struct IUSHR;

impl InstructionExecutor for IUSHR {
    fn execute(&self, frame: &mut Frame) {
        let val2 = frame.operand_stack_mut().pop_int();
        let val1 = frame.operand_stack_mut().pop_int();
        let result = val1 as u32 >> (val2 as u32 & 0x1f);
        frame.operand_stack_mut().push_int(result as i32);
    }
}

#[derive(NoOperand, Debug)]
#[allow(non_camel_case_types)]
pub struct LSHL;

impl InstructionExecutor for LSHL {
    fn execute(&self, frame: &mut Frame) {
        let val2 = frame.operand_stack_mut().pop_int();
        let val1 = frame.operand_stack_mut().pop_long();
        let result = val1 << (val2 as u32 & 0x3f);
        frame.operand_stack_mut().push_long(result);
    }
}

#[derive(NoOperand, Debug)]
#[allow(non_camel_case_types)]
pub struct LSHR;

impl InstructionExecutor for LSHR {
    fn execute(&self, frame: &mut Frame) {
        let val2 = frame.operand_stack_mut().pop_int();
        let val1 = frame.operand_stack_mut().pop_long();
        let result = val1 >> (val2 as u32 & 0x3f);
        frame.operand_stack_mut().push_long(result);
    }
}

#[derive(NoOperand, Debug)]
#[allow(non_camel_case_types)]
pub struct LUSHR;

impl InstructionExecutor for LUSHR {
    fn execute(&self, frame: &mut Frame) {
        let val2 = frame.operand_stack_mut().pop_int();
        let val1 = frame.operand_stack_mut().pop_long();
        let result = val1 as u64 >> (val2 as u32 & 0x3f);
        frame.operand_stack_mut().push_long(result as i64);
    }
}
