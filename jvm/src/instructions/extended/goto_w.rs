use crate::instructions::{InstructionExecutor, InstructionReader};
use crate::rtda::Frame;
use bytes::Buf;
use std::io::Cursor;

#[derive(Default, Debug)]
#[allow(non_camel_case_types)]
pub struct GOTO_W {
    offset: i32,
}

impl<T: AsRef<[u8]>> InstructionReader<T> for GOTO_W {
    fn fetch_operands(&mut self, reader: &mut Cursor<T>) {
        self.offset = reader.get_i32();
    }
}

impl InstructionExecutor for GOTO_W {
    fn execute(&self, frame: &mut Frame) {
        frame.branch(self.offset);
    }
}
