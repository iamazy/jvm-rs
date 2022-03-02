use crate::errors::Error;
use crate::{BytesRef, NomErr, Res};
use nom::error::{context, ErrorKind, VerboseError, VerboseErrorKind};
use nom::multi::length_count;
use nom::number::complete::{be_u16, be_u8};
use nom::sequence::{pair, tuple};

#[derive(Debug, Clone)]
pub struct Attribute {
    pub attribute_name_index: u16,
    pub attribute_length: u32,
    pub attr_type: AttributeType,
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

#[derive(Debug, Clone)]
pub struct CodeAttribute {
    pub max_stack: u16,
    pub max_locals: u16,
    pub code: BytesRef,
    pub exception_table: Vec<Exception>,
    pub attributes: Vec<Attribute>,
}

#[derive(Debug, Clone)]
pub struct Exception {
    pub start_pc: u16,
    pub end_pc: u16,
    pub handler_pc: u16,
    pub catch_type: u16,
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

#[derive(Debug, Clone)]
pub struct StackMap {
    frame_type: u8,
    frame: StackMapFrame,
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

#[derive(Debug, Clone)]
pub struct InnerClass {
    pub inner_class_info_index: u16,
    pub outer_class_info_index: u16,
    pub inner_name_index: u16,
    pub inner_class_access_flags: u16,
}

#[derive(Debug, Clone)]
pub struct LineNumber {
    pub start_pc: u16,
    pub line_number: u16,
}

#[derive(Debug, Clone)]
pub struct LocalVariable {
    pub start_pc: u16,
    pub length: u16,
    pub name_index: u16,
    pub descriptor_index: u16,
    pub index: u16,
}

#[derive(Debug, Clone)]
pub struct LocalVariableType {
    pub start_pc: u16,
    pub length: u16,
    pub name_index: u16,
    pub signature_index: u16,
    pub index: u16,
}

#[derive(Debug, Clone)]
pub struct Annotation {
    pub type_index: u16,
    /// 0. element_name_index
    /// 1. value
    pub element_value_pairs: Vec<(u16, ElementValue)>,
}

#[derive(Debug, Clone)]
pub struct ElementValue {
    pub tag: u8,
    pub value: Element,
}

#[derive(Debug, Clone)]
pub enum Element {
    ConstValueIndex(u16),
    /// 0. type_name_index
    /// 1. const_name_index
    EnumConstValue((u16, u16)),
    ClassInfoIndex(u16),
    AnnotationValue(Annotation),
    ArrayValue(Vec<ElementValue>),
}

#[derive(Debug, Clone)]
pub struct ParameterAnnotation {
    annotations: Vec<Annotation>,
}

#[derive(Debug, Clone)]
pub struct TypeAnnotation {
    pub target_type: u8,
    pub target_info: TargetInfo,
    pub type_path: TypePath,
    pub type_index: u16,
    pub element_value_pairs: Vec<(u16, ElementValue)>,
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

pub fn target_info(input: &[u8]) -> Res<&[u8], TargetInfo> {
    context("target_info", be_u8)(input).and_then(|(next_input, target_type)| match target_type {
        0x00 | 0x01 => {
            let (next_input, type_parameter_index) = be_u8(next_input)?;
            Ok((
                next_input,
                TargetInfo::TypeParameterTarget(type_parameter_index),
            ))
        }
        0x10 => {
            let (next_input, supertype_index) = be_u16(next_input)?;
            Ok((next_input, TargetInfo::SupertypeTarget(supertype_index)))
        }
        0x11 | 0x12 => {
            let (next_input, type_parameter_index) = be_u8(next_input)?;
            let (next_input, bound_index) = be_u8(next_input)?;
            Ok((
                next_input,
                TargetInfo::TypeParameterBoundTarget {
                    type_parameter_index,
                    bound_index,
                },
            ))
        }
        0x13 | 0x14 | 0x15 => Ok((next_input, TargetInfo::EmptyTarget)),
        0x16 => {
            let (next_input, formal_parameter_index) = be_u8(next_input)?;
            Ok((
                next_input,
                TargetInfo::FormalParameterTarget(formal_parameter_index),
            ))
        }
        0x17 => {
            let (next_input, throws_type_index) = be_u16(next_input)?;
            Ok((next_input, TargetInfo::ThrowTarget(throws_type_index)))
        }
        0x40 | 0x41 => {
            let (next_input, local_vars) = length_count(be_u16, local_var)(input)?;
            Ok((next_input, TargetInfo::LocalVarTarget(local_vars)))
        }
        0x42 => {
            let (next_input, exception_table_index) = be_u16(input)?;
            Ok((next_input, TargetInfo::CatchTarget(exception_table_index)))
        }
        0x43 | 0x44 | 0x45 | 0x46 => {
            let (next_input, offset) = be_u16(input)?;
            Ok((next_input, TargetInfo::OffsetTarget(offset)))
        }
        0x47 | 0x48 | 0x49 | 0x4A | 0x4B => {
            let (next_input, offset) = be_u16(input)?;
            let (next_input, type_argument_index) = be_u8(next_input)?;
            Ok((
                next_input,
                TargetInfo::TypeArgumentTarget {
                    offset,
                    type_argument_index,
                },
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
    context("type_path", length_count(be_u8, pair(be_u8, be_u8)))(input)
        .map(|(next_input, path)| (next_input, TypePath { path }))
}

#[derive(Debug, Clone)]
pub struct LocalVar {
    pub start_pc: u16,
    pub length: u16,
    pub index: u16,
}

pub fn local_var(input: &[u8]) -> Res<&[u8], LocalVar> {
    context("local_var", tuple((be_u16, be_u16, be_u16)))(input).map(
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
        "bootstrap_method",
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
    context("method_parameter", pair(be_u16, be_u16))(input).map(
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
