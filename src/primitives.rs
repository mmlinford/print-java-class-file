use std::io::Read;

use byteorder::{BigEndian, ReadBytesExt};

use crate::error::Error;

pub type U1 = u8;
pub type U2 = u16;
pub type U4 = u32;

pub type ClassFileByteOrder = BigEndian;

pub fn print_u1(reader: &mut impl Read, field_name: &str) -> Result<U1, Error> {
    let result = reader.read_u8().map_err(|e| Error::ReadPrimitive {
        field_name: field_name.to_string(),
        source: e,
    })?;
    println!("{field_name}: {result}");

    Ok(result)
}

pub fn print_u2(reader: &mut impl Read, field_name: &str) -> Result<U2, Error> {
    let result = reader
        .read_u16::<ClassFileByteOrder>()
        .map_err(|e| Error::ReadPrimitive {
            field_name: field_name.to_string(),
            source: e,
        })?;
    println!("{field_name}: {result}");

    Ok(result)
}

pub fn print_u4(reader: &mut impl Read, field_name: &str) -> Result<U4, Error> {
    let result = reader
        .read_u32::<ClassFileByteOrder>()
        .map_err(|e| Error::ReadPrimitive {
            field_name: field_name.to_string(),
            source: e,
        })?;
    println!("{field_name}: {result}");

    Ok(result)
}
