use crate::instructions::{InstructionExecutor, InstructionReader};
use crate::rtda::Frame;
use jvm_macros::NoOperand;
use std::io::Cursor;
use std::ptr;

#[derive(NoOperand)]
#[allow(non_camel_case_types)]
pub struct ACONST_NULL;

#[derive(NoOperand)]
#[allow(non_camel_case_types)]
pub struct DCONST_0;

#[derive(NoOperand)]
#[allow(non_camel_case_types)]
pub struct DCONST_1;

#[derive(NoOperand)]
#[allow(non_camel_case_types)]
pub struct FCONST_0;

#[derive(NoOperand)]
#[allow(non_camel_case_types)]
pub struct FCONST_1;

#[derive(NoOperand)]
#[allow(non_camel_case_types)]
pub struct FCONST_2;

#[derive(NoOperand)]
#[allow(non_camel_case_types)]
pub struct ICONST_M1;

#[derive(NoOperand)]
#[allow(non_camel_case_types)]
pub struct ICONST_0;

#[derive(NoOperand)]
#[allow(non_camel_case_types)]
pub struct ICONST_1;

#[derive(NoOperand)]
#[allow(non_camel_case_types)]
pub struct ICONST_2;

#[derive(NoOperand)]
#[allow(non_camel_case_types)]
pub struct ICONST_3;

#[derive(NoOperand)]
#[allow(non_camel_case_types)]
pub struct ICONST_4;

#[derive(NoOperand)]
#[allow(non_camel_case_types)]
pub struct ICONST_5;

#[derive(NoOperand)]
#[allow(non_camel_case_types)]
pub struct LCONST_0;

#[derive(NoOperand)]
#[allow(non_camel_case_types)]
pub struct LCONST_1;

macro_rules! register_const {
    ($(($inst:ident, $stack:ident, $expr:expr)),*) => {
        $(
            impl InstructionExecutor for $inst {
                fn execute(&self, frame: &mut Frame) {
                    let $stack = frame.operand_stack();
                    $expr;
                }
            }
        )*
    };
}

register_const!{
    (ACONST_NULL, stack, stack.push_ref(ptr::null_mut())),
    (DCONST_0, stack, stack.push_double(0.0)),
    (DCONST_1, stack, stack.push_double(1.0)),
    (FCONST_0, stack, stack.push_float(0.0)),
    (FCONST_1, stack, stack.push_float(1.0)),
    (FCONST_2, stack, stack.push_float(2.0)),
    (ICONST_M1, stack, stack.push_int(-1)),
    (ICONST_0, stack, stack.push_int(0)),
    (ICONST_1, stack, stack.push_int(1)),
    (ICONST_2, stack, stack.push_int(2)),
    (ICONST_3, stack, stack.push_int(3)),
    (ICONST_4, stack, stack.push_int(4)),
    (ICONST_5, stack, stack.push_int(5)),
    (LCONST_0, stack, stack.push_long(0)),
    (LCONST_1, stack, stack.push_long(1))
}