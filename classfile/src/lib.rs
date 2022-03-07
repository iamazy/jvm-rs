pub use attribute::{
    Annotation, Attribute, AttributeType, BootstrapMethod, CodeAttribute, Element, ElementValue,
    Exception, Export, InnerClass, LineNumber, LocalVar, LocalVariable, LocalVariableType,
    MethodParameter, Open, ParameterAnnotation, Provide, RecordComponent, Require, StackMap,
    StackMapFrame, TargetInfo, TypeAnnotation, TypePath, VerificationTypeInfo,
};
pub use class_file::ClassFile;
pub use constant::{Constant, ConstantTag};
pub use field::FieldInfo;
pub use method::MethodInfo;

use crate::attribute::AttributeTag;
use bitflags::bitflags;
use nom::bytes::complete::{tag, take};
use nom::combinator::{all_consuming, map, success};
use nom::error::{context, ErrorKind, ParseError, VerboseError, VerboseErrorKind};
use nom::multi::length_count;
use nom::number::complete::{be_f32, be_f64, be_i32, be_i64, be_u16, be_u32, be_u8};
use nom::sequence::{pair, tuple};
use nom::Err as NomErr;
use std::rc::Rc;

mod attribute;
mod class_file;
mod constant;
mod errors;
mod field;
mod method;

const MAGIC: &[u8] = b"\xCA\xFE\xBA\xBE";

type ConstantPoolRef<'a> = Rc<Vec<Constant<'a>>>;

type IResult<I, O, E = (I, ErrorKind)> = Result<(I, O), NomErr<E>>;
type Res<T, U> = IResult<T, U, VerboseError<T>>;

pub fn get_utf8(constant_pool: ConstantPoolRef, index: usize) -> &[u8] {
    match constant_pool.get(index) {
        Some(Constant::Utf8(bytes)) => *bytes,
        Some(Constant::Class { name_index }) => {
            get_utf8(constant_pool.clone(), *name_index as usize)
        }
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
            tag(MAGIC),
            be_u16,
            be_u16,
            constant_pool,
            be_u16,
            be_u16,
            be_u16,
            length_count(be_u16, be_u16),
        )),
    )(input)
    .and_then(
        |(
            input,
            (
                _,
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

fn constant_pool(input: &[u8]) -> Res<&[u8], Vec<Constant>> {
    context("constant pool", be_u16)(input).and_then(|(mut next_input, count)| {
        let mut constant_pool = Vec::with_capacity(count as usize);
        constant_pool.push(Constant::Placeholder);
        let mut i = 1;
        while i < count {
            let (input, constant) = constant(next_input)?;
            constant_pool.push(constant.clone());
            next_input = input;
            i += 1;
            match constant {
                Constant::Long(_) | Constant::Double(_) => {
                    constant_pool.push(Constant::Placeholder);
                    i += 1;
                }
                _ => {}
            }
        }
        Ok((next_input, constant_pool))
    })
}

fn attribute<'a, E: ParseError<&'a [u8]>>(
    constant_pool: ConstantPoolRef,
) -> impl FnMut(&'a [u8]) -> IResult<&'a [u8], Attribute, E> + '_
where
    NomErr<E>: From<NomErr<VerboseError<&'a [u8]>>>,
{
    move |input: &'a [u8]| {
        let (input, attribute_name_index) = be_u16(input)?;
        let (input, attribute_length) = be_u32(input)?;
        let attribute_name = get_utf8(constant_pool.clone(), attribute_name_index as usize);
        let (input, attr_type) = match attribute_name.into() {
            AttributeTag::ConstantValue => {
                let (input, constant_value_index) = be_u16(input)?;
                (
                    input,
                    AttributeType::ConstantValue {
                        constant_value_index,
                    },
                )
            }
            AttributeTag::Code => {
                let (input, code) = code_attribute(constant_pool.clone())(input)?;
                (input, AttributeType::Code { code })
            }
            AttributeTag::StackMapTable => {
                let (input, entries) = length_count(be_u16, stack_map)(input)?;
                (input, AttributeType::StackMapTable { entries })
            }
            AttributeTag::Exceptions => {
                let (input, exception_index_table) = length_count(be_u16, be_u16)(input)?;
                (
                    input,
                    AttributeType::Exceptions {
                        exception_index_table,
                    },
                )
            }
            AttributeTag::InnerClasses => {
                let (input, classes) = length_count(be_u16, inner_class)(input)?;
                (input, AttributeType::InnerClasses { classes })
            }
            AttributeTag::EnclosingMethod => {
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
            AttributeTag::Synthetic => (input, AttributeType::Synthetic),
            AttributeTag::Signature => {
                let (input, signature_index) = be_u16(input)?;
                (input, AttributeType::Signature { signature_index })
            }
            AttributeTag::SourceFile => {
                let (input, sourcefile_index) = be_u16(input)?;
                (input, AttributeType::SourceFile { sourcefile_index })
            }
            AttributeTag::SourceDebugExtension => {
                let (input, debug_extension) =
                    length_count(success(attribute_length), be_u8)(input)?;
                (
                    input,
                    AttributeType::SourceDebugExtension { debug_extension },
                )
            }
            AttributeTag::LineNumberTable => {
                let (input, line_number_table) = length_count(be_u16, line_number)(input)?;
                (input, AttributeType::LineNumberTable { line_number_table })
            }
            AttributeTag::LocalVariableTable => {
                let (input, local_variable_table) = length_count(be_u16, local_variable)(input)?;
                (
                    input,
                    AttributeType::LocalVariableTable {
                        local_variable_table,
                    },
                )
            }
            AttributeTag::LocalVariableTypeTable => {
                let (input, local_variable_type_table) =
                    length_count(be_u16, local_variable_type)(input)?;
                (
                    input,
                    AttributeType::LocalVariableTypeTable {
                        local_variable_type_table,
                    },
                )
            }
            AttributeTag::Deprecated => (input, AttributeType::Deprecated),
            AttributeTag::RuntimeVisibleAnnotations => {
                let (input, annotations) = length_count(be_u16, annotation)(input)?;
                (
                    input,
                    AttributeType::RuntimeVisibleAnnotations { annotations },
                )
            }
            AttributeTag::RuntimeInvisibleAnnotations => {
                let (input, annotations) = length_count(be_u16, annotation)(input)?;
                (
                    input,
                    AttributeType::RuntimeInvisibleAnnotations { annotations },
                )
            }
            AttributeTag::RuntimeVisibleParameterAnnotations => {
                let (input, parameter_annotations) =
                    length_count(be_u8, parameter_annotation)(input)?;
                (
                    input,
                    AttributeType::RuntimeVisibleParameterAnnotations {
                        parameter_annotations,
                    },
                )
            }
            AttributeTag::RuntimeInvisibleParameterAnnotations => {
                let (input, parameter_annotations) =
                    length_count(be_u8, parameter_annotation)(input)?;
                (
                    input,
                    AttributeType::RuntimeInvisibleParameterAnnotations {
                        parameter_annotations,
                    },
                )
            }
            AttributeTag::RuntimeVisibleTypeAnnotations => {
                let (input, annotations) = length_count(be_u8, type_annotation)(input)?;
                (
                    input,
                    AttributeType::RuntimeVisibleTypeAnnotations { annotations },
                )
            }
            AttributeTag::RuntimeInvisibleTypeAnnotations => {
                let (input, annotations) = length_count(be_u8, type_annotation)(input)?;
                (
                    input,
                    AttributeType::RuntimeInvisibleTypeAnnotations { annotations },
                )
            }
            AttributeTag::AnnotationDefault => {
                let (input, default_value) = element_value(input)?;
                (input, AttributeType::AnnotationDefault { default_value })
            }
            AttributeTag::BootstrapMethods => {
                let (input, bootstrap_methods) = length_count(be_u16, bootstrap_method)(input)?;
                (input, AttributeType::BootstrapMethods { bootstrap_methods })
            }
            AttributeTag::MethodParameters => {
                let (input, parameters) = length_count(be_u16, method_parameter)(input)?;
                (input, AttributeType::MethodParameters { parameters })
            }
            AttributeTag::Module => {
                let (input, module_name_index) = be_u16(input)?;
                let (input, module_flags) = be_u16(input)?;
                let (input, module_version_index) = be_u16(input)?;
                let (input, requires) = length_count(be_u16, require)(input)?;
                let (input, exports) = length_count(be_u16, export)(input)?;
                let (input, opens) = length_count(be_u16, open)(input)?;
                let (input, uses) = length_count(be_u16, be_u16)(input)?;
                let (input, provides) = length_count(be_u16, provide)(input)?;
                (
                    input,
                    AttributeType::Module {
                        module_name_index,
                        module_flags,
                        module_version_index,
                        requires,
                        exports,
                        opens,
                        uses,
                        provides,
                    },
                )
            }
            AttributeTag::ModulePackages => {
                let (input, package_index) = length_count(be_u16, be_u16)(input)?;
                (input, AttributeType::ModulePackages { package_index })
            }
            AttributeTag::ModuleMainClass => {
                let (input, main_class_index) = be_u16(input)?;
                (input, AttributeType::ModuleMainClass { main_class_index })
            }
            AttributeTag::NestHost => {
                let (input, host_class_index) = be_u16(input)?;
                (input, AttributeType::NestHost { host_class_index })
            }
            AttributeTag::NestMembers => {
                let (input, classes) = length_count(be_u16, be_u16)(input)?;
                (input, AttributeType::NestMembers { classes })
            }
            AttributeTag::Record => {
                let (input, components) =
                    length_count(be_u16, record_component(constant_pool.clone()))(input)?;
                (input, AttributeType::Record { components })
            }
            AttributeTag::PermittedSubclasses => {
                let (input, classes) = length_count(be_u16, be_u16)(input)?;
                (input, AttributeType::PermittedSubclasses { classes })
            }
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
) -> impl FnMut(&'a [u8]) -> IResult<&'a [u8], CodeAttribute, E> + '_
where
    NomErr<E>: From<NomErr<VerboseError<&'a [u8]>>>,
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
        |(input, (start_pc, end_pc, handler_pc, catch_type))| {
            (
                input,
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
    context("stack map", be_u8)(input).and_then(|(input, frame_type)| match frame_type {
        0..=63 => Ok((
            input,
            StackMap {
                frame_type,
                frame: StackMapFrame::SameFrame,
            },
        )),
        64..=127 => {
            let (input, stack) = verification_type_info(input)?;
            Ok((
                input,
                StackMap {
                    frame_type,
                    frame: StackMapFrame::SameLocals1StackItemFrame { stack },
                },
            ))
        }
        247 => {
            let (input, offset_delta) = be_u16(input)?;
            let (input, stack) = verification_type_info(input)?;
            Ok((
                input,
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
            let (input, offset_delta) = be_u16(input)?;
            Ok((
                input,
                StackMap {
                    frame_type,
                    frame: StackMapFrame::ChopFrame { offset_delta },
                },
            ))
        }
        251 => {
            let (input, offset_delta) = be_u16(input)?;
            Ok((
                input,
                StackMap {
                    frame_type,
                    frame: StackMapFrame::SameFrameExtended { offset_delta },
                },
            ))
        }
        252..=254 => {
            let (input, offset_delta) = be_u16(input)?;
            let (input, locals) =
                length_count(success(frame_type - 251), verification_type_info)(input)?;
            Ok((
                input,
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
            let (input, offset_delta) = be_u16(input)?;
            let (input, locals) = length_count(be_u16, verification_type_info)(input)?;
            let (input, stack) = length_count(be_u16, verification_type_info)(input)?;
            Ok((
                input,
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
    context("verification type info", be_u8)(input).and_then(|(input, tag)| match tag {
        0 => Ok((input, VerificationTypeInfo::Top)),
        1 => Ok((input, VerificationTypeInfo::Integer)),
        2 => Ok((input, VerificationTypeInfo::Float)),
        3 => Ok((input, VerificationTypeInfo::Long)),
        4 => Ok((input, VerificationTypeInfo::Double)),
        5 => Ok((input, VerificationTypeInfo::Null)),
        6 => Ok((input, VerificationTypeInfo::UninitializedThis)),
        7 => {
            let (input, cpool_index) = be_u16(input)?;
            Ok((input, VerificationTypeInfo::Object { cpool_index }))
        }
        8 => {
            let (input, offset) = be_u16(input)?;
            Ok((input, VerificationTypeInfo::Uninitialized { offset }))
        }
        _ => Err(NomErr::Error(VerboseError {
            errors: vec![(input, VerboseErrorKind::Char(tag as char))],
        })),
    })
}

fn inner_class(input: &[u8]) -> Res<&[u8], InnerClass> {
    context("inner class", tuple((be_u16, be_u16, be_u16, be_u16)))(input).map(
        |(
            input,
            (
                inner_class_info_index,
                outer_class_info_index,
                inner_name_index,
                inner_class_access_flags,
            ),
        )| {
            (
                input,
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
    context("line number", pair(be_u16, be_u16))(input).map(|(input, (start_pc, line_number))| {
        (
            input,
            LineNumber {
                start_pc,
                line_number,
            },
        )
    })
}

fn local_variable(input: &[u8]) -> Res<&[u8], LocalVariable> {
    context(
        "local variable",
        tuple((be_u16, be_u16, be_u16, be_u16, be_u16)),
    )(input)
    .map(
        |(input, (start_pc, length, name_index, descriptor_index, index))| {
            (
                input,
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
        |(input, (start_pc, length, name_index, signature_index, index))| {
            (
                input,
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
    .map(|(input, (type_index, element_value_pairs))| {
        (
            input,
            Annotation {
                type_index,
                element_value_pairs,
            },
        )
    })
}

fn element_value(input: &[u8]) -> Res<&[u8], ElementValue> {
    context("element value", be_u8)(input).and_then(|(input, tag)| match tag {
        b'B' | b'C' | b'D' | b'F' | b'I' | b'J' | b'S' | b'Z' | b's' => {
            let (input, value) = be_u16(input)?;
            Ok((
                input,
                ElementValue {
                    tag,
                    value: Element::ConstValueIndex(value),
                },
            ))
        }
        b'e' => {
            let (input, type_name_index) = be_u16(input)?;
            let (input, const_value_index) = be_u16(input)?;
            Ok((
                input,
                ElementValue {
                    tag,
                    value: Element::EnumConstValue(type_name_index, const_value_index),
                },
            ))
        }
        b'c' => {
            let (input, class_info_index) = be_u16(input)?;
            Ok((
                input,
                ElementValue {
                    tag,
                    value: Element::ClassInfoIndex(class_info_index),
                },
            ))
        }
        b'@' => {
            let (input, annotation) = annotation(input)?;
            Ok((
                input,
                ElementValue {
                    tag,
                    value: Element::AnnotationValue(annotation),
                },
            ))
        }
        b'[' => {
            let (input, values) = length_count(be_u16, element_value)(input)?;
            Ok((
                input,
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
        .map(|(input, annotations)| (input, ParameterAnnotation { annotations }))
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
        |(input, ((target_type, target_info), type_path, type_index, element_value_pairs))| {
            (
                input,
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
    context("target info", be_u8)(input).and_then(|(input, target_type)| match target_type {
        0x00 | 0x01 => {
            let (input, type_parameter_index) = be_u8(input)?;
            Ok((
                input,
                (
                    target_type,
                    TargetInfo::TypeParameterTarget(type_parameter_index),
                ),
            ))
        }
        0x10 => {
            let (input, supertype_index) = be_u16(input)?;
            Ok((
                input,
                (target_type, TargetInfo::SupertypeTarget(supertype_index)),
            ))
        }
        0x11 | 0x12 => {
            let (input, type_parameter_index) = be_u8(input)?;
            let (input, bound_index) = be_u8(input)?;
            Ok((
                input,
                (
                    target_type,
                    TargetInfo::TypeParameterBoundTarget {
                        type_parameter_index,
                        bound_index,
                    },
                ),
            ))
        }
        0x13 | 0x14 | 0x15 => Ok((input, (target_type, TargetInfo::EmptyTarget))),
        0x16 => {
            let (input, formal_parameter_index) = be_u8(input)?;
            Ok((
                input,
                (
                    target_type,
                    TargetInfo::FormalParameterTarget(formal_parameter_index),
                ),
            ))
        }
        0x17 => {
            let (input, throws_type_index) = be_u16(input)?;
            Ok((
                input,
                (target_type, TargetInfo::ThrowTarget(throws_type_index)),
            ))
        }
        0x40 | 0x41 => {
            let (input, local_vars) = length_count(be_u16, local_var)(input)?;
            Ok((input, (target_type, TargetInfo::LocalVarTarget(local_vars))))
        }
        0x42 => {
            let (input, exception_table_index) = be_u16(input)?;
            Ok((
                input,
                (target_type, TargetInfo::CatchTarget(exception_table_index)),
            ))
        }
        0x43 | 0x44 | 0x45 | 0x46 => {
            let (input, offset) = be_u16(input)?;
            Ok((input, (target_type, TargetInfo::OffsetTarget(offset))))
        }
        0x47 | 0x48 | 0x49 | 0x4A | 0x4B => {
            let (input, offset) = be_u16(input)?;
            let (input, type_argument_index) = be_u8(input)?;
            Ok((
                input,
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
        .map(|(input, path)| (input, TypePath { path }))
}

fn local_var(input: &[u8]) -> Res<&[u8], LocalVar> {
    context("local var", tuple((be_u16, be_u16, be_u16)))(input).map(
        |(input, (start_pc, length, index))| {
            (
                input,
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
    .map(|(input, (bootstrap_method_ref, bootstrap_arguments))| {
        (
            input,
            BootstrapMethod {
                bootstrap_method_ref,
                bootstrap_arguments,
            },
        )
    })
}

fn method_parameter(input: &[u8]) -> Res<&[u8], MethodParameter> {
    context("method parameter", pair(be_u16, be_u16))(input).map(
        |(input, (name_index, access_flags))| {
            (
                input,
                MethodParameter {
                    name_index,
                    access_flags,
                },
            )
        },
    )
}

fn constant_tag(input: &[u8]) -> Res<&[u8], ConstantTag> {
    context("constant tag", map(be_u8, Into::into))(input)
}

fn constant(input: &[u8]) -> Res<&[u8], Constant> {
    context("constant", constant_tag)(input).and_then(|(input, tag)| match tag {
        ConstantTag::Class => {
            let (input, name_index) = be_u16(input)?;
            Ok((input, Constant::Class { name_index }))
        }
        ConstantTag::FieldRef => {
            let (input, class_index) = be_u16(input)?;
            let (input, name_and_type_index) = be_u16(input)?;
            Ok((
                input,
                Constant::FieldRef {
                    class_index,
                    name_and_type_index,
                },
            ))
        }
        ConstantTag::MethodRef => {
            let (input, class_index) = be_u16(input)?;
            let (input, name_and_type_index) = be_u16(input)?;
            Ok((
                input,
                Constant::MethodRef {
                    class_index,
                    name_and_type_index,
                },
            ))
        }
        ConstantTag::InterfaceMethodRef => {
            let (input, class_index) = be_u16(input)?;
            let (input, name_and_type_index) = be_u16(input)?;
            Ok((
                input,
                Constant::InterfaceMethodRef {
                    class_index,
                    name_and_type_index,
                },
            ))
        }
        ConstantTag::String => {
            let (input, string_index) = be_u16(input)?;
            Ok((input, Constant::String { string_index }))
        }
        ConstantTag::Integer => {
            let (input, value) = be_i32(input)?;
            Ok((input, Constant::Integer(value)))
        }
        ConstantTag::Float => {
            let (input, value) = be_f32(input)?;
            Ok((input, Constant::Float(value)))
        }
        ConstantTag::Long => {
            let (input, value) = be_i64(input)?;
            Ok((input, Constant::Long(value)))
        }
        ConstantTag::Double => {
            let (input, value) = be_f64(input)?;
            Ok((input, Constant::Double(value)))
        }
        ConstantTag::NameAndType => {
            let (input, name_index) = be_u16(input)?;
            let (input, descriptor_index) = be_u16(input)?;
            Ok((
                input,
                Constant::NameAndType {
                    name_index,
                    descriptor_index,
                },
            ))
        }
        ConstantTag::Utf8 => {
            let (input, length) = be_u16(input)?;
            let (input, bytes) = take(length)(input)?;
            Ok((input, Constant::Utf8(bytes)))
        }
        ConstantTag::MethodHandle => {
            let (input, reference_kind) = be_u8(input)?;
            let (input, reference_index) = be_u16(input)?;
            Ok((
                input,
                Constant::MethodHandle {
                    reference_kind,
                    reference_index,
                },
            ))
        }
        ConstantTag::MethodType => {
            let (input, descriptor_index) = be_u16(input)?;
            Ok((input, Constant::MethodType { descriptor_index }))
        }
        ConstantTag::Dynamic => {
            let (input, bootstrap_method_attr_index) = be_u16(input)?;
            let (input, name_and_type_index) = be_u16(input)?;
            Ok((
                input,
                Constant::Dynamic {
                    bootstrap_method_attr_index,
                    name_and_type_index,
                },
            ))
        }
        ConstantTag::InvokeDynamic => {
            let (input, bootstrap_method_attr_index) = be_u16(input)?;
            let (input, name_and_type_index) = be_u16(input)?;
            Ok((
                input,
                Constant::InvokeDynamic {
                    bootstrap_method_attr_index,
                    name_and_type_index,
                },
            ))
        }
        ConstantTag::Module => {
            let (input, name_index) = be_u16(input)?;
            Ok((input, Constant::Module { name_index }))
        }
        ConstantTag::Package => {
            let (input, name_index) = be_u16(input)?;
            Ok((input, Constant::Package { name_index }))
        }
    })
}

fn field_info<'a, E: ParseError<&'a [u8]>>(
    constant_pool: ConstantPoolRef,
) -> impl FnMut(&'a [u8]) -> IResult<&'a [u8], FieldInfo, E> + '_
where
    NomErr<E>: From<NomErr<VerboseError<&'a [u8]>>>,
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
) -> impl FnMut(&'a [u8]) -> IResult<&'a [u8], MethodInfo, E> + '_
where
    NomErr<E>: From<NomErr<VerboseError<&'a [u8]>>>,
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

fn require(input: &[u8]) -> Res<&[u8], Require> {
    context("require", tuple((be_u16, be_u16, be_u16)))(input).map(
        |(input, (require_index, require_flags, require_version_index))| {
            (
                input,
                Require {
                    require_index,
                    require_flags,
                    require_version_index,
                },
            )
        },
    )
}

fn export(input: &[u8]) -> Res<&[u8], Export> {
    context(
        "export",
        tuple((be_u16, be_u16, length_count(be_u16, be_u16))),
    )(input)
    .map(|(input, (export_index, export_flags, export_to_index))| {
        (
            input,
            Export {
                export_index,
                export_flags,
                export_to_index,
            },
        )
    })
}

fn open(input: &[u8]) -> Res<&[u8], Open> {
    context(
        "open",
        tuple((be_u16, be_u16, length_count(be_u16, be_u16))),
    )(input)
    .map(|(input, (open_index, open_flags, open_to_index))| {
        (
            input,
            Open {
                open_index,
                open_flags,
                open_to_index,
            },
        )
    })
}

fn provide(input: &[u8]) -> Res<&[u8], Provide> {
    context("provide", pair(be_u16, length_count(be_u16, be_u16)))(input).map(
        |(input, (provide_index, provide_with_index))| {
            (
                input,
                Provide {
                    provide_index,
                    provide_with_index,
                },
            )
        },
    )
}

fn record_component<'a, E: ParseError<&'a [u8]>>(
    constant_pool: ConstantPoolRef,
) -> impl FnMut(&'a [u8]) -> IResult<&'a [u8], RecordComponent, E> + '_
where
    NomErr<E>: From<NomErr<VerboseError<&'a [u8]>>>,
{
    move |input| {
        let (input, name_index) = be_u16(input)?;
        let (input, descriptor_index) = be_u16(input)?;
        let (input, attributes) = length_count(be_u16, attribute(constant_pool.clone()))(input)?;
        Ok((
            input,
            RecordComponent {
                name_index,
                descriptor_index,
                attributes,
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
    use crate::parse;
    use std::io::Read;

    #[test]
    fn read_class_file() {
        let file = std::fs::File::open("../data/jvm8/HelloWorld.class").unwrap();
        let bytes: Vec<u8> = file.bytes().map(|x| x.unwrap()).collect();
        let ret = parse(bytes.as_slice());

        match ret {
            Ok((input, class_file)) => {
                println!("{:?}", class_file);
                assert_eq!(input.len(), 0);
            }
            n => println!("{:?}", n),
        }
    }
}
