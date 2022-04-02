use crate::instructions::{InstructionExecutor, InstructionReader};
use crate::rtda::Frame;
use bytes::Buf;

#[derive(Default, Debug)]
#[allow(non_camel_case_types)]
pub struct TABLE_SWITCH {
    default_offset: i32,
    low: i32,
    high: i32,
    jump_offsets: Vec<i32>,
}

impl<T: AsRef<[u8]>> InstructionReader<T> for TABLE_SWITCH {
    fn fetch_operands(&mut self, reader: &mut std::io::Cursor<T>) {
        while reader.position() % 4 != 0 {
            reader.get_u8();
        }
        self.default_offset = reader.get_i32();
        self.low = reader.get_i32();
        self.high = reader.get_i32();
        let jump_offsets_count = self.high - self.low + 1;
        let mut jump_offsets = Vec::with_capacity(jump_offsets_count as usize);
        for _ in 0..jump_offsets_count {
            jump_offsets.push(reader.get_i32());
        }
        self.jump_offsets = jump_offsets;
    }
}

impl InstructionExecutor for TABLE_SWITCH {
    fn execute(&self, frame: &mut Frame) {
        let index = frame.operand_stack_mut().pop_int();
        let offset = if index >= self.low && index <= self.high {
            self.jump_offsets[(index - self.low) as usize]
        } else {
            self.default_offset
        };
        frame.branch(offset);
    }
}

#[derive(Default, Debug)]
#[allow(non_camel_case_types)]
pub struct LOOKUP_SWITCH {
    default_offset: i32,
    pairs_size: i32,
    match_offsets: Vec<i32>,
}

impl<T> InstructionReader<T> for LOOKUP_SWITCH
where
    T: AsRef<[u8]> + std::convert::AsRef<[u8]>,
{
    fn fetch_operands(&mut self, reader: &mut std::io::Cursor<T>) {
        while reader.position() % 4 != 0 {
            reader.get_u8();
        }
        self.default_offset = reader.get_i32();
        self.pairs_size = reader.get_i32();
        let total_size = (self.pairs_size * 2) as usize;
        let mut match_offsets = Vec::with_capacity(total_size);
        for _ in 0..total_size {
            match_offsets.push(reader.get_i32());
        }
        self.match_offsets = match_offsets;
    }
}

impl InstructionExecutor for LOOKUP_SWITCH {
    fn execute(&self, frame: &mut Frame) {
        let key = frame.operand_stack_mut().pop_int();
        let total_size = (self.pairs_size * 2) as usize;
        for i in (0..total_size).step_by(2) {
            if self.match_offsets[i] == key {
                let offset = self.match_offsets[i + 1];
                frame.branch(offset);
                return;
            }
        }
        frame.branch(self.default_offset);
    }
}
