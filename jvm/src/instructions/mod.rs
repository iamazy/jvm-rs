use crate::rtda::Frame;
use std::io::Cursor;

mod comparisons;
mod constants;
mod control;
mod conversions;
mod extended;
mod loads;
mod math;
mod opcode;
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

pub trait InstructionReader<T>
where
    T: AsRef<[u8]>,
{
    fn fetch_operands(&mut self, reader: &mut Cursor<T>);
}

pub trait InstructionExecutor {
    fn execute(&self, frame: &mut Frame);
}

pub trait Instruction<T: AsRef<[u8]>>: InstructionReader<T> + InstructionExecutor {}

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
    WIDE,
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

pub fn new_inst<T: AsRef<[u8]>>(opcode: u8) -> Box<dyn Instruction<T>> {
    match opcode {
        0x00 => Box::new(NOP {}),
        _ => unimplemented!("implement me"),
    }
}
