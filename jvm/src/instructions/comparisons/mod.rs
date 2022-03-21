use crate::instructions::{InstructionExecutor, InstructionReader};
use crate::rtda::Frame;
use bytes::Buf;
use jvm_macros::Branch;
use std::io::Cursor;

macro_rules! register_if_cmp {
    ($(($inst:ident, $func:ident, $op:tt)),*) => {
        $(
            #[derive(Branch, Default, Debug)]
            #[allow(non_camel_case_types)]
            pub struct $inst {
                offset: i32,
            }

            impl InstructionExecutor for $inst {
                fn execute(&self, frame: &mut Frame) {
                    let val2 = frame.operand_stack_mut().$func();
                    let val1 = frame.operand_stack_mut().$func();
                    if val1 $op val2 {
                        frame.branch(self.offset)
                    }
                }
            }
        )*
    };
}

macro_rules! fn_cmp {
    ($(($name:ident, $func:ident)),*) => {
        $(
            fn $name(frame: &mut Frame, flag: bool) {
                let v2 = frame.operand_stack_mut().$func();
                let v1 = frame.operand_stack_mut().$func();
                let value = if v1 > v2 {
                    1
                } else if v1 == v2 {
                    0
                } else if v1 < v2 {
                    -1
                } else if flag {
                    1
                } else {
                    -1
                };
                frame.operand_stack_mut().push_int(value);
            }
        )*
    }
}

fn_cmp! {
    (dcmp, pop_double),
    (fcmp, pop_float)
}

register_if_cmp! {
    (IF_ACMPEQ, pop_ref, ==),
    (IF_ACMPNE, pop_ref, !=)
}

register_if_cmp! {
    (IF_ICMPEQ, pop_int, ==),
    (IF_ICMPNE, pop_int, !=),
    (IF_ICMPLE, pop_int, <=),
    (IF_ICMPLT, pop_int, <),
    (IF_ICMPGE, pop_int, >=),
    (IF_ICMPGT, pop_int, >)
}

pub(crate) mod dcmp;
pub(crate) mod fcmp;
pub(crate) mod ifcond;
pub(crate) mod lcmp;
