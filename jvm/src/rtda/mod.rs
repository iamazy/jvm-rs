use std::cell::RefCell;
use std::rc::Rc;
use crate::rtda::object::Object;

mod object;
mod thread;

struct Frame {
    lower: RefCell<Frame>,
    local_vars: LocalVars,
    operand_stack: RefCell<OperandStack>
}

struct Slot {
    num: i32,
    r#ref: RefCell<Object>
}

type LocalVars = Vec<Slot>;

struct OperandStack {
    size: u32,
    slots: Vec<Slot>
}