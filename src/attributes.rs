use std::io::Read;

use crate::error::Error;
use crate::primitives::{print_u2, print_u4, U1, U2, U4};
use crate::print_buffer;

pub struct AttributeInfo {
    attribute_name_index: U2,
    attribute_length: U4,
    info: Vec<U1>,
}

pub fn print_attribute_info(
    reader: &mut impl Read,
    name_root: &str,
) -> Result<AttributeInfo, Error> {
    let attribute_name_index = print_u2(reader, &format!("{name_root}/attribute_name_index"))?;
    let attribute_length = print_u4(reader, &format!("{name_root}/attribute_length"))?;

    let buffer_len = attribute_length
        .try_into()
        .map_err(|e| Error::AttributeLengthTooLarge {
            attribute_name: format!("{name_root}/attribute_length"),
            attribute_length,
            source: e,
        })?;
    let info = print_buffer(reader, &format!("{name_root}/info"), buffer_len)?;

    Ok(AttributeInfo {
        attribute_name_index,
        attribute_length,
        info,
    })
}

pub fn print_attributes(
    reader: &mut impl Read,
    name_root: &str,
) -> Result<(U2, Vec<AttributeInfo>), Error> {
    let attributes_count = print_u2(reader, &format!("{name_root}/attributes_count"))?;
    let mut attributes = Vec::with_capacity(attributes_count.into());
    for attributes_index in 0..attributes_count {
        attributes.push(print_attribute_info(
            reader,
            &format!("{name_root}/attributes[{attributes_index}]"),
        )?);
    }

    Ok((attributes_count, attributes))
}
