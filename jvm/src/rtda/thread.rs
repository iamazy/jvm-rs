use crate::rtda::{Frame, Stack};

struct Thread {
    pc: usize,
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

    pub fn pop_frame(&mut self) -> Option<Box<Frame>> {
        self.stack.as_mut().unwrap().pop()
    }

    pub fn current_frame(&self) -> Option<&Frame> {
        self.stack.as_ref().and_then(Stack::peek)
    }
}
