use crate::attribute::{attribute, Attribute, AttributeType};
use crate::{get_utf8, ConstantPoolRef};
use nom::error::{ParseError, VerboseError};
use nom::multi::length_count;
use nom::number::complete::be_u16;
use nom::IResult;

#[derive(Debug, Clone)]
pub struct MethodInfo {
    pub access_flags: u16,
    pub name_index: u16,
    pub descriptor_index: u16,
    pub attributes: Vec<Attribute>,
    pub code_attr_index: Option<usize>,
}

pub fn method_info<'a, E: ParseError<&'a [u8]>>(
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
