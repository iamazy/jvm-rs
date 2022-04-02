use crate::instructions::{InstructionExecutor, InstructionReader};
use crate::rtda::Frame;
use jvm_macros::NoOperand;

macro_rules! register_conversion {
    ($(($inst:ident, $pop_fn:ident, $push_fn:ident, $typ:ty)),*) => {
        $(
            #[derive(NoOperand, Debug)]
            #[allow(non_camel_case_types)]
            pub struct $inst;

            impl InstructionExecutor for $inst {
                fn execute(&self, frame: &mut Frame) {
                    let val = frame.operand_stack_mut().$pop_fn();
                    frame.operand_stack_mut().$push_fn(val as $typ);
                }
            }
        )*
    };
    ($(($inst:ident, $pop_fn:ident, $push_fn:ident, $typ1:ty, $typ2:ty)),*) => {
        $(
            #[derive(NoOperand, Debug)]
            #[allow(non_camel_case_types)]
            pub struct $inst;

            impl InstructionExecutor for $inst {
                fn execute(&self, frame: &mut Frame) {
                    let val = frame.operand_stack_mut().$pop_fn() as $typ1;
                    frame.operand_stack_mut().$push_fn(val as $typ2);
                }
            }
        )*
    };
}

register_conversion! {
    (I2B, pop_int, push_int, i8, i32),
    (I2C, pop_int, push_int, u16, i32),
    (I2S, pop_int, push_int, i16, i32)
}

register_conversion! {
    // d2x
    (D2L, pop_double, push_long, i64),
    (D2I, pop_double, push_int, i32),
    (D2F, pop_double, push_float, f32),

    // f2x
    (F2L, pop_float, push_long, i64),
    (F2I, pop_float, push_int, i32),
    (F2D, pop_float, push_double, f64),

    // i2x
    (I2L, pop_int, push_long, i64),
    (I2F, pop_int, push_float, f32),
    (I2D, pop_int, push_double, f64),

    // l2x
    (L2I, pop_long, push_int, i32),
    (L2F, pop_long, push_float, f32),
    (L2D, pop_long, push_double, f64)

}
