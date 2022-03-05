use crate::attribute::{
    Annotation, Attribute, AttributeType, BootstrapMethod, CodeAttribute, Element, ElementValue,
    Exception, InnerClass, LineNumber, LocalVar, LocalVariable, LocalVariableType, MethodParameter,
    ParameterAnnotation, StackMap, StackMapFrame, TargetInfo, TypeAnnotation, TypePath,
    VerificationTypeInfo,
};
use crate::class_file::ClassFile;
use crate::constant::{Constant, ConstantTag};
use crate::field::FieldInfo;
use crate::method::MethodInfo;
use bitflags::bitflags;
use nom::bytes::complete::take;
use nom::combinator::{all_consuming, map, success, verify};
use nom::error::{context, ErrorKind, ParseError, VerboseError, VerboseErrorKind};
use nom::multi::length_count;
use nom::number::complete::{be_f32, be_f64, be_i16, be_i32, be_i64, be_u16, be_u32, be_u8};
use nom::sequence::{pair, tuple};
use nom::Err as NomErr;
use std::rc::Rc;

mod attribute;
mod class_file;
mod constant;
mod errors;
mod field;
mod method;

const MAGIC: u32 = 0xCAFEBABE;

type BytesRef = Rc<Vec<u8>>;
type ConstantPoolRef = Rc<Vec<Constant>>;

type IResult<I, O, E = (I, ErrorKind)> = Result<(I, O), NomErr<E>>;
type Res<T, U> = IResult<T, U, VerboseError<T>>;

fn get_utf8(constant_pool: ConstantPoolRef, index: usize) -> BytesRef {
    match constant_pool.get(index - 1) {
        Some(Constant::Utf8(bytes)) => bytes.clone(),
        _ => unreachable!("constant pool index mismatch"),
    }
}

pub fn parse(input: &[u8]) -> Res<&[u8], ClassFile> {
    context("class file", all_consuming(class_file))(input)
}

fn class_file(input: &[u8]) -> Res<&[u8], ClassFile> {
    context(
        "class file",
        tuple((
            verify(be_u32, |magic| *magic == MAGIC),
            be_u16,
            be_u16,
            length_count(map(be_u16, |n| n - 1), constant),
            be_u16,
            be_u16,
            be_u16,
            length_count(be_u16, constant),
        )),
    )(input)
    .and_then(
        |(
            input,
            (
                magic,
                minor_version,
                major_version,
                constant_pool,
                access_flags,
                this_class,
                super_class,
                interfaces,
            ),
        )| {
            let constant_pool = Rc::new(constant_pool);
            let (input, fields) = length_count(be_u16, field_info(constant_pool.clone()))(input)?;
            let (input, methods) = length_count(be_u16, method_info(constant_pool.clone()))(input)?;
            let (input, attributes) =
                length_count(be_u16, attribute(constant_pool.clone()))(input)?;

            Ok((
                input,
                ClassFile {
                    magic,
                    minor_version,
                    major_version,
                    constant_pool,
                    access_flags,
                    this_class,
                    super_class,
                    interfaces,
                    fields,
                    methods,
                    attributes,
                },
            ))
        },
    )
}

fn attribute<'a, E: ParseError<&'a [u8]>>(
    constant_pool: ConstantPoolRef,
) -> impl FnMut(&'a [u8]) -> IResult<&'a [u8], Attribute, E>
where
    nom::Err<E>: From<nom::Err<VerboseError<&'a [u8]>>>,
{
    move |input: &'a [u8]| {
        let (input, attribute_name_index) = be_u16(input)?;
        let (input, attribute_length) = be_u32(input)?;
        let attribute_name = get_utf8(constant_pool.clone(), attribute_name_index as usize);
        let attribute_name = attribute_name.as_slice();
        let (input, attr_type) = match attribute_name {
            b"ConstantValue" => {
                let (input, constant_value_index) = be_u16(input)?;
                (
                    input,
                    AttributeType::ConstantValue {
                        constant_value_index,
                    },
                )
            }
            b"Code" => {
                let (input, code) = code_attribute(constant_pool.clone())(input)?;
                (input, AttributeType::Code { code })
            }
            b"StackMapTable" => {
                let (input, entries) = length_count(be_u16, stack_map)(input)?;
                (input, AttributeType::StackMapTable { entries })
            }
            b"Exceptions" => {
                let (input, exception_index_table) = length_count(be_u16, be_u16)(input)?;
                (
                    input,
                    AttributeType::Exceptions {
                        exception_index_table,
                    },
                )
            }
            b"InnerClasses" => {
                let (input, classes) = length_count(be_u16, inner_class)(input)?;
                (input, AttributeType::InnerClasses { classes })
            }
            b"EnclosingMethod" => {
                let (input, class_index) = be_u16(input)?;
                let (input, method_index) = be_u16(input)?;
                (
                    input,
                    AttributeType::EnclosingMethod {
                        class_index,
                        method_index,
                    },
                )
            }
            b"Synthetic" => (input, AttributeType::Synthetic),
            b"Signature" => {
                let (input, signature_index) = be_u16(input)?;
                (input, AttributeType::Signature { signature_index })
            }
            b"SourceFile" => {
                let (input, sourcefile_index) = be_u16(input)?;
                (input, AttributeType::SourceFile { sourcefile_index })
            }
            b"SourceDebugExtension" => {
                let (input, debug_extension) =
                    length_count(success(attribute_length), be_u8)(input)?;
                (
                    input,
                    AttributeType::SourceDebugExtension { debug_extension },
                )
            }
            b"LineNumberTable" => {
                let (input, line_number_table) = length_count(be_u16, line_number)(input)?;
                (input, AttributeType::LineNumberTable { line_number_table })
            }
            b"LocalVariableTable" => {
                let (input, local_variable_table) = length_count(be_u16, local_variable)(input)?;
                (
                    input,
                    AttributeType::LocalVariableTable {
                        local_variable_table,
                    },
                )
            }
            b"LocalVariableTypeTable" => {
                let (input, local_variable_type_table) =
                    length_count(be_u16, local_variable_type)(input)?;
                (
                    input,
                    AttributeType::LocalVariableTypeTable {
                        local_variable_type_table,
                    },
                )
            }
            b"Deprecated" => (input, AttributeType::Deprecated),
            b"RuntimeVisibleAnnotations" => {
                let (input, annotations) = length_count(be_u16, annotation)(input)?;
                (
                    input,
                    AttributeType::RuntimeVisibleAnnotations { annotations },
                )
            }
            b"RuntimeInvisibleAnnotations" => {
                let (input, annotations) = length_count(be_u16, annotation)(input)?;
                (
                    input,
                    AttributeType::RuntimeInvisibleAnnotations { annotations },
                )
            }
            b"RuntimeVisibleParameterAnnotations" => {
                let (input, parameter_annotations) =
                    length_count(be_u8, parameter_annotation)(input)?;
                (
                    input,
                    AttributeType::RuntimeVisibleParameterAnnotations {
                        parameter_annotations,
                    },
                )
            }
            b"RuntimeInvisibleParameterAnnotations" => {
                let (input, parameter_annotations) =
                    length_count(be_u8, parameter_annotation)(input)?;
                (
                    input,
                    AttributeType::RuntimeInvisibleParameterAnnotations {
                        parameter_annotations,
                    },
                )
            }
            b"RuntimeVisibleTypeAnnotations" => {
                let (input, annotations) = length_count(be_u8, type_annotation)(input)?;
                (
                    input,
                    AttributeType::RuntimeVisibleTypeAnnotations { annotations },
                )
            }
            b"RuntimeInvisibleTypeAnnotations" => {
                let (input, annotations) = length_count(be_u8, type_annotation)(input)?;
                (
                    input,
                    AttributeType::RuntimeInvisibleTypeAnnotations { annotations },
                )
            }
            b"AnnotationDefault" => {
                let (input, default_value) = element_value(input)?;
                (input, AttributeType::AnnotationDefault { default_value })
            }
            b"BootstrapMethods" => {
                let (input, bootstrap_methods) = length_count(be_u16, bootstrap_method)(input)?;
                (input, AttributeType::BootstrapMethods { bootstrap_methods })
            }
            b"MethodParameters" => {
                let (input, parameters) = length_count(be_u16, method_parameter)(input)?;
                (input, AttributeType::MethodParameters { parameters })
            }
            _ => unreachable!("unexpected attribute type"),
        };
        Ok((
            input,
            Attribute {
                attribute_name_index,
                attribute_length,
                attribute_type: attr_type,
            },
        ))
    }
}

fn code_attribute<'a, E: ParseError<&'a [u8]>>(
    constant_pool: ConstantPoolRef,
) -> impl FnMut(&'a [u8]) -> IResult<&'a [u8], CodeAttribute, E>
where
    nom::Err<E>: From<nom::Err<VerboseError<&'a [u8]>>>,
{
    move |input: &[u8]| {
        let (input, max_stack) = be_u16(input)?;
        let (input, max_locals) = be_u16(input)?;
        let (input, code) = length_count(be_u32, be_u8)(input)?;
        let (input, exception_table) = length_count(be_u16, exception)(input)?;
        let (input, attributes) = length_count(be_u16, attribute(constant_pool.clone()))(input)?;
        Ok((
            input,
            CodeAttribute {
                max_stack,
                max_locals,
                code,
                exception_table,
                attributes,
            },
        ))
    }
}

fn exception(input: &[u8]) -> Res<&[u8], Exception> {
    context("exception", tuple((be_u16, be_u16, be_u16, be_u16)))(input).map(
        |(next_input, (start_pc, end_pc, handler_pc, catch_type))| {
            (
                next_input,
                Exception {
                    start_pc,
                    end_pc,
                    handler_pc,
                    catch_type,
                },
            )
        },
    )
}

fn stack_map(input: &[u8]) -> Res<&[u8], StackMap> {
    context("stack map", be_u8)(input).and_then(|(next_input, frame_type)| match frame_type {
        0..=63 => Ok((
            next_input,
            StackMap {
                frame_type,
                frame: StackMapFrame::SameFrame,
            },
        )),
        64..=127 => {
            let (next_input, stack) = verification_type_info(next_input)?;
            Ok((
                next_input,
                StackMap {
                    frame_type,
                    frame: StackMapFrame::SameLocals1StackItemFrame { stack },
                },
            ))
        }
        247 => {
            let (next_input, offset_delta) = be_u16(next_input)?;
            let (next_input, stack) = verification_type_info(next_input)?;
            Ok((
                next_input,
                StackMap {
                    frame_type,
                    frame: StackMapFrame::SameLocals1StackItemFrameExtended {
                        offset_delta,
                        stack,
                    },
                },
            ))
        }
        248..=250 => {
            let (next_input, offset_delta) = be_u16(next_input)?;
            Ok((
                next_input,
                StackMap {
                    frame_type,
                    frame: StackMapFrame::ChopFrame { offset_delta },
                },
            ))
        }
        251 => {
            let (next_input, offset_delta) = be_u16(next_input)?;
            Ok((
                next_input,
                StackMap {
                    frame_type,
                    frame: StackMapFrame::SameFrameExtended { offset_delta },
                },
            ))
        }
        252..=254 => {
            let (next_input, offset_delta) = be_u16(next_input)?;
            let (next_input, locals) =
                length_count(success(frame_type - 251), verification_type_info)(next_input)?;
            Ok((
                next_input,
                StackMap {
                    frame_type,
                    frame: StackMapFrame::AppendFrame {
                        offset_delta,
                        locals,
                    },
                },
            ))
        }
        255 => {
            let (next_input, offset_delta) = be_u16(next_input)?;
            let (next_input, locals) = length_count(be_u16, verification_type_info)(next_input)?;
            let (next_input, stack) = length_count(be_u16, verification_type_info)(next_input)?;
            Ok((
                next_input,
                StackMap {
                    frame_type,
                    frame: StackMapFrame::FullFrame {
                        offset_delta,
                        locals,
                        stack,
                    },
                },
            ))
        }
        _ => Err(NomErr::Error(VerboseError {
            errors: vec![(input, VerboseErrorKind::Context("invalid frame type"))],
        })),
    })
}

fn verification_type_info(input: &[u8]) -> Res<&[u8], VerificationTypeInfo> {
    context("verification type info", be_u8)(input).and_then(|(next_input, tag)| match tag {
        0 => Ok((next_input, VerificationTypeInfo::Top)),
        1 => Ok((next_input, VerificationTypeInfo::Integer)),
        2 => Ok((next_input, VerificationTypeInfo::Float)),
        3 => Ok((next_input, VerificationTypeInfo::Long)),
        4 => Ok((next_input, VerificationTypeInfo::Double)),
        5 => Ok((next_input, VerificationTypeInfo::Null)),
        6 => Ok((next_input, VerificationTypeInfo::UninitializedThis)),
        7 => {
            let (next_input, cpool_index) = be_u16(next_input)?;
            Ok((next_input, VerificationTypeInfo::Object { cpool_index }))
        }
        8 => {
            let (next_input, offset) = be_u16(next_input)?;
            Ok((next_input, VerificationTypeInfo::Uninitialized { offset }))
        }
        _ => Err(NomErr::Error(VerboseError {
            errors: vec![(input, VerboseErrorKind::Char(tag as char))],
        })),
    })
}

fn inner_class(input: &[u8]) -> Res<&[u8], InnerClass> {
    context("inner class", tuple((be_u16, be_u16, be_u16, be_u16)))(input).map(
        |(
            next_input,
            (
                inner_class_info_index,
                outer_class_info_index,
                inner_name_index,
                inner_class_access_flags,
            ),
        )| {
            (
                next_input,
                InnerClass {
                    inner_class_info_index,
                    outer_class_info_index,
                    inner_name_index,
                    inner_class_access_flags,
                },
            )
        },
    )
}

fn line_number(input: &[u8]) -> Res<&[u8], LineNumber> {
    context("line number", pair(be_u16, be_u16))(input).map(
        |(next_input, (start_pc, line_number))| {
            (
                next_input,
                LineNumber {
                    start_pc,
                    line_number,
                },
            )
        },
    )
}

fn local_variable(input: &[u8]) -> Res<&[u8], LocalVariable> {
    context(
        "local variable",
        tuple((be_u16, be_u16, be_u16, be_u16, be_u16)),
    )(input)
    .map(
        |(next_input, (start_pc, length, name_index, descriptor_index, index))| {
            (
                next_input,
                LocalVariable {
                    start_pc,
                    length,
                    name_index,
                    descriptor_index,
                    index,
                },
            )
        },
    )
}

fn local_variable_type(input: &[u8]) -> Res<&[u8], LocalVariableType> {
    context(
        "local variable type",
        tuple((be_u16, be_u16, be_u16, be_u16, be_u16)),
    )(input)
    .map(
        |(next_input, (start_pc, length, name_index, signature_index, index))| {
            (
                next_input,
                LocalVariableType {
                    start_pc,
                    length,
                    name_index,
                    signature_index,
                    index,
                },
            )
        },
    )
}

fn annotation(input: &[u8]) -> Res<&[u8], Annotation> {
    context(
        "annotation",
        pair(be_u16, length_count(be_u16, pair(be_u16, element_value))),
    )(input)
    .map(|(next_input, (type_index, element_value_pairs))| {
        (
            next_input,
            Annotation {
                type_index,
                element_value_pairs,
            },
        )
    })
}

fn element_value(input: &[u8]) -> Res<&[u8], ElementValue> {
    context("element value", be_u8)(input).and_then(|(next_input, tag)| match tag {
        b'B' | b'C' | b'D' | b'F' | b'I' | b'J' | b'S' | b'Z' | b's' => {
            let (next_input, value) = be_u16(next_input)?;
            Ok((
                next_input,
                ElementValue {
                    tag,
                    value: Element::ConstValueIndex(value),
                },
            ))
        }
        b'e' => {
            let (next_input, type_name_index) = be_u16(next_input)?;
            let (next_input, const_value_index) = be_u16(next_input)?;
            Ok((
                next_input,
                ElementValue {
                    tag,
                    value: Element::EnumConstValue(type_name_index, const_value_index),
                },
            ))
        }
        b'c' => {
            let (next_input, class_info_index) = be_u16(next_input)?;
            Ok((
                next_input,
                ElementValue {
                    tag,
                    value: Element::ClassInfoIndex(class_info_index),
                },
            ))
        }
        b'@' => {
            let (next_input, annotation) = annotation(next_input)?;
            Ok((
                next_input,
                ElementValue {
                    tag,
                    value: Element::AnnotationValue(annotation),
                },
            ))
        }
        b'[' => {
            let (next_input, values) = length_count(be_u16, element_value)(next_input)?;
            Ok((
                next_input,
                ElementValue {
                    tag,
                    value: Element::ArrayValue(values),
                },
            ))
        }
        c => Err(NomErr::Error(VerboseError {
            errors: vec![(input, VerboseErrorKind::Char(c as char))],
        })),
    })
}

fn parameter_annotation(input: &[u8]) -> Res<&[u8], ParameterAnnotation> {
    context("parameter annotation", length_count(be_u16, annotation))(input)
        .map(|(next_input, annotations)| (next_input, ParameterAnnotation { annotations }))
}

fn type_annotation(input: &[u8]) -> Res<&[u8], TypeAnnotation> {
    context(
        "type annotation",
        tuple((
            target_info,
            type_path,
            be_u16,
            length_count(be_u16, pair(be_u16, element_value)),
        )),
    )(input)
    .map(
        |(next_input, ((target_type, target_info), type_path, type_index, element_value_pairs))| {
            (
                next_input,
                TypeAnnotation {
                    target_type,
                    target_info,
                    type_path,
                    type_index,
                    element_value_pairs,
                },
            )
        },
    )
}

fn target_info(input: &[u8]) -> Res<&[u8], (u8, TargetInfo)> {
    context("target info", be_u8)(input).and_then(|(next_input, target_type)| match target_type {
        0x00 | 0x01 => {
            let (next_input, type_parameter_index) = be_u8(next_input)?;
            Ok((
                next_input,
                (
                    target_type,
                    TargetInfo::TypeParameterTarget(type_parameter_index),
                ),
            ))
        }
        0x10 => {
            let (next_input, supertype_index) = be_u16(next_input)?;
            Ok((
                next_input,
                (target_type, TargetInfo::SupertypeTarget(supertype_index)),
            ))
        }
        0x11 | 0x12 => {
            let (next_input, type_parameter_index) = be_u8(next_input)?;
            let (next_input, bound_index) = be_u8(next_input)?;
            Ok((
                next_input,
                (
                    target_type,
                    TargetInfo::TypeParameterBoundTarget {
                        type_parameter_index,
                        bound_index,
                    },
                ),
            ))
        }
        0x13 | 0x14 | 0x15 => Ok((next_input, (target_type, TargetInfo::EmptyTarget))),
        0x16 => {
            let (next_input, formal_parameter_index) = be_u8(next_input)?;
            Ok((
                next_input,
                (
                    target_type,
                    TargetInfo::FormalParameterTarget(formal_parameter_index),
                ),
            ))
        }
        0x17 => {
            let (next_input, throws_type_index) = be_u16(next_input)?;
            Ok((
                next_input,
                (target_type, TargetInfo::ThrowTarget(throws_type_index)),
            ))
        }
        0x40 | 0x41 => {
            let (next_input, local_vars) = length_count(be_u16, local_var)(input)?;
            Ok((
                next_input,
                (target_type, TargetInfo::LocalVarTarget(local_vars)),
            ))
        }
        0x42 => {
            let (next_input, exception_table_index) = be_u16(input)?;
            Ok((
                next_input,
                (target_type, TargetInfo::CatchTarget(exception_table_index)),
            ))
        }
        0x43 | 0x44 | 0x45 | 0x46 => {
            let (next_input, offset) = be_u16(input)?;
            Ok((next_input, (target_type, TargetInfo::OffsetTarget(offset))))
        }
        0x47 | 0x48 | 0x49 | 0x4A | 0x4B => {
            let (next_input, offset) = be_u16(input)?;
            let (next_input, type_argument_index) = be_u8(next_input)?;
            Ok((
                next_input,
                (
                    target_type,
                    TargetInfo::TypeArgumentTarget {
                        offset,
                        type_argument_index,
                    },
                ),
            ))
        }
        _ => Err(NomErr::Error(VerboseError {
            errors: vec![(input, VerboseErrorKind::Context("invalid target_type"))],
        })),
    })
}

fn type_path(input: &[u8]) -> Res<&[u8], TypePath> {
    context("type path", length_count(be_u8, pair(be_u8, be_u8)))(input)
        .map(|(next_input, path)| (next_input, TypePath { path }))
}

fn local_var(input: &[u8]) -> Res<&[u8], LocalVar> {
    context("local var", tuple((be_u16, be_u16, be_u16)))(input).map(
        |(next_input, (start_pc, length, index))| {
            (
                next_input,
                LocalVar {
                    start_pc,
                    length,
                    index,
                },
            )
        },
    )
}

fn bootstrap_method(input: &[u8]) -> Res<&[u8], BootstrapMethod> {
    context(
        "bootstrap method",
        pair(be_u16, length_count(be_u16, be_u16)),
    )(input)
    .map(
        |(next_input, (bootstrap_method_ref, bootstrap_arguments))| {
            (
                next_input,
                BootstrapMethod {
                    bootstrap_method_ref,
                    bootstrap_arguments,
                },
            )
        },
    )
}

fn method_parameter(input: &[u8]) -> Res<&[u8], MethodParameter> {
    context("method parameter", pair(be_u16, be_u16))(input).map(
        |(next_input, (name_index, access_flags))| {
            (
                next_input,
                MethodParameter {
                    name_index,
                    access_flags,
                },
            )
        },
    )
}

fn constant_tag(input: &[u8]) -> Res<&[u8], ConstantTag> {
    context("constant tag", be_u8)(input).and_then(|(next_input, tag)| match tag {
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

fn constant(input: &[u8]) -> Res<&[u8], Constant> {
    context("constant", constant_tag)(input).and_then(|(next_input, tag)| match tag {
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

fn field_info<'a, E: ParseError<&'a [u8]>>(
    constant_pool: ConstantPoolRef,
) -> impl FnMut(&'a [u8]) -> IResult<&'a [u8], FieldInfo, E>
where
    nom::Err<E>: From<nom::Err<VerboseError<&'a [u8]>>>,
{
    move |input| {
        let (input, access_flags) = be_u16(input)?;
        let (input, name_index) = be_u16(input)?;
        let (input, descriptor_index) = be_u16(input)?;
        let (input, attributes) = length_count(be_u16, attribute(constant_pool.clone()))(input)?;
        Ok((
            input,
            FieldInfo {
                access_flags,
                name_index,
                descriptor_index,
                attributes,
            },
        ))
    }
}

fn method_info<'a, E: ParseError<&'a [u8]>>(
    constant_pool: ConstantPoolRef,
) -> impl FnMut(&'a [u8]) -> IResult<&'a [u8], MethodInfo, E>
where
    nom::Err<E>: From<nom::Err<VerboseError<&'a [u8]>>>,
{
    move |input| {
        let (input, access_flags) = be_u16(input)?;
        let (input, name_index) = be_u16(input)?;
        let (input, descriptor_index) = be_u16(input)?;
        let (input, attributes) = length_count(be_u16, attribute(constant_pool.clone()))(input)?;
        let mut code_attr_index = None;
        for (i, attr) in attributes.iter().enumerate() {
            if let AttributeType::Code { .. } = attr.attribute_type {
                code_attr_index = Some(i);
            }
        }
        Ok((
            input,
            MethodInfo {
                access_flags,
                name_index,
                descriptor_index,
                attributes,
                code_attr_index,
            },
        ))
    }
}

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

#[cfg(test)]
mod test {
    use crate::class_file::ClassFile;
    use crate::{class_file, parse};
    use std::io::Read;

    #[test]
    fn read_class_file() {
        let file = std::fs::File::open("tests/HelloWorld.class").unwrap();
        let bytes: Vec<u8> = file.bytes().map(|x| x.unwrap()).collect();
        let ret = parse(bytes.as_slice());

        match ret {
            Ok((input, class_file)) => {
                println!("{:?}", class_file);
                println!("{:?}", input);
            }
            n => println!("{:?}", n),
        }
    }
}
