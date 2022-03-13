use crate::instructions::{InstructionExecutor, InstructionReader};
use crate::rtda::Frame;
use jvm_macros::NoOperand;
use std::io::Cursor;

#[derive(NoOperand)]
#[allow(non_camel_case_types)]
pub struct DUP;

impl InstructionExecutor for DUP {
    fn execute(&self, frame: &mut Frame) {
        let slot = frame.operand_stack().pop_slot();
        frame.operand_stack().push_slot(slot.clone());
        frame.operand_stack().push_slot(slot);
    }
}

#[derive(NoOperand)]
#[allow(non_camel_case_types)]
pub struct DUP_X1;

impl InstructionExecutor for DUP_X1 {
    fn execute(&self, frame: &mut Frame) {
        let slot1 = frame.operand_stack().pop_slot();
        let slot2 = frame.operand_stack().pop_slot();
        frame.operand_stack().push_slot(slot1.clone());
        frame.operand_stack().push_slot(slot2);
        frame.operand_stack().push_slot(slot1);
    }
}

#[derive(NoOperand)]
#[allow(non_camel_case_types)]
pub struct DUP_X2;

impl InstructionExecutor for DUP_X2 {
    fn execute(&self, frame: &mut Frame) {
        let slot1 = frame.operand_stack().pop_slot();
        let slot2 = frame.operand_stack().pop_slot();
        let slot3 = frame.operand_stack().pop_slot();
        frame.operand_stack().push_slot(slot1.clone());
        frame.operand_stack().push_slot(slot3);
        frame.operand_stack().push_slot(slot2);
        frame.operand_stack().push_slot(slot1);
    }
}

#[derive(NoOperand)]
#[allow(non_camel_case_types)]
pub struct DUP2;

impl InstructionExecutor for DUP2 {
    fn execute(&self, frame: &mut Frame) {
        let slot1 = frame.operand_stack().pop_slot();
        let slot2 = frame.operand_stack().pop_slot();
        frame.operand_stack().push_slot(slot2.clone());
        frame.operand_stack().push_slot(slot1.clone());
        frame.operand_stack().push_slot(slot2);
        frame.operand_stack().push_slot(slot1);
    }
}

#[derive(NoOperand)]
#[allow(non_camel_case_types)]
pub struct DUP2_X1;

impl InstructionExecutor for DUP2_X1 {
    fn execute(&self, frame: &mut Frame) {
        let slot1 = frame.operand_stack().pop_slot();
        let slot2 = frame.operand_stack().pop_slot();
        let slot3 = frame.operand_stack().pop_slot();
        frame.operand_stack().push_slot(slot2.clone());
        frame.operand_stack().push_slot(slot1.clone());
        frame.operand_stack().push_slot(slot3);
        frame.operand_stack().push_slot(slot2);
        frame.operand_stack().push_slot(slot1);
    }
}

#[derive(NoOperand)]
#[allow(non_camel_case_types)]
pub struct DUP2_X2;

impl InstructionExecutor for DUP2_X2 {
    fn execute(&self, frame: &mut Frame) {
        let slot1 = frame.operand_stack().pop_slot();
        let slot2 = frame.operand_stack().pop_slot();
        let slot3 = frame.operand_stack().pop_slot();
        let slot4 = frame.operand_stack().pop_slot();
        frame.operand_stack().push_slot(slot2.clone());
        frame.operand_stack().push_slot(slot1.clone());
        frame.operand_stack().push_slot(slot4);
        frame.operand_stack().push_slot(slot3);
        frame.operand_stack().push_slot(slot2);
        frame.operand_stack().push_slot(slot1);
    }
}
