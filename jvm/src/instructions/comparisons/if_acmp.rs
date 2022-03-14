use crate::instructions::{InstructionExecutor, InstructionReader};
use crate::rtda::Frame;
use bytes::Buf;
use jvm_macros::Branch;
use std::io::Cursor;

register_if_cmp! {
    (IF_ACMPEQ, pop_ref, |val1, val2| val1 == val2),
    (IF_ACMPNE, pop_ref, |val1, val2| val1 != val2)
}
