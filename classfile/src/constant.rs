use crate::BytesRef;

#[derive(Debug, Clone)]
pub enum Constant {
    Class {
        name_index: u16,
    },
    FieldRef {
        // The class_index item of a CONSTANT_Fieldref_info structure may be either a class type or an interface type.
        class_index: u16,
        name_and_type_index: u16,
    },
    MethodRef {
        // The class_index item of a CONSTANT_Methodref_info structure must be a class type, not an interface type.
        class_index: u16,
        // If the name of the method of a CONSTANT_Methodref_info structure begins with a '<' ('\u003c'),
        // then the name must be the special name <init>, representing an instance initialization method.
        // The return type of such a method must be void.
        name_and_type_index: u16,
    },
    InterfaceMethodRef {
        // The class_index item of a CONSTANT_InterfaceMethodref_info structure must be an interface type.
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
    InvokeDynamic {
        bootstrap_method_attr_index: u16,
        name_and_type_index: u16,
    },
}

#[derive(Debug, Clone)]
pub enum Tag {
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