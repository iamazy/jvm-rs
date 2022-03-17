use crate::instructions::{InstructionExecutor, InstructionReader};
use crate::rtda::Frame;
use bytes::Buf;
use jvm_macros::{Index8, NoOperand};
use std::io::Cursor;

macro_rules! register_load_fn {
    ($(($fn_name:ident, $get_fn:ident, $push_fn:ident)),*) => {
        $(
            fn $fn_name(frame: &mut Frame, index: usize) {
                let val = frame.local_vars().$get_fn(index);
                frame.operand_stack().$push_fn(val);
            }
        )*
    };
}

macro_rules! register_load {
    ($(($load:ident, $load_fn:ident, $val:literal)),*) => {
        $(
            #[derive(NoOperand)]
            #[allow(non_camel_case_types)]
            pub struct $load;

            impl InstructionExecutor for $load {
                fn execute(&self, frame: &mut Frame) {
                    $load_fn(frame, $val);
                }
            }
        )*
    };
    ($(($load:ident, $load_fn:ident)),*) => {
        $(
            #[derive(Index8)]
            #[allow(non_camel_case_types)]
            pub struct $load {
                index: u32,
            }

            impl $load {
                #[inline]
                pub const fn new(index: u32) -> Self {
                    Self { index }
                }
            }

            impl InstructionExecutor for $load {
                fn execute(&self, frame: &mut Frame) {
                    $load_fn(frame, self.index as usize);
                }
            }
        )*
    };
}

register_load_fn! {
    (aload, get_ref, push_ref),
    (dload, get_double, push_double),
    (fload, get_float, push_float),
    (iload, get_int, push_int),
    (lload, get_long, push_long)
}

register_load! {
    (ALOAD, aload),
    (DLOAD, dload),
    (FLOAD, fload),
    (ILOAD, iload),
    (LLOAD, lload)
}

register_load! {
    (ALOAD_0, aload, 0),
    (ALOAD_1, aload, 1),
    (ALOAD_2, aload, 2),
    (ALOAD_3, aload, 3),

    (DLOAD_0, dload, 0),
    (DLOAD_1, dload, 1),
    (DLOAD_2, dload, 2),
    (DLOAD_3, dload, 3),

    (FLOAD_0, fload, 0),
    (FLOAD_1, fload, 1),
    (FLOAD_2, fload, 2),
    (FLOAD_3, fload, 3),

    (ILOAD_0, iload, 0),
    (ILOAD_1, iload, 1),
    (ILOAD_2, iload, 2),
    (ILOAD_3, iload, 3),

    (LLOAD_0, lload, 0),
    (LLOAD_1, lload, 1),
    (LLOAD_2, lload, 2),
    (LLOAD_3, lload, 3)

}
