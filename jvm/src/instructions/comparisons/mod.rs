use crate::rtda::Frame;

macro_rules! register_if_cmp {
    ($(($inst:ident, $func:ident, | $val1:ident, $val2:ident| $expr:expr)),*) => {
        $(
            #[derive(Branch)]
            #[allow(non_camel_case_types)]
            pub struct $inst {
                offset: i32,
            }

            impl InstructionExecutor for $inst {
                fn execute(&self, frame: &mut Frame) {
                    let $val2 = frame.operand_stack().$func();
                    let $val1 = frame.operand_stack().$func();
                    if $expr {
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
                let v2 = frame.operand_stack().$func();
                let v1 = frame.operand_stack().$func();
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
                frame.operand_stack().push_int(value);
            }
        )*
    }
}

fn_cmp! {
    (dcmp, pop_double),
    (fcmp, pop_float)
}

pub(crate) mod dcmp;
pub(crate) mod fcmp;
pub(crate) mod if_acmp;
pub(crate) mod if_icmp;
pub(crate) mod ifcond;
pub(crate) mod lcmp;
