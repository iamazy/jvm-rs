use std::cell::RefCell;
use std::ptr;
use std::sync::Arc;

mod heap;
mod thread;

pub use crate::rtda::thread::Thread;
pub use heap::Object;

#[derive(Debug)]
pub struct Stack {
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
        frame.lower = self.top.take();
        self.top = Some(Box::new(frame));
        self.size += 1;
    }

    pub fn pop(&mut self) -> Option<Box<Frame>> {
        if self.top.is_none() {
            panic!("jvm stack is empty");
        } else {
            self.top.take().map(|mut boxed_frame| {
                self.top = boxed_frame.lower.take();
                self.size -= 1;
                boxed_frame
            })
        }
    }

    pub fn peek(&self) -> Option<&Frame> {
        if self.top.is_none() {
            panic!("jvm stack is empty");
        } else {
            self.top.as_deref()
        }
    }
}

#[derive(Debug)]
pub struct Frame {
    lower: Option<Box<Frame>>,
    local_vars: LocalVars,
    operand_stack: OperandStack,
    thread: Arc<RefCell<Thread>>,
    next_pc: isize,
}

impl Frame {
    pub fn new(thread: Arc<RefCell<Thread>>, max_locals: usize, max_stack: usize) -> Frame {
        Frame {
            lower: None,
            local_vars: LocalVars::new(max_locals),
            operand_stack: OperandStack::new(max_stack),
            thread,
            next_pc: 0,
        }
    }

    pub fn operand_stack_mut(&mut self) -> &mut OperandStack {
        &mut self.operand_stack
    }

    pub fn operand_stack(&self) -> &OperandStack {
        &self.operand_stack
    }

    pub fn local_vars(&self) -> &LocalVars {
        &self.local_vars
    }

    pub fn local_vars_mut(&mut self) -> &mut LocalVars {
        &mut self.local_vars
    }

    pub fn next_pc(&mut self) -> isize {
        self.next_pc
    }

    pub fn set_next_pc(&mut self, next_pc: isize) {
        self.next_pc = next_pc;
    }

    pub fn thread(&mut self) -> Arc<RefCell<Thread>> {
        self.thread.clone()
    }

    pub fn branch(&mut self, offset: i32) {
        let pc = self.thread.borrow_mut().pc();
        let next_pc = pc + offset as isize;
        self.set_next_pc(next_pc);
    }
}

#[derive(Debug, Clone)]
pub struct Slot {
    num: i32,
    r#ref: *mut Object,
}

#[derive(Debug)]
pub struct LocalVars(Vec<Slot>);

impl LocalVars {
    pub fn new(max_locals: usize) -> LocalVars {
        LocalVars(vec![
            Slot {
                num: 0,
                r#ref: ptr::null_mut(),
            };
            max_locals
        ])
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn set_int(&mut self, index: usize, value: i32) {
        self.0[index].num = value;
    }

    pub fn get_int(&self, index: usize) -> i32 {
        self.0[index].num
    }

    pub fn set_float(&mut self, index: usize, value: f32) {
        self.0[index].num = value.to_bits() as i32;
    }

    pub fn get_float(&self, index: usize) -> f32 {
        f32::from_bits(self.0[index].num as u32)
    }

    pub fn set_long(&mut self, index: usize, value: i64) {
        self.0[index].num = value as i32;
        self.0[index + 1].num = (value >> 32) as i32;
    }

    pub fn get_long(&self, index: usize) -> i64 {
        let low = self.0[index].num as u32;
        let high = self.0[index + 1].num as u32;
        ((high as i64) << 32) | (low as i64)
    }

    pub fn set_double(&mut self, index: usize, value: f64) {
        let value = value.to_bits();
        self.set_long(index, value as i64);
    }

    pub fn get_double(&self, index: usize) -> f64 {
        f64::from_bits(self.get_long(index) as u64)
    }

    pub fn set_ref(&mut self, index: usize, value: *mut Object) {
        self.0[index].r#ref = value;
    }

    pub fn get_ref(&self, index: usize) -> *mut Object {
        self.0[index].r#ref
    }
}

#[derive(Debug)]
pub struct OperandStack {
    size: usize,
    slots: Vec<Slot>,
}

impl OperandStack {
    pub fn new(max_stack: usize) -> OperandStack {
        OperandStack {
            size: 0,
            slots: Vec::with_capacity(max_stack),
        }
    }

    pub fn push_int(&mut self, value: i32) {
        self.slots.push(Slot {
            num: value,
            r#ref: ptr::null_mut(),
        });
        self.size += 1;
    }

    pub fn pop_int(&mut self) -> i32 {
        let value = self.slots.pop().unwrap().num;
        self.size -= 1;
        value
    }

    pub fn push_float(&mut self, value: f32) {
        self.push_int(value.to_bits() as i32);
    }

    pub fn pop_float(&mut self) -> f32 {
        f32::from_bits(self.pop_int() as u32)
    }

    pub fn push_long(&mut self, value: i64) {
        self.slots.push(Slot {
            num: (value >> 32) as i32,
            r#ref: ptr::null_mut(),
        });
        self.slots.push(Slot {
            num: value as i32,
            r#ref: ptr::null_mut(),
        });
        self.size += 2;
    }

    pub fn pop_long(&mut self) -> i64 {
        let low = self.slots.pop().unwrap().num as u32;
        let high = self.slots.pop().unwrap().num as u32;
        self.size -= 2;
        ((high as i64) << 32) | (low as i64)
    }

    pub fn push_double(&mut self, value: f64) {
        self.push_long(value.to_bits() as i64);
    }

    pub fn pop_double(&mut self) -> f64 {
        f64::from_bits(self.pop_long() as u64)
    }

    pub fn push_ref(&mut self, value: *mut Object) {
        self.slots.push(Slot {
            num: 0,
            r#ref: value,
        });
        self.size += 1;
    }

    pub fn pop_ref(&mut self) -> *mut Object {
        let value = self.slots.pop().unwrap().r#ref;
        self.size -= 1;
        value
    }

    pub fn push_slot(&mut self, slot: Slot) {
        self.slots.push(slot);
        self.size += 1;
    }

    pub fn pop_slot(&mut self) -> Slot {
        self.size -= 1;
        self.slots.pop().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use crate::rtda::heap::Object;
    use crate::rtda::{Frame, LocalVars, OperandStack, Slot, Stack, Thread};
    use std::cell::RefCell;
    use std::ptr;
    use std::sync::Arc;

    fn stack_init() -> Stack {
        let mut stack = Stack::new(10);
        let thread = Arc::new(RefCell::new(Thread::new()));
        for i in 0..10 {
            let mut local_vars = vec![];
            for j in 0..i {
                let slot = Slot {
                    num: j as i32,
                    r#ref: ptr::null_mut(),
                };
                local_vars.push(slot);
            }
            let frame = Frame {
                lower: None,
                local_vars: LocalVars(local_vars),
                operand_stack: OperandStack::new(0),
                thread: thread.clone(),
                next_pc: 0,
            };
            stack.push(frame);
        }
        stack
    }

    #[test]
    fn test_stack() {
        let mut stack = stack_init();
        for i in 0..10 {
            let frame = stack.pop().unwrap();
            let local_vars = frame.local_vars;
            assert_eq!(local_vars.len(), 10 - i - 1);
            println!("frame's local vars size: {:?}", local_vars.len());
            for i in 0..local_vars.len() {
                let slot = &local_vars.0[i];
                assert_eq!(slot.num, i as i32);
            }
        }
    }

    #[test]
    fn test_peek() {
        let stack = stack_init();
        let mut count = 0;
        while let Some(frame) = stack.peek() {
            assert_eq!(frame.local_vars.len(), 9);
            count += 1;
            if count >= 1000000 {
                break;
            }
        }
    }

    #[test]
    fn test_local_vars() {
        let mut local_var = LocalVars::new(10);
        local_var.set_int(0, 100);
        local_var.set_int(1, -100);
        local_var.set_long(2, 2997924580);
        local_var.set_long(4, -2997924580);
        local_var.set_float(6, std::f64::consts::PI as f32);
        local_var.set_double(7, std::f64::consts::E);
        let object = &mut Object::new(1) as *mut Object;
        local_var.set_ref(9, object);
        assert_eq!(local_var.get_int(0), 100);
        assert_eq!(local_var.get_int(1), -100);
        assert_eq!(local_var.get_long(2), 2997924580);
        assert_eq!(local_var.get_long(4), -2997924580);
        assert_eq!(local_var.get_float(6), std::f64::consts::PI as f32);
        assert_eq!(local_var.get_double(7), std::f64::consts::E);
        assert_eq!(local_var.get_ref(9), object);
    }

    #[test]
    fn test_operand_stack() {
        let mut operand_stack = OperandStack::new(10);
        operand_stack.push_int(100);
        operand_stack.push_int(-100);
        operand_stack.push_long(2997924580);
        operand_stack.push_long(-2997924580);
        operand_stack.push_float(std::f64::consts::PI as f32);
        operand_stack.push_double(std::f64::consts::E);
        let object = &mut Object::new(1) as *mut Object;
        operand_stack.push_ref(object);
        assert_eq!(operand_stack.pop_ref(), object);
        assert_eq!(operand_stack.pop_double(), std::f64::consts::E);
        assert_eq!(operand_stack.pop_float(), std::f64::consts::PI as f32);
        assert_eq!(operand_stack.pop_long(), -2997924580);
        assert_eq!(operand_stack.pop_long(), 2997924580);
        assert_eq!(operand_stack.pop_int(), -100);
        assert_eq!(operand_stack.pop_int(), 100);
    }
}
