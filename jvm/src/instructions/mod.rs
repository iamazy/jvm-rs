use crate::rtda::Frame;
use bytes::{Buf, BytesMut};

mod comparisons;
mod constants;
mod control;
mod conversions;
mod extended;
mod loads;
mod math;
mod opcode;
mod stack;
mod stores;

trait Instruction {
    fn fetch_operands(&mut self, reader: &mut BytesMut);
}

trait InstructionExecutor {
    fn execute(&self, frame: &mut Frame);
}

struct BranchInstruction {
    offset: i32,
}

impl Instruction for BranchInstruction {
    fn fetch_operands(&mut self, reader: &mut BytesMut) {
        self.offset = reader.get_i16() as i32;
    }
}

pub struct Index8Instruction {
    index: u32,
}

impl Instruction for Index8Instruction {
    fn fetch_operands(&mut self, reader: &mut BytesMut) {
        self.index = reader.get_u8() as u32;
    }
}

pub struct Index16Instruction {
    index: u32,
}

impl Instruction for Index16Instruction {
    fn fetch_operands(&mut self, reader: &mut BytesMut) {
        self.index = reader.get_u16() as u32;
    }
}
