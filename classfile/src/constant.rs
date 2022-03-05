#[derive(Debug, Clone)]
pub enum Constant<'a> {
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
    Utf8(&'a [u8]),
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

impl From<u8> for ConstantTag {
    fn from(tag: u8) -> Self {
        match tag {
            7 => ConstantTag::Class,
            9 => ConstantTag::FieldRef,
            10 => ConstantTag::MethodRef,
            11 => ConstantTag::InterfaceMethodRef,
            8 => ConstantTag::String,
            3 => ConstantTag::Integer,
            4 => ConstantTag::Float,
            5 => ConstantTag::Long,
            6 => ConstantTag::Double,
            12 => ConstantTag::NameAndType,
            1 => ConstantTag::Utf8,
            15 => ConstantTag::MethodHandle,
            16 => ConstantTag::MethodType,
            17 => ConstantTag::Dynamic,
            18 => ConstantTag::InvokeDynamic,
            19 => ConstantTag::Module,
            20 => ConstantTag::Package,
            _ => panic!("unknown constant tag: {}", tag),
        }
    }
}
