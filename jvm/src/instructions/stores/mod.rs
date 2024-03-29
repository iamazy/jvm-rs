use crate::instructions::{InstructionExecutor, InstructionReader};
use crate::rtda::Frame;
use bytes::Buf;
use jvm_macros::{Index8, NoOperand};

macro_rules! register_store_fn {
    ($(($fn_name:ident, $pop_fn:ident, $set_fn:ident)),*) => {
        $(
            fn $fn_name(frame: &mut Frame, index: usize) {
                let val = frame.operand_stack_mut().$pop_fn();
                frame.local_vars_mut().$set_fn(index, val);
            }
        )*
    };
}

register_store_fn! {
    (astore, pop_ref, set_ref),
    (dstore, pop_double, set_double),
    (fstore, pop_float, set_float),
    (istore, pop_int, set_int),
    (lstore, pop_long, set_long)
}

register_load_store! {
    (ASTORE, astore),
    (DSTORE, dstore),
    (FSTORE, fstore),
    (ISTORE, istore),
    (LSTORE, lstore)
}

register_load_store! {
    // astore
    (ASTORE_0, astore, 0),
    (ASTORE_1, astore, 1),
    (ASTORE_2, astore, 2),
    (ASTORE_3, astore, 3),

    // dstore
    (DSTORE_0, dstore, 0),
    (DSTORE_1, dstore, 1),
    (DSTORE_2, dstore, 2),
    (DSTORE_3, dstore, 3),

    // fstore
    (FSTORE_0, fstore, 0),
    (FSTORE_1, fstore, 1),
    (FSTORE_2, fstore, 2),
    (FSTORE_3, fstore, 3),

    // istore
    (ISTORE_0, istore, 0),
    (ISTORE_1, istore, 1),
    (ISTORE_2, istore, 2),
    (ISTORE_3, istore, 3),

    // lstore
    (LSTORE_0, lstore, 0),
    (LSTORE_1, lstore, 1),
    (LSTORE_2, lstore, 2),
    (LSTORE_3, lstore, 3)
}
