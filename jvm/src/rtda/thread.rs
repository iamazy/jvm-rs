use crate::rtda::{Frame, Stack};
use std::cell::RefCell;
use std::sync::Arc;

#[derive(Debug)]
pub struct Thread {
    pc: isize,
    stack: Option<Stack>,
}

impl Thread {
    pub fn new() -> Thread {
        Thread {
            pc: 0,
            // TODO: add -Xss to set stack size
            stack: Some(Stack::new(1024)),
        }
    }

    pub fn push_frame(&mut self, frame: Frame) {
        self.stack.as_mut().unwrap().push(frame);
    }

    pub fn pop_frame(&mut self) -> Box<Frame> {
        self.stack.as_mut().unwrap().pop().unwrap()
    }

    pub fn current_frame(&self) -> Option<&Frame> {
        self.stack.as_ref().and_then(Stack::peek)
    }

    pub fn pc(&self) -> isize {
        self.pc
    }

    pub fn set_pc(&mut self, pc: isize) {
        self.pc = pc;
    }

    pub fn new_frame(thread: Arc<RefCell<Thread>>, max_locals: usize, max_stack: usize) -> Frame {
        Frame::new(thread, max_locals, max_stack)
    }
}
