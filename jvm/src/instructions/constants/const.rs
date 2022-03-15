use crate::instructions::{InstructionExecutor, InstructionReader};
use crate::rtda::Frame;
use jvm_macros::NoOperand;
use std::io::Cursor;
use std::ptr;

macro_rules! register_const {
    ($(($inst:ident, $func:ident($expr:expr))),*) => {
        $(
            #[derive(NoOperand)]
            #[allow(non_camel_case_types)]
            pub struct $inst;

            impl InstructionExecutor for $inst {
                fn execute(&self, frame: &mut Frame) {
                   frame.operand_stack().$func($expr);
                }
            }
        )*
    };
}

register_const! {
    (ACONST_NULL, push_ref(ptr::null_mut())),
    (DCONST_0, push_double(0.0)),
    (DCONST_1, push_double(1.0)),
    (FCONST_0, push_float(0.0)),
    (FCONST_1, push_float(1.0)),
    (FCONST_2, push_float(2.0)),
    (ICONST_M1, push_int(-1)),
    (ICONST_0, push_int(0)),
    (ICONST_1, push_int(1)),
    (ICONST_2, push_int(2)),
    (ICONST_3, push_int(3)),
    (ICONST_4, push_int(4)),
    (ICONST_5, push_int(5)),
    (LCONST_0, push_long(0)),
    (LCONST_1, push_long(1))
}
