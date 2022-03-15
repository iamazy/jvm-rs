use crate::instructions::{InstructionExecutor, InstructionReader};
use crate::rtda::Frame;
use bytes::Buf;
use jvm_macros::Branch;
use std::io::Cursor;

register_if_cmp! {
    (IF_ACMPEQ, pop_ref, ==),
    (IF_ACMPNE, pop_ref, !=)
}
