use crate::instructions::{InstructionExecutor, InstructionReader};
use crate::rtda::Frame;
use bytes::Buf;
use jvm_macros::Branch;

macro_rules! register_ifcond {
    ($(($inst:ident, $sign:tt)),*) => {
        $(
            #[derive(Branch, Default, Debug)]
            #[allow(non_camel_case_types)]
            pub struct $inst {
                offset: i32,
            }

            impl InstructionExecutor for $inst {
                fn execute(&self, frame: &mut Frame) {
                    let val = frame.operand_stack_mut().pop_int();
                    if val $sign 0 {
                        frame.branch(self.offset);
                    }
                }
            }
        )*
    };
}

register_ifcond! {
    (IFEQ, ==),
    (IFNE, !=),
    (IFLE, <=),
    (IFLT, <),
    (IFGE, >=),
    (IFGT, >)
}
