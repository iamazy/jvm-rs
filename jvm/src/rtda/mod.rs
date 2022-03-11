use crate::rtda::object::Object;
use std::mem;
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
        Stack {
            max_size,
            size: 0,
            top: None,
        }
    }

    pub fn push(&mut self, mut frame: Frame) {
        if self.size >= self.max_size {
            panic!("java.lang.StackOverflowError");
        }
        frame.lower = mem::replace(&mut self.top, None);
        self.top = Some(Box::new(frame));
        self.size += 1;
    }

    pub fn pop(&mut self) -> Option<Box<Frame>> {
        if self.top.is_none() {
            None
        } else {
            match mem::replace(&mut self.top, None) {
                None => None,
                Some(mut frame) => {
                    let lower = mem::replace(&mut frame.lower, None);
                    self.top = lower;
                    self.size -= 1;
                    Some(frame)
                }
            }
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
