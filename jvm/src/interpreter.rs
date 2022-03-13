use crate::instructions::new_inst;
use crate::rtda::Thread;
use bytes::Buf;
use std::cell::RefCell;
use std::io::Cursor;
use std::sync::Arc;

fn interpret(method_info: &classfile::MethodInfo) {
    if let Some(code) = method_info.code_attribute() {
        let thread = Arc::new(RefCell::new(Thread::new()));
        let max_locals = code.max_locals as usize;
        let max_stack = code.max_stack as usize;
        let frame = Thread::new_frame(thread.clone(), max_locals, max_stack);
        thread.borrow_mut().push_frame(frame);
        loop_interpret(thread, &mut Cursor::new(code.code.as_slice()));
    }
}

fn loop_interpret<T: AsRef<[u8]>>(thread: Arc<RefCell<Thread>>, cursor: &mut Cursor<T>) {
    let mut frame = thread.borrow_mut().pop_frame();
    loop {
        let pc = frame.next_pc();
        thread.borrow_mut().set_pc(pc);
        let opcode = cursor.get_u8();
        let mut inst = new_inst(opcode);
        inst.fetch_operands(cursor);
        frame.set_next_pc(cursor.position() as usize);
        inst.execute(&mut frame);
    }
}
