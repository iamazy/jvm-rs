use crate::instructions::{Instruction, InstructionExecutor};
use crate::rtda::Frame;
use bytes::BytesMut;
use jvm_macros::NoOperand;

#[derive(NoOperand)]
pub struct Nop;

#[derive(NoOperand)]
#[allow(non_camel_case_types)]
pub struct ACONST_NULL;

impl InstructionExecutor for ACONST_NULL {
    fn execute(&self, frame: &mut Frame) {
        frame.operand_stack().push_ref(None);
    }
}

#[derive(NoOperand)]
#[allow(non_camel_case_types)]
pub struct DCONST_0;

impl InstructionExecutor for DCONST_0 {
    fn execute(&self, frame: &mut Frame) {
        frame.operand_stack().push_double(0.0);
    }
}

#[derive(NoOperand)]
#[allow(non_camel_case_types)]
pub struct DCONST_1;

impl InstructionExecutor for DCONST_1 {
    fn execute(&self, frame: &mut Frame) {
        frame.operand_stack().push_double(1.0);
    }
}

#[derive(NoOperand)]
#[allow(non_camel_case_types)]
pub struct FCONST_0;

impl InstructionExecutor for FCONST_0 {
    fn execute(&self, frame: &mut Frame) {
        frame.operand_stack().push_float(0.0);
    }
}

#[derive(NoOperand)]
#[allow(non_camel_case_types)]
pub struct FCONST_1;

impl InstructionExecutor for FCONST_1 {
    fn execute(&self, frame: &mut Frame) {
        frame.operand_stack().push_float(1.0);
    }
}

#[derive(NoOperand)]
#[allow(non_camel_case_types)]
pub struct FCONST_2;

impl InstructionExecutor for FCONST_2 {
    fn execute(&self, frame: &mut Frame) {
        frame.operand_stack().push_float(2.0);
    }
}

#[derive(NoOperand)]
#[allow(non_camel_case_types)]
pub struct ICONST_M1;

impl InstructionExecutor for ICONST_M1 {
    fn execute(&self, frame: &mut Frame) {
        frame.operand_stack().push_int(-1);
    }
}

#[derive(NoOperand)]
#[allow(non_camel_case_types)]
pub struct ICONST_0;

impl InstructionExecutor for ICONST_0 {
    fn execute(&self, frame: &mut Frame) {
        frame.operand_stack().push_int(0);
    }
}

#[derive(NoOperand)]
#[allow(non_camel_case_types)]
pub struct ICONST_1;

impl InstructionExecutor for ICONST_1 {
    fn execute(&self, frame: &mut Frame) {
        frame.operand_stack().push_int(1);
    }
}

#[derive(NoOperand)]
#[allow(non_camel_case_types)]
pub struct ICONST_2;

impl InstructionExecutor for ICONST_2 {
    fn execute(&self, frame: &mut Frame) {
        frame.operand_stack().push_int(2);
    }
}

#[derive(NoOperand)]
#[allow(non_camel_case_types)]
pub struct ICONST_3;

impl InstructionExecutor for ICONST_3 {
    fn execute(&self, frame: &mut Frame) {
        frame.operand_stack().push_int(3);
    }
}

#[derive(NoOperand)]
#[allow(non_camel_case_types)]
pub struct ICONST_4;

impl InstructionExecutor for ICONST_4 {
    fn execute(&self, frame: &mut Frame) {
        frame.operand_stack().push_int(4);
    }
}

#[derive(NoOperand)]
#[allow(non_camel_case_types)]
pub struct ICONST_5;

impl InstructionExecutor for ICONST_5 {
    fn execute(&self, frame: &mut Frame) {
        frame.operand_stack().push_int(5);
    }
}

#[derive(NoOperand)]
#[allow(non_camel_case_types)]
pub struct LCONST_0;

impl InstructionExecutor for LCONST_0 {
    fn execute(&self, frame: &mut Frame) {
        frame.operand_stack().push_long(0);
    }
}

#[derive(NoOperand)]
#[allow(non_camel_case_types)]
pub struct LCONST_1;

impl InstructionExecutor for LCONST_1 {
    fn execute(&self, frame: &mut Frame) {
        frame.operand_stack().push_long(1);
    }
}
