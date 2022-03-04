use crate::{BytesRef, NomErr, Res};
use nom::bytes::complete::take;
use nom::error::{context, VerboseError, VerboseErrorKind};
use nom::number::complete::{be_f32, be_f64, be_i32, be_i64, be_u16, be_u8};

#[derive(Debug, Clone)]
pub enum Constant {
    Class {
        name_index: u16,
    },
    FieldRef {
        class_index: u16,
        name_and_type_index: u16,
    },
    MethodRef {
        class_index: u16,
        name_and_type_index: u16,
    },
    InterfaceMethodRef {
        class_index: u16,
        name_and_type_index: u16,
    },
    String {
        string_index: u16,
    },
    Integer(i32),
    Float(f32),
    Long(i64),
    Double(f64),
    NameAndType {
        name_index: u16,
        descriptor_index: u16,
    },
    Utf8(BytesRef),
    MethodHandle {
        reference_kind: u8,
        reference_index: u16,
    },
    MethodType {
        descriptor_index: u16,
    },
    Dynamic {
        bootstrap_method_attr_index: u16,
        name_and_type_index: u16,
    },
    InvokeDynamic {
        bootstrap_method_attr_index: u16,
        name_and_type_index: u16,
    },
    Module {
        name_index: u16,
    },
    Package {
        name_index: u16,
    },
}

pub fn constant(input: &[u8]) -> Res<&[u8], Constant> {
    context("constant", tag)(input).and_then(|(next_input, tag)| match tag {
        ConstantTag::Class => {
            let (next_input, name_index) = be_u16(next_input)?;
            Ok((next_input, Constant::Class { name_index }))
        }
        ConstantTag::FieldRef => {
            let (next_input, class_index) = be_u16(next_input)?;
            let (next_input, name_and_type_index) = be_u16(next_input)?;
            Ok((
                next_input,
                Constant::FieldRef {
                    class_index,
                    name_and_type_index,
                },
            ))
        }
        ConstantTag::MethodRef => {
            let (next_input, class_index) = be_u16(next_input)?;
            let (next_input, name_and_type_index) = be_u16(next_input)?;
            Ok((
                next_input,
                Constant::MethodRef {
                    class_index,
                    name_and_type_index,
                },
            ))
        }
        ConstantTag::InterfaceMethodRef => {
            let (next_input, class_index) = be_u16(next_input)?;
            let (next_input, name_and_type_index) = be_u16(next_input)?;
            Ok((
                next_input,
                Constant::InterfaceMethodRef {
                    class_index,
                    name_and_type_index,
                },
            ))
        }
        ConstantTag::String => {
            let (next_input, string_index) = be_u16(next_input)?;
            Ok((next_input, Constant::String { string_index }))
        }
        ConstantTag::Integer => {
            let (next_input, value) = be_i32(next_input)?;
            Ok((next_input, Constant::Integer(value as i32)))
        }
        ConstantTag::Float => {
            let (next_input, value) = be_f32(next_input)?;
            Ok((next_input, Constant::Float(value)))
        }
        ConstantTag::Long => {
            let (next_input, value) = be_i64(next_input)?;
            Ok((next_input, Constant::Long(value)))
        }
        ConstantTag::Double => {
            let (next_input, value) = be_f64(next_input)?;
            Ok((next_input, Constant::Double(value)))
        }
        ConstantTag::NameAndType => {
            let (next_input, name_index) = be_u16(next_input)?;
            let (next_input, descriptor_index) = be_u16(next_input)?;
            Ok((
                next_input,
                Constant::NameAndType {
                    name_index,
                    descriptor_index,
                },
            ))
        }
        ConstantTag::Utf8 => {
            let (next_input, length) = be_u16(next_input)?;
            let (next_input, bytes) = take(length)(next_input)?;
            Ok((next_input, Constant::Utf8(BytesRef::new(bytes.to_vec()))))
        }
        ConstantTag::MethodHandle => {
            let (next_input, reference_kind) = be_u8(next_input)?;
            let (next_input, reference_index) = be_u16(next_input)?;
            Ok((
                next_input,
                Constant::MethodHandle {
                    reference_kind,
                    reference_index,
                },
            ))
        }
        ConstantTag::MethodType => {
            let (next_input, descriptor_index) = be_u16(next_input)?;
            Ok((next_input, Constant::MethodType { descriptor_index }))
        }
        ConstantTag::Dynamic => {
            let (next_input, bootstrap_method_attr_index) = be_u16(next_input)?;
            let (next_input, name_and_type_index) = be_u16(next_input)?;
            Ok((
                next_input,
                Constant::Dynamic {
                    bootstrap_method_attr_index,
                    name_and_type_index,
                },
            ))
        }
        ConstantTag::InvokeDynamic => {
            let (next_input, bootstrap_method_attr_index) = be_u16(next_input)?;
            let (next_input, name_and_type_index) = be_u16(next_input)?;
            Ok((
                next_input,
                Constant::InvokeDynamic {
                    bootstrap_method_attr_index,
                    name_and_type_index,
                },
            ))
        }
        ConstantTag::Module => {
            let (next_input, name_index) = be_u16(next_input)?;
            Ok((next_input, Constant::Module { name_index }))
        }
        ConstantTag::Package => {
            let (next_input, name_index) = be_u16(next_input)?;
            Ok((next_input, Constant::Package { name_index }))
        }
    })
}

#[derive(Debug, Clone)]
pub enum ConstantTag {
    Class,
    FieldRef,
    MethodRef,
    InterfaceMethodRef,
    String,
    Integer,
    Float,
    Long,
    Double,
    NameAndType,
    Utf8,
    MethodHandle,
    MethodType,
    Dynamic,
    InvokeDynamic,
    Module,
    Package,
}

pub fn tag(input: &[u8]) -> Res<&[u8], ConstantTag> {
    context("tag", be_u8)(input).and_then(|(next_input, tag)| match tag {
        7 => Ok((next_input, ConstantTag::Class)),
        9 => Ok((next_input, ConstantTag::FieldRef)),
        10 => Ok((next_input, ConstantTag::MethodRef)),
        11 => Ok((next_input, ConstantTag::InterfaceMethodRef)),
        8 => Ok((next_input, ConstantTag::String)),
        3 => Ok((next_input, ConstantTag::Integer)),
        4 => Ok((next_input, ConstantTag::Float)),
        5 => Ok((next_input, ConstantTag::Long)),
        6 => Ok((next_input, ConstantTag::Double)),
        12 => Ok((next_input, ConstantTag::NameAndType)),
        1 => Ok((next_input, ConstantTag::Utf8)),
        15 => Ok((next_input, ConstantTag::MethodHandle)),
        16 => Ok((next_input, ConstantTag::MethodType)),
        17 => Ok((next_input, ConstantTag::Dynamic)),
        18 => Ok((next_input, ConstantTag::InvokeDynamic)),
        19 => Ok((next_input, ConstantTag::Module)),
        20 => Ok((next_input, ConstantTag::Package)),
        _ => Err(NomErr::Error(VerboseError {
            errors: vec![(input, VerboseErrorKind::Context("invalid tag"))],
        })),
    })
}
