use std::io::Read;

use crate::attributes::{print_attributes, AttributeInfo};
use crate::error::Error;
use crate::primitives::{print_u2, U2};

pub struct MethodInfo {
    access_flags: U2,
    name_index: U2,
    descriptor_index: U2,
    attributes_count: U2,
    attributes: Vec<AttributeInfo>,
}

pub fn print_method_info(
    reader: &mut impl Read,
    methods_index: usize,
) -> Result<MethodInfo, Error> {
    let name_root = format!("/methods[{methods_index}]");
    let access_flags = print_u2(reader, &format!("{name_root}/access_flags"))?;
    let name_index = print_u2(reader, &format!("{name_root}/name_index"))?;
    let descriptor_index = print_u2(reader, &format!("{name_root}/descriptor_index"))?;
    let (attributes_count, attributes) = print_attributes(reader, &name_root)?;

    Ok(MethodInfo {
        access_flags,
        name_index,
        descriptor_index,
        attributes_count,
        attributes,
    })
}

pub fn print_methods(reader: &mut impl Read) -> Result<Vec<MethodInfo>, Error> {
    let methods_count = usize::from(print_u2(reader, "/methods_count")?);
    let mut methods = Vec::with_capacity(methods_count);
    for methods_index in 0..methods_count {
        methods.push(print_method_info(reader, methods_index)?);
    }

    Ok(methods)
}
