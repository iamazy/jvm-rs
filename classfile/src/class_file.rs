use crate::attribute::Attribute;
use crate::constant::Constant;
use crate::field::FieldInfo;
use crate::method::MethodInfo;
use crate::ConstantPoolRef;

#[derive(Debug, Clone)]
pub struct ClassFile<'a> {
    pub magic: u32,
    pub minor_version: u16,
    pub major_version: u16,
    pub constant_pool: ConstantPoolRef<'a>,
    pub access_flags: u16,
    pub this_class: u16,
    pub super_class: u16,
    pub interfaces: Vec<Constant<'a>>,
    pub fields: Vec<FieldInfo>,
    pub methods: Vec<MethodInfo>,
    pub attributes: Vec<Attribute>,
}
