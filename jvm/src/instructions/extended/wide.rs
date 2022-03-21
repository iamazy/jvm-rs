use crate::instructions::opcode::OpCode;
use crate::instructions::{
    Instruction, InstructionExecutor, InstructionReader, ALOAD, ASTORE, DLOAD, DSTORE, FLOAD,
    FSTORE, IINC, ILOAD, ISTORE, LLOAD, LSTORE, NOP,
};
use crate::rtda::Frame;
use bytes::Buf;
use std::io::Cursor;

pub struct WIDE<T> {
    modified_instruction: Box<dyn Instruction<T>>,
}

impl<T: AsRef<[u8]>> Default for WIDE<T> {
    fn default() -> Self {
        Self {
            modified_instruction: Box::new(NOP {}),
        }
    }
}

impl<T: AsRef<[u8]>> InstructionReader<T> for WIDE<T> {
    fn fetch_operands(&mut self, reader: &mut Cursor<T>) {
        let opcode = reader.get_u8();
        match opcode.into() {
            OpCode::iload => {
                let inst = ILOAD::new(reader.get_u16() as usize);
                self.modified_instruction = Box::new(inst);
            }
            OpCode::lload => {
                let inst = LLOAD::new(reader.get_u16() as usize);
                self.modified_instruction = Box::new(inst);
            }
            OpCode::fload => {
                let inst = FLOAD::new(reader.get_u16() as usize);
                self.modified_instruction = Box::new(inst);
            }
            OpCode::dload => {
                let inst = DLOAD::new(reader.get_u16() as usize);
                self.modified_instruction = Box::new(inst);
            }
            OpCode::aload => {
                let inst = ALOAD::new(reader.get_u16() as usize);
                self.modified_instruction = Box::new(inst);
            }
            OpCode::istore => {
                let inst = ISTORE::new(reader.get_u16() as usize);
                self.modified_instruction = Box::new(inst);
            }
            OpCode::lstore => {
                let inst = LSTORE::new(reader.get_u16() as usize);
                self.modified_instruction = Box::new(inst);
            }
            OpCode::fstore => {
                let inst = FSTORE::new(reader.get_u16() as usize);
                self.modified_instruction = Box::new(inst);
            }
            OpCode::dstore => {
                let inst = DSTORE::new(reader.get_u16() as usize);
                self.modified_instruction = Box::new(inst);
            }
            OpCode::astore => {
                let inst = ASTORE::new(reader.get_u16() as usize);
                self.modified_instruction = Box::new(inst);
            }
            OpCode::iinc => {
                let index = reader.get_u16() as usize;
                let r#const = reader.get_u16() as i32;
                let inst = IINC::new(index, r#const);
                self.modified_instruction = Box::new(inst);
            }
            _ => {
                panic!("Unsupported opcode: {:02x}", opcode);
            }
        }
    }
}

impl<T> InstructionExecutor for WIDE<T> {
    fn execute(&self, frame: &mut Frame) {
        self.modified_instruction.execute(frame);
    }
}
