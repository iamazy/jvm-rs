use crate::instructions::{InstructionExecutor, InstructionReader};
use crate::rtda::Frame;
use bytes::Buf;
use jvm_macros::Branch;
use std::io::Cursor;

register_if_cmp! {
    (IF_ICMPEQ, pop_int, ==),
    (IF_ICMPNE, pop_int, !=),
    (IF_ICMPLE, pop_int, <=),
    (IF_ICMPLT, pop_int, <),
    (IF_ICMPGE, pop_int, >=),
    (IF_ICMPGT, pop_int, >)
}
