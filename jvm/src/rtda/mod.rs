use crate::rtda::object::Object;
use std::cell::RefCell;
use std::rc::Rc;

mod object;
mod thread;

struct Stack {
    max_size: usize,
    size: usize,
    top: Option<Box<Frame>>,
}

impl Stack {
    pub fn new(max_size: usize) -> Stack {
        Box();
        Stack {
            max_size,
            size: 0,
            top: None,
        }
    }
}

struct Frame {
    lower: Option<Box<Frame>>,
    local_vars: LocalVars,
    operand_stack: Option<OperandStack>,
}

struct Slot {
    num: i32,
    r#ref: Option<Object>,
}

type LocalVars = Vec<Slot>;

struct OperandStack {
    size: u32,
    slots: Vec<Slot>,
}
