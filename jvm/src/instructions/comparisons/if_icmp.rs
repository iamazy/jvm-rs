use crate::instructions::{InstructionExecutor, InstructionReader};
use crate::rtda::Frame;
use bytes::Buf;
use jvm_macros::Branch;
use std::io::Cursor;

register_if_cmp! {
    (IF_ICMPEQ, pop_int, |val1, val2 | val1 == val2),
    (IF_ICMPNE, pop_int, |val1, val2 | val1 != val2),
    (IF_ICMPLE, pop_int, |val1, val2 | val1 <= val2),
    (IF_ICMPLT, pop_int, |val1, val2 | val1 < val2),
    (IF_ICMPGE, pop_int, |val1, val2 | val1 >= val2),
    (IF_ICMPGT, pop_int, |val1, val2 | val1 > val2)
}
