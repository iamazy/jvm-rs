use crate::{get_utf8, ConstantPoolRef, NomErr, Res};
use nom::combinator::success;
use nom::error::{context, ParseError, VerboseError, VerboseErrorKind};
use nom::multi::length_count;
use nom::number::complete::{be_u16, be_u32, be_u8};
use nom::sequence::{pair, tuple};
use nom::IResult;

#[derive(Debug, Clone)]
pub struct Attribute {
    pub attribute_name_index: u16,
    pub attribute_length: u32,
    pub attribute_type: AttributeType,
}

#[derive(Debug, Clone)]
pub enum AttributeType {
    ConstantValue {
        constant_value_index: u16,
    },
    Code {
        code: CodeAttribute,
    },
    StackMapTable {
        entries: Vec<StackMap>,
    },
    Exceptions {
        exception_index_table: Vec<u16>,
    },
    InnerClasses {
        classes: Vec<InnerClass>,
    },
    EnclosingMethod {
        class_index: u16,
        method_index: u16,
    },
    Synthetic,
    Signature {
        signature_index: u16,
    },
    SourceFile {
        sourcefile_index: u16,
    },
    SourceDebugExtension {
        debug_extension: Vec<u8>,
    },
    LineNumberTable {
        line_number_table: Vec<LineNumber>,
    },
    LocalVariableTable {
        local_variable_table: Vec<LocalVariable>,
    },
    LocalVariableTypeTable {
        local_variable_type_table: Vec<LocalVariableType>,
    },
    Deprecated,
    RuntimeVisibleAnnotations {
        annotations: Vec<Annotation>,
    },
    RuntimeInvisibleAnnotations {
        annotations: Vec<Annotation>,
    },
    RuntimeVisibleParameterAnnotations {
        parameter_annotations: Vec<ParameterAnnotation>,
    },
    RuntimeInvisibleParameterAnnotations {
        parameter_annotations: Vec<ParameterAnnotation>,
    },
    RuntimeVisibleTypeAnnotations {
        annotations: Vec<TypeAnnotation>,
    },
    RuntimeInvisibleTypeAnnotations {
        annotations: Vec<TypeAnnotation>,
    },
    AnnotationDefault {
        default_value: ElementValue,
    },
    BootstrapMethods {
        bootstrap_methods: Vec<BootstrapMethod>,
    },
    MethodParameters {
        parameters: Vec<MethodParameter>,
    },
}

pub fn attribute<'a, E: ParseError<&'a [u8]>>(
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

#[derive(Debug, Clone)]
pub struct CodeAttribute {
    pub max_stack: u16,
    pub max_locals: u16,
    pub code: Vec<u8>,
    pub exception_table: Vec<Exception>,
    pub attributes: Vec<Attribute>,
}

pub fn code_attribute<'a, E: ParseError<&'a [u8]>>(
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

#[derive(Debug, Clone)]
pub struct Exception {
    pub start_pc: u16,
    pub end_pc: u16,
    pub handler_pc: u16,
    pub catch_type: u16,
}

pub fn exception(input: &[u8]) -> Res<&[u8], Exception> {
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

#[derive(Debug, Clone)]
pub struct StackMap {
    frame_type: u8,
    frame: StackMapFrame,
}

#[derive(Debug, Clone)]
pub enum StackMapFrame {
    SameFrame,
    SameLocals1StackItemFrame {
        stack: VerificationTypeInfo,
    },
    SameLocals1StackItemFrameExtended {
        offset_delta: u16,
        stack: VerificationTypeInfo,
    },
    ChopFrame {
        offset_delta: u16,
    },
    SameFrameExtended {
        offset_delta: u16,
    },
    AppendFrame {
        offset_delta: u16,
        locals: Vec<VerificationTypeInfo>,
    },
    FullFrame {
        offset_delta: u16,
        locals: Vec<VerificationTypeInfo>,
        stack: Vec<VerificationTypeInfo>,
    },
}

pub fn stack_map(input: &[u8]) -> Res<&[u8], StackMap> {
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

#[derive(Debug, Clone)]
pub enum VerificationTypeInfo {
    Top,
    Integer,
    Float,
    Long,
    Double,
    Null,
    UninitializedThis,
    Object { cpool_index: u16 },
    Uninitialized { offset: u16 },
}

pub fn verification_type_info(input: &[u8]) -> Res<&[u8], VerificationTypeInfo> {
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

#[derive(Debug, Clone)]
pub struct InnerClass {
    pub inner_class_info_index: u16,
    pub outer_class_info_index: u16,
    pub inner_name_index: u16,
    pub inner_class_access_flags: u16,
}

pub fn inner_class(input: &[u8]) -> Res<&[u8], InnerClass> {
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

#[derive(Debug, Clone)]
pub struct LineNumber {
    pub start_pc: u16,
    pub line_number: u16,
}

pub fn line_number(input: &[u8]) -> Res<&[u8], LineNumber> {
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

#[derive(Debug, Clone)]
pub struct LocalVariable {
    pub start_pc: u16,
    pub length: u16,
    pub name_index: u16,
    pub descriptor_index: u16,
    pub index: u16,
}

pub fn local_variable(input: &[u8]) -> Res<&[u8], LocalVariable> {
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

#[derive(Debug, Clone)]
pub struct LocalVariableType {
    pub start_pc: u16,
    pub length: u16,
    pub name_index: u16,
    pub signature_index: u16,
    pub index: u16,
}

pub fn local_variable_type(input: &[u8]) -> Res<&[u8], LocalVariableType> {
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

#[derive(Debug, Clone)]
pub struct Annotation {
    pub type_index: u16,
    /// 0. element_name_index
    /// 1. value
    pub element_value_pairs: Vec<(u16, ElementValue)>,
}

pub fn annotation(input: &[u8]) -> Res<&[u8], Annotation> {
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

#[derive(Debug, Clone)]
pub struct ElementValue {
    pub tag: u8,
    pub value: Element,
}

pub fn element_value(input: &[u8]) -> Res<&[u8], ElementValue> {
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

#[derive(Debug, Clone)]
pub enum Element {
    ConstValueIndex(u16),
    /// 0. type_name_index
    /// 1. const_name_index
    EnumConstValue(u16, u16),
    ClassInfoIndex(u16),
    AnnotationValue(Annotation),
    ArrayValue(Vec<ElementValue>),
}

#[derive(Debug, Clone)]
pub struct ParameterAnnotation {
    annotations: Vec<Annotation>,
}

pub fn parameter_annotation(input: &[u8]) -> Res<&[u8], ParameterAnnotation> {
    context("parameter annotation", length_count(be_u16, annotation))(input)
        .map(|(next_input, annotations)| (next_input, ParameterAnnotation { annotations }))
}

#[derive(Debug, Clone)]
pub struct TypeAnnotation {
    pub target_type: u8,
    pub target_info: TargetInfo,
    pub type_path: TypePath,
    pub type_index: u16,
    pub element_value_pairs: Vec<(u16, ElementValue)>,
}

pub fn type_annotation(input: &[u8]) -> Res<&[u8], TypeAnnotation> {
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

#[derive(Debug, Clone)]
pub enum TargetInfo {
    /// type_parameter_index
    TypeParameterTarget(u8),
    /// supertype_index
    SupertypeTarget(u16),
    TypeParameterBoundTarget {
        type_parameter_index: u8,
        bound_index: u8,
    },
    EmptyTarget,
    /// formal_parameter_index
    FormalParameterTarget(u8),
    /// throws_type_index
    ThrowTarget(u16),
    LocalVarTarget(Vec<LocalVar>),
    /// exception_table_index
    CatchTarget(u16),
    /// offset
    OffsetTarget(u16),
    TypeArgumentTarget {
        offset: u16,
        type_argument_index: u8,
    },
}

pub fn target_info(input: &[u8]) -> Res<&[u8], (u8, TargetInfo)> {
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

#[derive(Debug, Clone)]
pub struct TypePath {
    /// 0. type_path_kind
    /// 1. type_argument_index
    pub path: Vec<(u8, u8)>,
}

pub fn type_path(input: &[u8]) -> Res<&[u8], TypePath> {
    context("type path", length_count(be_u8, pair(be_u8, be_u8)))(input)
        .map(|(next_input, path)| (next_input, TypePath { path }))
}

#[derive(Debug, Clone)]
pub struct LocalVar {
    pub start_pc: u16,
    pub length: u16,
    pub index: u16,
}

pub fn local_var(input: &[u8]) -> Res<&[u8], LocalVar> {
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

#[derive(Debug, Clone)]
pub struct BootstrapMethod {
    pub bootstrap_method_ref: u16,
    pub bootstrap_arguments: Vec<u16>,
}

pub fn bootstrap_method(input: &[u8]) -> Res<&[u8], BootstrapMethod> {
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

#[derive(Debug, Clone)]
pub struct MethodParameter {
    pub name_index: u16,
    pub access_flags: u16,
}

pub fn method_parameter(input: &[u8]) -> Res<&[u8], MethodParameter> {
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
