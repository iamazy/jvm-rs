use crate::attribute::{attribute, Attribute};
use crate::constant::{constant, Constant};
use crate::field::{field_info, FieldInfo};
use crate::method::{method_info, MethodInfo};
use crate::{ConstantPoolRef, Res, MAGIC};
use nom::combinator::{map, verify};
use nom::error::context;
use nom::multi::length_count;
use nom::number::complete::{be_u16, be_u32};
use nom::sequence::tuple;
use std::rc::Rc;

#[derive(Debug, Clone)]
pub struct ClassFile {
    pub magic: u32,
    pub minor_version: u16,
    pub major_version: u16,
    pub constant_pool: ConstantPoolRef,
    pub access_flags: u16,
    pub this_class: u16,
    pub super_class: u16,
    pub interfaces: Vec<Constant>,
    pub fields: Vec<FieldInfo>,
    pub methods: Vec<MethodInfo>,
    pub attributes: Vec<Attribute>,
}

pub fn class_file(input: &[u8]) -> Res<&[u8], ClassFile> {
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

#[cfg(test)]
mod test {
    use crate::class_file::{class_file, ClassFile};
    use std::io::Read;

    #[test]
    fn read_class_file() {
        let file = std::fs::File::open("tests/HelloWorld.class").unwrap();
        let bytes: Vec<u8> = file.bytes().map(|x| x.unwrap()).collect();
        let ret = class_file(bytes.as_slice());

        match ret {
            Ok((input, class_file)) => {
                println!("{:?}", class_file);
            }
            n => println!("{:?}", n),
        }
    }
}
