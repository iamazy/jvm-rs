use crate::instructions::{InstructionExecutor, InstructionReader};
use crate::rtda::Frame;
use jvm_macros::NoOperand;
use std::io::Cursor;

macro_rules! register_math {
    ($(($inst:ident, $pop_fn:ident, $push_fn:ident, $sign:tt)),*) => {
        $(
            #[derive(NoOperand)]
            #[allow(non_camel_case_types)]
            pub struct $inst;

            impl InstructionExecutor for $inst {
                fn execute(&self, frame: &mut Frame) {
                    let val1 = frame.operand_stack().$pop_fn();
                    let val2 = frame.operand_stack().$pop_fn();
                    frame.operand_stack().$push_fn(val1 $sign val2);
                }
            }
        )*
    };
}

register_math! {
    // add
    (LADD, pop_long, push_long, +),
    (IADD, pop_int, push_int, +),
    (FADD, pop_float, push_float, +),
    (DADD, pop_double, push_double, +),

    // sub
    (LSUB, pop_long, push_long, -),
    (ISUB, pop_int, push_int, -),
    (FSUB, pop_float, push_float, -),
    (DSUB, pop_double, push_double, -),

    // and
    (IAND, pop_int, push_int, &),
    (LAND, pop_long, push_long, &),

    // div
    (LDIV, pop_long, push_long, /),
    (IDIV, pop_int, push_int, /),
    (FDIV, pop_float, push_float, /),
    (DDIV, pop_double, push_double, /),

    // mul
    (LMUL, pop_long, push_long, *),
    (IMUL, pop_int, push_int, *),
    (FMUL, pop_float, push_float, *),
    (DMUL, pop_double, push_double, *),

    // or
    (LOR, pop_long, push_long, |),
    (IOR, pop_int, push_int, |),

    // xor
    (LXOR, pop_long, push_long, ^),
    (IXOR, pop_int, push_int, ^)


}

pub(crate) mod iinc;
pub(crate) mod neg;
pub(crate) mod rem;
pub(crate) mod sh;
