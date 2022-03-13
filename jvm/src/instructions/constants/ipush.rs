use crate::instructions::{InstructionExecutor, InstructionReader};
use bytes::Buf;
use std::io::Cursor;

pub struct BIPUSH {
    pub val: i8,
}

impl<T: AsRef<[u8]>> InstructionReader<T> for BIPUSH {
    fn fetch_operands(&mut self, reader: &mut Cursor<T>) {
        self.val = reader.get_i8();
    }
}

impl InstructionExecutor for BIPUSH {
    fn execute(&self, frame: &mut crate::rtda::Frame) {
        frame.operand_stack().push_int(self.val as i32);
    }
}

pub struct SIPUSH {
    pub val: i16,
}

impl<T: AsRef<[u8]>> InstructionReader<T> for SIPUSH {
    fn fetch_operands(&mut self, reader: &mut Cursor<T>) {
        self.val = reader.get_i16();
    }
}

impl InstructionExecutor for SIPUSH {
    fn execute(&self, frame: &mut crate::rtda::Frame) {
        frame.operand_stack().push_int(self.val as i32);
    }
}
