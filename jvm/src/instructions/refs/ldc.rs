use crate::instructions::{InstructionExecutor, InstructionReader};
use crate::rtda::{Constant, Frame};
use bytes::Buf;
use jvm_macros::{Index16, Index8};

#[derive(Debug, Default, Index8)]
#[allow(non_camel_case_types)]
pub struct LDC {
    pub index: usize,
}

impl InstructionExecutor for LDC {
    fn execute(&self, frame: &mut Frame) {
        unsafe {
            let constant_pool = frame.method().borrow().class.as_ref().constant_pool;
            let constant = constant_pool.as_ref().get(self.index);
            match constant {
                Constant::Integer(int) => {
                    frame.operand_stack_mut().push_int(*int);
                }
                Constant::Float(float) => {
                    frame.operand_stack_mut().push_float(*float);
                }
                _ => {
                    panic!("ldc: not implemented");
                }
            }
        }
    }
}

#[derive(Debug, Default, Index16)]
#[allow(non_camel_case_types)]
pub struct LDC_W {
    pub index: usize,
}

impl InstructionExecutor for LDC_W {
    fn execute(&self, frame: &mut Frame) {
        unsafe {
            let constant_pool = frame.method().borrow().class.as_ref().constant_pool;
            let constant = constant_pool.as_ref().get(self.index);
            match constant {
                Constant::Integer(int) => {
                    frame.operand_stack_mut().push_int(*int);
                }
                Constant::Float(float) => {
                    frame.operand_stack_mut().push_float(*float);
                }
                _ => {
                    panic!("ldc: not implemented");
                }
            }
        }
    }
}

#[derive(Debug, Default, Index16)]
#[allow(non_camel_case_types)]
pub struct LDC2_W {
    pub index: usize,
}

impl InstructionExecutor for LDC2_W {
    fn execute(&self, frame: &mut Frame) {
        unsafe {
            let constant_pool = frame.method().borrow().class.as_ref().constant_pool;
            let constant = constant_pool.as_ref().get(self.index);
            match constant {
                Constant::Long(long) => {
                    frame.operand_stack_mut().push_long(*long);
                }
                Constant::Double(double) => {
                    frame.operand_stack_mut().push_double(*double);
                }
                _ => {
                    panic!("java.lang.ClassFormatError");
                }
            }
        }
    }
}
