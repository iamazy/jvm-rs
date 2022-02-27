use std::sync::Arc;
use bitflags::bitflags;
use nom::error::{ErrorKind, VerboseError};
use crate::constant::Constant;

mod class_file;
mod attribute;
mod constant;
mod field;
mod method;
mod errors;

const MAGIC: u32 = 0xCAFEBABE;

type BytesRef = Arc<Vec<u8>>;
type ConstantPoolRef = Arc<Vec<Constant>>;

type IResult<I, O, E = (I, ErrorKind)> = Result<(I, O), NomErr<E>>;
type Res<T, U> = IResult<T, U, VerboseError<T>>;

bitflags! {
    struct AccessFlag: u16 {
        const ACC_PUBLIC = 0x0001;
        const ACC_PRIVATE = 0x0002;
        const ACC_PROTECTED = 0x0004;
        const ACC_STATIC = 0x0008;
        const ACC_FINAL = 0x0010;
        const ACC_SUPER = 0x0020;
        const ACC_SYNCHRONIZED = 0x0020;
        const ACC_BRIDGE = 0x0040;
        const ACC_VOLATILE = 0x0040;
        const ACC_VARARGS = 0x0080;
        const ACC_TRANSIENT = 0x0080;
        const ACC_NATIVE = 0x0100;
        const ACC_INTERFACE = 0x0200;
        const ACC_ABSTRACT = 0x0400;
        const ACC_STRICT = 0x0800;
        const ACC_SYNTHETIC = 0x1000;
        const ACC_ANNOTATION = 0x2000;
        const ACC_ENUM = 0x4000;
    }
}