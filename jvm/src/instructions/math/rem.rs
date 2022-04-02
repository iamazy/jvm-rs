use crate::instructions::{InstructionExecutor, InstructionReader};
use crate::rtda::Frame;
use jvm_macros::NoOperand;

#[derive(NoOperand, Debug)]
#[allow(non_camel_case_types)]
pub struct DREM;

impl InstructionExecutor for DREM {
    fn execute(&self, frame: &mut Frame) {
        let val2 = frame.operand_stack_mut().pop_double();
        let val1 = frame.operand_stack_mut().pop_double();
        let result = val1 % val2;
        frame.operand_stack_mut().push_double(result);
    }
}

#[derive(NoOperand, Debug)]
#[allow(non_camel_case_types)]
pub struct FREM;

impl InstructionExecutor for FREM {
    fn execute(&self, frame: &mut Frame) {
        let val2 = frame.operand_stack_mut().pop_float() as f64;
        let val1 = frame.operand_stack_mut().pop_float() as f64;
        let result = val1 % val2;
        frame.operand_stack_mut().push_float(result as f32);
    }
}

#[derive(NoOperand, Debug)]
#[allow(non_camel_case_types)]
pub struct IREM;

impl InstructionExecutor for IREM {
    fn execute(&self, frame: &mut Frame) {
        let val2 = frame.operand_stack_mut().pop_int();
        let val1 = frame.operand_stack_mut().pop_int();
        if val2 == 0 {
            panic!("java.lang.ArithmeticException: / by zero");
        }
        let result = val1 % val2;
        frame.operand_stack_mut().push_int(result);
    }
}

#[derive(NoOperand, Debug)]
#[allow(non_camel_case_types)]
pub struct LREM;

impl InstructionExecutor for LREM {
    fn execute(&self, frame: &mut Frame) {
        let val2 = frame.operand_stack_mut().pop_long();
        let val1 = frame.operand_stack_mut().pop_long();
        if val2 == 0 {
            panic!("java.lang.ArithmeticException: / by zero");
        }
        let result = val1 % val2;
        frame.operand_stack_mut().push_long(result);
    }
}
