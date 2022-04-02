use crate::instructions::new_inst;
use crate::rtda::{Class, Method, Thread};
use bytes::Buf;

use std::cell::RefCell;
use std::io::Cursor;
use std::sync::Arc;

pub fn interpret(method: Arc<RefCell<Method>>) {
    if let Some(code) = method.borrow().code() {
        let thread = Arc::new(RefCell::new(Thread::new()));
        let frame = Thread::new_frame(thread.clone(), method.clone());
        thread.borrow_mut().push_frame(frame);
        loop_interpret(thread, &mut Cursor::new(code));
    }
}

fn loop_interpret<T: AsRef<[u8]>>(thread: Arc<RefCell<Thread>>, cursor: &mut Cursor<T>) {
    let mut frame = thread.borrow_mut().pop_frame();
    loop {
        let pc = frame.next_pc();
        thread.borrow_mut().set_pc(pc);
        cursor.set_position(pc as u64);
        let opcode = cursor.get_u8();
        let mut inst = new_inst(opcode);
        inst.fetch_operands(cursor);
        frame.set_next_pc(cursor.position() as isize);
        println!("pc: {} inst: {:?}", pc, inst);
        println!(
            "local_vars: {:?}, operand_stack: {:?}",
            frame.local_vars(),
            frame.operand_stack()
        );
        inst.execute(&mut frame);
    }
}

fn get_main_method(class: &Class) -> Option<Arc<RefCell<Method>>> {
    for method in class.methods.iter() {
        if method.borrow().name.as_str() == "main"
            && method.borrow().descriptor.as_str() == "([Ljava/lang/String;)V"
        {
            return Some(method.clone());
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use crate::classpath::ClassPath;
    use crate::interpreter;
    use crate::interpreter::get_main_method;
    use crate::rtda::ClassLoader;

    #[test]
    #[should_panic]
    fn test_gauss() {
        let class_path = ClassPath::new("".to_string(), "../data/jvm8".to_string());
        let class_loader = ClassLoader::new(class_path);
        let class = class_loader.load_class("GaussTest").unwrap();
        if let Some(method) = get_main_method(class) {
            interpreter::interpret(method.clone());
        }
    }
}
