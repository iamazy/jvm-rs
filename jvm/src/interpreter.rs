use crate::instructions::new_inst;
use crate::rtda::Thread;
use bytes::Buf;
use classfile::get_utf8;
use std::cell::RefCell;
use std::io::Cursor;
use std::sync::Arc;

pub fn interpret(method_info: &classfile::MethodInfo) {
    if let Some(code) = method_info.code_attribute() {
        let thread = Arc::new(RefCell::new(Thread::new()));
        let max_locals = code.max_locals as usize;
        let max_stack = code.max_stack as usize;
        let frame = Thread::new_frame(thread.clone(), max_locals, max_stack);
        thread.borrow_mut().push_frame(frame);
        loop_interpret(thread, &mut Cursor::new(code.code));
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

fn get_main_method<'a>(cf: &'a classfile::ClassFile) -> Option<&'a classfile::MethodInfo<'a>> {
    for method in cf.methods.iter() {
        let method_name = get_utf8(cf.constant_pool.clone(), method.name_index as usize);
        let method_descriptor =
            get_utf8(cf.constant_pool.clone(), method.descriptor_index as usize);
        if method_name == b"main" && method_descriptor == b"([Ljava/lang/String;)V" {
            return Some(method);
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use crate::classpath::{ClassPath, Entry};
    use crate::interpreter;
    use crate::interpreter::get_main_method;

    #[test]
    #[should_panic]
    fn test_gauss() {
        let class_path = ClassPath::new("".to_string(), "../data/jvm8".to_string());
        if let Ok(class_bytes) = class_path.read_class("GaussTest") {
            if let Ok((_, class_file)) = classfile::parse(class_bytes.as_slice()) {
                if let Some(method) = get_main_method(&class_file) {
                    interpreter::interpret(&method);
                }
            }
        }
    }
}
