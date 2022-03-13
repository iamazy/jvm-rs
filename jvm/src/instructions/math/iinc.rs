use crate::instructions::{InstructionExecutor, InstructionReader};
use bytes::Buf;
use std::io::Cursor;

pub struct IINC {
    pub index: usize,
    pub r#const: i32,
}

impl IINC {
    #[inline]
    pub fn new(index: usize, r#const: i32) -> IINC {
        IINC { index, r#const }
    }
}

impl<T: AsRef<[u8]>> InstructionReader<T> for IINC {
    fn fetch_operands(&mut self, reader: &mut Cursor<T>) {
        self.index = reader.get_u8() as usize;
        self.r#const = reader.get_i8() as i32;
    }
}

impl InstructionExecutor for IINC {
    fn execute(&self, frame: &mut crate::rtda::Frame) {
        let val = frame.local_vars().get_int(self.index);
        frame.local_vars().set_int(self.index, val + self.r#const);
    }
}
