use std::io::Read;

use crate::attributes::AttributeInfo;
use crate::error::Error;
use crate::primitives::{print_u2, U2};

pub struct FieldInfo {
    access_flags: U2,
    name_index: U2,
    descriptor_index: U2,
    attributes_count: U2,
    attributes: AttributeInfo,
}

pub fn print_field_info(reader: &mut impl Read, fields_index: usize) -> Result<FieldInfo, Error> {
    let name_root = format!("/fields[{fields_index}]");
    let access_flags = print_u2(reader, &format!("{name_root}/access_flags"));
    let name_index = print_u2(reader, &format!("{name_root}/name_index"));
    let descriptor_index = print_u2(reader, &format!("{name_root}/descriptor_index"));
    let attributes_count = print_u2(reader, &format!("{name_root}/attributes_count"));
    todo!()
}

pub fn print_fields(reader: &mut impl Read) -> Result<Vec<FieldInfo>, Error> {
    let fields_count = usize::from(print_u2(reader, "/fields_count")?);
    let mut fields = Vec::with_capacity(fields_count);
    for fields_index in 0..fields_count {
        fields.push(print_field_info(reader, fields_index)?);
    }

    Ok(fields)
}
