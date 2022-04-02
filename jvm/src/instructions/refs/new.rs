use jvm_macros::Index8;
use crate::instructions::{InstructionReader, InstructionExecutor};
use crate::rtda::Frame;
use std::io::Cursor;

// #[derive(Index8)]
// pub struct NEW {
//     index: usize,
// }

// impl InstructionExecutor for NEW {
//     fn execute(&self, frame: &mut Frame) {
//         let slot = frame
//     }
// }