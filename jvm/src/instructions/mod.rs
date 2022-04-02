#![allow(clippy::upper_case_acronyms)]
use crate::rtda::Frame;
use std::fmt::{Debug, Formatter};
use std::io::Cursor;

mod comparisons;
mod constants;
mod control;
mod conversions;
mod extended;
#[macro_use]
mod loads;
mod math;
mod opcode;
mod refs;
mod stack;
mod stores;

#[rustfmt::skip]
pub use {
    comparisons::{
        dcmp::{DCMPG, DCMPL},
        fcmp::{FCMPG, FCMPL},
        ifcond::{IFEQ, IFGE, IFGT, IFLE, IFLT, IFNE},
        lcmp::LCMP,
        IF_ACMPEQ, IF_ACMPNE, 
        IF_ICMPEQ, IF_ICMPGE, IF_ICMPGT, IF_ICMPLE, IF_ICMPLT, IF_ICMPNE,
    },
    constants::{
        ipush::{BIPUSH, SIPUSH},
        r#const::{
            ACONST_NULL, 
            DCONST_0, DCONST_1, 
            FCONST_0, FCONST_1, FCONST_2, 
            ICONST_0, ICONST_1, ICONST_2, ICONST_3, ICONST_4, ICONST_5, ICONST_M1, 
            LCONST_0, LCONST_1,
        },
        NOP,
    },
    control::{
        goto::GOTO,
        switch::{LOOKUP_SWITCH, TABLE_SWITCH},
    },
    conversions::{
        D2F, D2I, D2L, 
        F2D, F2I, F2L, 
        I2B, I2C, I2D, 
        I2F, I2L, I2S, 
        L2D, L2F, L2I},
    extended::{
        goto_w::GOTO_W,
        ifnull::{IFNONNULL, IFNULL},
        wide::WIDE,
    },
    loads::{
        ALOAD, ALOAD_0, ALOAD_1, ALOAD_2, ALOAD_3, 
        DLOAD, DLOAD_0, DLOAD_1, DLOAD_2, DLOAD_3,
        FLOAD, FLOAD_0, FLOAD_1, FLOAD_2, FLOAD_3, 
        ILOAD, ILOAD_0, ILOAD_1, ILOAD_2, ILOAD_3,
        LLOAD, LLOAD_0, LLOAD_1, LLOAD_2, LLOAD_3,
    },
    math::{
        iinc::IINC,
        neg::{DNEG, FNEG, INEG, LNEG},
        rem::{DREM, FREM, IREM, LREM},
        sh::{ISHL, ISHR, IUSHR, LSHL, LSHR, LUSHR},
        DADD, DDIV, DMUL, DSUB, 
        FADD, FDIV, FMUL, FSUB, 
        IADD, IAND, IDIV, IMUL, IOR, ISUB, IXOR,
        LADD, LAND, LDIV, LMUL, LOR, LSUB, LXOR,
    },
    stack::{
        dup::{
            DUP, DUP2, DUP2_X1, 
            DUP2_X2, DUP_X1, DUP_X2
        },
        pop::{POP, POP2},
        swap::SWAP,
    },
    stores::{
        ASTORE, ASTORE_0, ASTORE_1, ASTORE_2, ASTORE_3, 
        DSTORE, DSTORE_0, DSTORE_1, DSTORE_2, DSTORE_3, 
        FSTORE, FSTORE_0, FSTORE_1, FSTORE_2, FSTORE_3, 
        ISTORE, ISTORE_0, ISTORE_1, ISTORE_2, ISTORE_3, 
        LSTORE, LSTORE_0, LSTORE_1, LSTORE_2, LSTORE_3,
    },
};
use crate::instructions::opcode::OpCode;

pub trait InstructionReader<T>
where
    T: AsRef<[u8]>,
{
    fn fetch_operands(&mut self, reader: &mut Cursor<T>);
}

pub trait InstructionExecutor {
    fn execute(&self, frame: &mut Frame);
}

pub trait Instruction<T: AsRef<[u8]>>: Debug + InstructionReader<T> + InstructionExecutor {}

macro_rules! register_inst {
    ($($inst:ident),*) => {
        $(impl<T: AsRef<[u8]>> Instruction<T> for $inst {})*
    };
}

register_inst! {
    // comparisons
    DCMPG, DCMPL,
    FCMPG, FCMPL,
    IF_ACMPEQ, IF_ACMPNE,
    IF_ICMPEQ, IF_ICMPGE, IF_ICMPGT, IF_ICMPLE, IF_ICMPLT, IF_ICMPNE,
    IFEQ, IFGE, IFGT, IFLE, IFLT, IFNE,
    LCMP,
    // constants
    BIPUSH, SIPUSH,
    ACONST_NULL,
    DCONST_0, DCONST_1,
    FCONST_0, FCONST_1, FCONST_2,
    ICONST_0, ICONST_1, ICONST_2, ICONST_3, ICONST_4, ICONST_5, ICONST_M1,
    LCONST_0, LCONST_1,
    NOP,
    // control
    GOTO,
    LOOKUP_SWITCH, TABLE_SWITCH,
    // conversions
    D2F, D2I, D2L,
    F2D, F2I, F2L,
    I2D, I2F, I2L, I2B, I2C, I2S,
    L2D, L2F, L2I,
    // extended
    GOTO_W,
    IFNONNULL, IFNULL,
    // WIDE,
    // loads
    ILOAD, ILOAD_0, ILOAD_1, ILOAD_2, ILOAD_3,
    ALOAD, ALOAD_0, ALOAD_1, ALOAD_2, ALOAD_3,
    DLOAD, DLOAD_0, DLOAD_1, DLOAD_2, DLOAD_3,
    FLOAD, FLOAD_0, FLOAD_1, FLOAD_2, FLOAD_3,
    LLOAD, LLOAD_0, LLOAD_1, LLOAD_2, LLOAD_3,
    // math
    IADD, FADD, DADD, LADD,
    IAND, LAND,
    DDIV, FDIV, IDIV, LDIV,
    DMUL, FMUL, IMUL, LMUL,
    DNEG, FNEG, INEG, LNEG,
    DSUB, FSUB, ISUB, LSUB,
    IOR, LOR,
    IXOR, LXOR,
    IINC,
    DREM, FREM, IREM, LREM,
    ISHL, ISHR, IUSHR, LSHL, LSHR, LUSHR,
    // stack
    DUP, DUP2, DUP2_X1, DUP2_X2, DUP_X1, DUP_X2,
    POP, POP2,
    SWAP,
    // lstore
    ASTORE, ASTORE_0, ASTORE_1, ASTORE_2, ASTORE_3,
    DSTORE, DSTORE_0, DSTORE_1, DSTORE_2, DSTORE_3,
    FSTORE, FSTORE_0, FSTORE_1, FSTORE_2, FSTORE_3,
    ISTORE, ISTORE_0, ISTORE_1, ISTORE_2, ISTORE_3,
    LSTORE, LSTORE_0, LSTORE_1, LSTORE_2, LSTORE_3
}

impl<T: AsRef<[u8]>> Debug for WIDE<T> {
    fn fmt(&self, _f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl<T: AsRef<[u8]>> Instruction<T> for WIDE<T> {}

pub fn new_inst<T: AsRef<[u8]>>(opcode: u8) -> Box<dyn Instruction<T>> {
    match opcode.into() {
        OpCode::nop => Box::new(NOP {}),
        OpCode::aconst_null => Box::new(ACONST_NULL {}),
        OpCode::iconst_m1 => Box::new(ICONST_M1 {}),
        OpCode::iconst_0 => Box::new(ICONST_0 {}),
        OpCode::iconst_1 => Box::new(ICONST_1 {}),
        OpCode::iconst_2 => Box::new(ICONST_2 {}),
        OpCode::iconst_3 => Box::new(ICONST_3 {}),
        OpCode::iconst_4 => Box::new(ICONST_4 {}),
        OpCode::iconst_5 => Box::new(ICONST_5 {}),
        OpCode::lconst_0 => Box::new(LCONST_0 {}),
        OpCode::lconst_1 => Box::new(LCONST_1 {}),
        OpCode::fconst_0 => Box::new(FCONST_0 {}),
        OpCode::fconst_1 => Box::new(FCONST_1 {}),
        OpCode::fconst_2 => Box::new(FCONST_2 {}),
        OpCode::dconst_0 => Box::new(DCONST_0 {}),
        OpCode::dconst_1 => Box::new(DCONST_1 {}),
        OpCode::bipush => Box::new(BIPUSH::default()),
        OpCode::sipush => Box::new(SIPUSH::default()),
        // OpCode::ldc => Box::new(LDC {}),
        // OpCode::ldc_w => Box::new(LDC_W {}),
        // OpCode::ldc2_w => Box::new(LDC2_W {}),
        OpCode::iload => Box::new(ILOAD::default()),
        OpCode::lload => Box::new(LLOAD::default()),
        OpCode::fload => Box::new(FLOAD::default()),
        OpCode::dload => Box::new(DLOAD::default()),
        OpCode::aload => Box::new(ALOAD::default()),
        OpCode::iload_0 => Box::new(ILOAD_0 {}),
        OpCode::iload_1 => Box::new(ILOAD_1 {}),
        OpCode::iload_2 => Box::new(ILOAD_2 {}),
        OpCode::iload_3 => Box::new(ILOAD_3 {}),
        OpCode::lload_0 => Box::new(LLOAD_0 {}),
        OpCode::lload_1 => Box::new(LLOAD_1 {}),
        OpCode::lload_2 => Box::new(LLOAD_2 {}),
        OpCode::lload_3 => Box::new(LLOAD_3 {}),
        OpCode::fload_0 => Box::new(FLOAD_0 {}),
        OpCode::fload_1 => Box::new(FLOAD_1 {}),
        OpCode::fload_2 => Box::new(FLOAD_2 {}),
        OpCode::fload_3 => Box::new(FLOAD_3 {}),
        OpCode::dload_0 => Box::new(DLOAD_0 {}),
        OpCode::dload_1 => Box::new(DLOAD_1 {}),
        OpCode::dload_2 => Box::new(DLOAD_2 {}),
        OpCode::dload_3 => Box::new(DLOAD_3 {}),
        OpCode::aload_0 => Box::new(ALOAD_0 {}),
        OpCode::aload_1 => Box::new(ALOAD_1 {}),
        OpCode::aload_2 => Box::new(ALOAD_2 {}),
        OpCode::aload_3 => Box::new(ALOAD_3 {}),
        // OpCode::iaload => Box::new(IALOAD {}),
        // OpCode::laload => Box::new(LALOAD {}),
        // OpCode::faload => Box::new(FALOAD {}),
        // OpCode::daload => Box::new(DALOAD {}),
        // OpCode::aaload => Box::new(AALOAD {}),
        // OpCode::baload => Box::new(BALOAD {}),
        // OpCode::caload => Box::new(CALOAD {}),
        // OpCode::saload => Box::new(SALOAD {}),
        OpCode::istore => Box::new(ISTORE::default()),
        OpCode::lstore => Box::new(LSTORE::default()),
        OpCode::fstore => Box::new(FSTORE::default()),
        OpCode::dstore => Box::new(DSTORE::default()),
        OpCode::astore => Box::new(ASTORE::default()),
        OpCode::istore_0 => Box::new(ISTORE_0 {}),
        OpCode::istore_1 => Box::new(ISTORE_1 {}),
        OpCode::istore_2 => Box::new(ISTORE_2 {}),
        OpCode::istore_3 => Box::new(ISTORE_3 {}),
        OpCode::lstore_0 => Box::new(LSTORE_0 {}),
        OpCode::lstore_1 => Box::new(LSTORE_1 {}),
        OpCode::lstore_2 => Box::new(LSTORE_2 {}),
        OpCode::lstore_3 => Box::new(LSTORE_3 {}),
        OpCode::fstore_0 => Box::new(FSTORE_0 {}),
        OpCode::fstore_1 => Box::new(FSTORE_1 {}),
        OpCode::fstore_2 => Box::new(FSTORE_2 {}),
        OpCode::fstore_3 => Box::new(FSTORE_3 {}),
        OpCode::dstore_0 => Box::new(DSTORE_0 {}),
        OpCode::dstore_1 => Box::new(DSTORE_1 {}),
        OpCode::dstore_2 => Box::new(DSTORE_2 {}),
        OpCode::dstore_3 => Box::new(DSTORE_3 {}),
        OpCode::astore_0 => Box::new(ASTORE_0 {}),
        OpCode::astore_1 => Box::new(ASTORE_1 {}),
        OpCode::astore_2 => Box::new(ASTORE_2 {}),
        OpCode::astore_3 => Box::new(ASTORE_3 {}),
        // OpCode::iastore => Box::new(IASTORE {}),
        // OpCode::lastore => Box::new(LASTORE {}),
        // OpCode::fastore => Box::new(FASTORE {}),
        // OpCode::dastore => Box::new(DASTORE {}),
        // OpCode::aastore => Box::new(AASTORE {}),
        // OpCode::bastore => Box::new(BASTORE {}),
        // OpCode::castore => Box::new(CASTORE {}),
        // OpCode::sastore => Box::new(SASTORE {}),
        OpCode::pop => Box::new(POP {}),
        OpCode::pop2 => Box::new(POP2 {}),
        OpCode::dup => Box::new(DUP {}),
        OpCode::dup_x1 => Box::new(DUP_X1 {}),
        OpCode::dup_x2 => Box::new(DUP_X2 {}),
        OpCode::dup2 => Box::new(DUP2 {}),
        OpCode::dup2_x1 => Box::new(DUP2_X1 {}),
        OpCode::dup2_x2 => Box::new(DUP2_X2 {}),
        OpCode::swap => Box::new(SWAP {}),
        OpCode::iadd => Box::new(IADD {}),
        OpCode::ladd => Box::new(LADD {}),
        OpCode::fadd => Box::new(FADD {}),
        OpCode::dadd => Box::new(DADD {}),
        OpCode::isub => Box::new(ISUB {}),
        OpCode::lsub => Box::new(LSUB {}),
        OpCode::fsub => Box::new(FSUB {}),
        OpCode::dsub => Box::new(DSUB {}),
        OpCode::imul => Box::new(IMUL {}),
        OpCode::lmul => Box::new(LMUL {}),
        OpCode::fmul => Box::new(FMUL {}),
        OpCode::dmul => Box::new(DMUL {}),
        OpCode::idiv => Box::new(IDIV {}),
        OpCode::ldiv => Box::new(LDIV {}),
        OpCode::fdiv => Box::new(FDIV {}),
        OpCode::ddiv => Box::new(DDIV {}),
        OpCode::irem => Box::new(IREM {}),
        OpCode::lrem => Box::new(LREM {}),
        OpCode::frem => Box::new(FREM {}),
        OpCode::drem => Box::new(DREM {}),
        OpCode::ineg => Box::new(INEG {}),
        OpCode::lneg => Box::new(LNEG {}),
        OpCode::fneg => Box::new(FNEG {}),
        OpCode::dneg => Box::new(DNEG {}),
        OpCode::ishl => Box::new(ISHL {}),
        OpCode::lshl => Box::new(LSHL {}),
        OpCode::ishr => Box::new(ISHR {}),
        OpCode::lshr => Box::new(LSHR {}),
        OpCode::iushr => Box::new(IUSHR {}),
        OpCode::lushr => Box::new(LUSHR {}),
        OpCode::iand => Box::new(IAND {}),
        OpCode::land => Box::new(LAND {}),
        OpCode::ior => Box::new(IOR {}),
        OpCode::lor => Box::new(LOR {}),
        OpCode::ixor => Box::new(IXOR {}),
        OpCode::lxor => Box::new(LXOR {}),
        OpCode::iinc => Box::new(IINC::default()),
        OpCode::i2l => Box::new(I2L {}),
        OpCode::i2f => Box::new(I2F {}),
        OpCode::i2d => Box::new(I2D {}),
        OpCode::l2i => Box::new(L2I {}),
        OpCode::l2f => Box::new(L2F {}),
        OpCode::l2d => Box::new(L2D {}),
        OpCode::f2i => Box::new(F2I {}),
        OpCode::f2l => Box::new(F2L {}),
        OpCode::f2d => Box::new(F2D {}),
        OpCode::d2i => Box::new(D2I {}),
        OpCode::d2l => Box::new(D2L {}),
        OpCode::d2f => Box::new(D2F {}),
        OpCode::i2b => Box::new(I2B {}),
        OpCode::i2c => Box::new(I2C {}),
        OpCode::i2s => Box::new(I2S {}),
        OpCode::lcmp => Box::new(LCMP {}),
        OpCode::fcmpl => Box::new(FCMPL {}),
        OpCode::fcmpg => Box::new(FCMPG {}),
        OpCode::dcmpl => Box::new(DCMPL {}),
        OpCode::dcmpg => Box::new(DCMPG {}),
        OpCode::ifeq => Box::new(IFEQ::default()),
        OpCode::ifne => Box::new(IFNE::default()),
        OpCode::iflt => Box::new(IFLT::default()),
        OpCode::ifge => Box::new(IFGE::default()),
        OpCode::ifgt => Box::new(IFGT::default()),
        OpCode::ifle => Box::new(IFLE::default()),
        OpCode::if_icmpeq => Box::new(IF_ICMPEQ::default()),
        OpCode::if_icmpne => Box::new(IF_ICMPNE::default()),
        OpCode::if_icmplt => Box::new(IF_ICMPLT::default()),
        OpCode::if_icmpge => Box::new(IF_ICMPGE::default()),
        OpCode::if_icmpgt => Box::new(IF_ICMPGT::default()),
        OpCode::if_icmple => Box::new(IF_ICMPLE::default()),
        OpCode::if_acmpeq => Box::new(IF_ACMPEQ::default()),
        OpCode::if_acmpne => Box::new(IF_ACMPNE::default()),
        OpCode::goto => Box::new(GOTO::default()),
        // OpCode::jsr => Box::new(JSR {}),
        // OpCode::ret => Box::new(RET {}),
        OpCode::tableswitch => Box::new(TABLE_SWITCH::default()),
        OpCode::lookupswitch => Box::new(LOOKUP_SWITCH::default()),
        // OpCode::ireturn => Box::new(IRETURN {}),
        // OpCode::lreturn => Box::new(LRETURN {}),
        // OpCode::freturn => Box::new(FRETURN {}),
        // OpCode::dreturn => Box::new(DRETURN {}),
        // OpCode::areturn => Box::new(ARETURN {}),
        // OpCode::vreturn => Box::new(VRETURN {}),
        // OpCode::getstatic => Box::new(GETSTATIC {}),
        // OpCode::putstatic => Box::new(PUTSTATIC {}),
        // OpCode::getfield => Box::new(GETFIELD {}),
        // OpCode::putfield => Box::new(PUTFIELD {}),
        // OpCode::invokevirtual => Box::new(INVOKEVIRTUAL {}),
        // OpCode::invokespecial => Box::new(INVOKESPECIAL {}),
        // OpCode::invokestatic => Box::new(INVOKESTATIC {}),
        // OpCode::invokeinterface => Box::new(INVOKEINTERFACE {}),
        // OpCode::invokedynamic => Box::new(INVOKEDYNAMIC {}),
        // OpCode::new => Box::new(NEW {}),
        // OpCode::newarray => Box::new(NEWARRAY {}),
        // OpCode::anewarray => Box::new(ANEWARRAY {}),
        // OpCode::arraylength => Box::new(ARRAYLENGTH {}),
        // OpCode::athrow => Box::new(ATHROW {}),
        // OpCode::checkcast => Box::new(CHECKCAST {}),
        // OpCode::instanceof => Box::new(INSTANCEOF {}),
        // OpCode::monitorenter => Box::new(MONITORENTER {}),
        // OpCode::monitorexit => Box::new(MONITOREXIT {}),
        // OpCode::wide => Box::new(WIDE::default()),
        // OpCode::multianewarray => Box::new(MULTIANEWARRAY {}),
        OpCode::ifnull => Box::new(IFNULL::default()),
        OpCode::ifnonnull => Box::new(IFNONNULL::default()),
        OpCode::goto_w => Box::new(GOTO_W::default()),
        // OpCode::jsr_w => Box::new(JSR_W {}),
        // OpCode::breakpoint => Box::new(BREAKPOINT {}),
        // OpCode::impdep1 => Box::new(IMPDEP1 {}),
        // OpCode::impdep2 => Box::new(IMPDEP2 {}),
        _ => panic!("Unsupported opcode: {}", opcode),
    }
}
