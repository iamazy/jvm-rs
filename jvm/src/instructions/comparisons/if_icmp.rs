use crate::instructions::{InstructionExecutor, InstructionReader};
use crate::rtda::Frame;
use bytes::Buf;
use jvm_macros::Branch;
use std::io::Cursor;

#[derive(Branch)]
#[allow(non_camel_case_types)]
pub struct IF_ICMPEQ {
    offset: i32,
}

#[derive(Branch)]
#[allow(non_camel_case_types)]
pub struct IF_ICMPNE {
    offset: i32,
}

#[derive(Branch)]
#[allow(non_camel_case_types)]
pub struct IF_ICMPLE {
    offset: i32,
}

#[derive(Branch)]
#[allow(non_camel_case_types)]
pub struct IF_ICMPLT {
    offset: i32,
}

#[derive(Branch)]
#[allow(non_camel_case_types)]
pub struct IF_ICMPGE {
    offset: i32,
}

#[derive(Branch)]
#[allow(non_camel_case_types)]
pub struct IF_ICMPGT {
    offset: i32,
}

macro_rules! register_if_icmp {
    ($(($inst:ident, $val1:ident, $val2:ident, $expr:expr)),*) => {
        $(
            impl InstructionExecutor for $inst {
                fn execute(&self, frame: &mut Frame) {
                    let $val2 = frame.operand_stack().pop_int();
                    let $val1 = frame.operand_stack().pop_int();
                    if $expr {
                        frame.branch(self.offset)
                    }
                }
            }
        )*
    };
}

register_if_icmp!{
    (IF_ICMPEQ, val1, val2, val1 == val2),
    (IF_ICMPNE, val1, val2, val1 != val2),
    (IF_ICMPLE, val1, val2, val1 <= val2),
    (IF_ICMPLT, val1, val2, val1 < val2),
    (IF_ICMPGE, val1, val2, val1 >=  val2),
    (IF_ICMPGT, val1, val2, val1 > val2)
}