use crate::instructions::{
    InstructionExecutor, InstructionReader, ALOAD, ASTORE, DLOAD, DSTORE, FLOAD, FSTORE, IINC,
    ILOAD, ISTORE, LLOAD, LSTORE,
};
use crate::rtda::Frame;
use bytes::Buf;
use std::io::Cursor;

pub struct WIDE {
    modified_instruction: Box<dyn InstructionExecutor>,
}

impl<T: AsRef<[u8]>> InstructionReader<T> for WIDE {
    fn fetch_operands(&mut self, reader: &mut Cursor<T>) {
        let opcode = reader.get_u8();
        match opcode {
            0x15 => {
                let inst = ILOAD::new(reader.get_u16() as u32);
                self.modified_instruction = Box::new(inst);
            }
            0x16 => {
                let inst = LLOAD::new(reader.get_u16() as u32);
                self.modified_instruction = Box::new(inst);
            }
            0x17 => {
                let inst = FLOAD::new(reader.get_u16() as u32);
                self.modified_instruction = Box::new(inst);
            }
            0x18 => {
                let inst = DLOAD::new(reader.get_u16() as u32);
                self.modified_instruction = Box::new(inst);
            }
            0x19 => {
                let inst = ALOAD::new(reader.get_u16() as u32);
                self.modified_instruction = Box::new(inst);
            }
            0x36 => {
                let inst = ISTORE::new(reader.get_u16() as u32);
                self.modified_instruction = Box::new(inst);
            }
            0x37 => {
                let inst = LSTORE::new(reader.get_u16() as u32);
                self.modified_instruction = Box::new(inst);
            }
            0x38 => {
                let inst = FSTORE::new(reader.get_u16() as u32);
                self.modified_instruction = Box::new(inst);
            }
            0x39 => {
                let inst = DSTORE::new(reader.get_u16() as u32);
                self.modified_instruction = Box::new(inst);
            }
            0x3a => {
                let inst = ASTORE::new(reader.get_u16() as u32);
                self.modified_instruction = Box::new(inst);
            }
            0x84 => {
                let index = reader.get_u16() as usize;
                let r#const = reader.get_u16() as i32;
                let inst = IINC::new(index, r#const);
                self.modified_instruction = Box::new(inst);
            }
            0xa9 => {
                // ret
                panic!("Unsupported opcode: {:02x}", opcode);
            }
            _ => {
                panic!("Unsupported opcode: {:02x}", opcode);
            }
        }
    }
}

impl InstructionExecutor for WIDE {
    fn execute(&self, frame: &mut Frame) {
        self.modified_instruction.execute(frame);
    }
}
