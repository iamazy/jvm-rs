use crate::instructions::{InstructionExecutor, InstructionReader};
use crate::rtda::Frame;
use bytes::Buf;
use jvm_macros::Branch;
use std::io::Cursor;

#[derive(Branch)]
#[allow(non_camel_case_types)]
pub struct IFEQ {
    offset: i32,
}

#[derive(Branch)]
#[allow(non_camel_case_types)]
pub struct IFNE {
    offset: i32,
}

#[derive(Branch)]
#[allow(non_camel_case_types)]
pub struct IFLE {
    offset: i32,
}

#[derive(Branch)]
#[allow(non_camel_case_types)]
pub struct IFLT {
    offset: i32,
}

#[derive(Branch)]
#[allow(non_camel_case_types)]
pub struct IFGE {
    offset: i32,
}

#[derive(Branch)]
#[allow(non_camel_case_types)]
pub struct IFGT {
    offset: i32,
}

macro_rules! register_ifcond {
    ($(($inst:ident, $val:ident, $expr:expr)),*) => {
        $(
            impl InstructionExecutor for $inst {
                fn execute(&self, frame: &mut Frame) {
                    let $val = frame.operand_stack().pop_int();
                    if $expr {
                        frame.branch(self.offset);
                    }
                }
            }
        )*
    };
}

register_ifcond! {
    (IFEQ, val, val == 0),
    (IFNE, val, val != 0),
    (IFLE, val, val <= 0),
    (IFLT, val, val < 0),
    (IFGE, val, val >=0),
    (IFGT, val, val > 0)
}