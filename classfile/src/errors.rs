use crate::attribute::StackMapFrame;

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum Error {
    InvalidLength,

    InvalidString(String),

    InvalidConstantTag(u8),

    InvalidTargetType(u8),

    InvalidTargetInfo,

    // Constant
    MismatchConstantType,

    // element_value
    InvalidElementValueTag(char),

    InvalidElementValue,

    // Verification_type_info
    InvalidVerificationTypeInfo,

    // StackFrame
    InvalidFrameType,

    MismatchFrameType(u8, StackMapFrame),

    // Attribute
    InvalidAttributeName(String),
}
