use std::io::Read;

use crate::error::Error;

use crate::primitives::{print_u1, print_u2, U1, U2};
use crate::print_buffer;

const MIN_CONST_POOL_COUNT: U2 = 1;

const UTF8_CP_INFO_TAG: U1 = 1;
const CLASS_CP_INFO_TAG: U1 = 7;
const METHOD_REF_CP_INFO_TAG: U1 = 10;
const NAME_AND_TYPE_CP_INFO_TAG: U1 = 12;

pub fn print_constant_pool_count(reader: &mut impl Read) -> Result<U2, Error> {
    let constant_pool_count = print_u2(reader, "/constant_pool_count")?;
    if constant_pool_count < MIN_CONST_POOL_COUNT {
        return Err(Error::InvalidConstantPoolCount(constant_pool_count));
    }

    Ok(constant_pool_count)
}

pub fn print_cp_item(reader: &mut impl Read, constant_pool_index: U2) -> Result<CpInfo, Error> {
    let tag = print_u1(
        reader,
        &format!("/constant_pool[{constant_pool_index}]/tag"),
    )?;
    match tag {
        UTF8_CP_INFO_TAG => print_utf8_cp_info(reader, constant_pool_index),
        CLASS_CP_INFO_TAG => print_class_cp_info(reader, constant_pool_index),
        METHOD_REF_CP_INFO_TAG => print_method_ref_cp_info(reader, constant_pool_index),
        NAME_AND_TYPE_CP_INFO_TAG => print_name_and_type_cp_info(reader, constant_pool_index),
        unsupported => Err(Error::UnsupportedCpInfoTag(unsupported)),
    }
}

fn print_utf8_cp_info(reader: &mut impl Read, constant_pool_index: U2) -> Result<CpInfo, Error> {
    let name_root = format!("/constant_pool[{constant_pool_index}]/utf8");
    let length = print_u2(reader, &format!("{name_root}/length"))?;
    let bytes = print_buffer(reader, &format!("{name_root}/bytes"), length.into())?;

    Ok(CpInfo::Utf8 { length, bytes })
}

fn print_class_cp_info(reader: &mut impl Read, constant_pool_index: U2) -> Result<CpInfo, Error> {
    let name_index = print_u2(
        reader,
        &format!("/constant_pool[{constant_pool_index}]/class/name_index"),
    )?;

    Ok(CpInfo::Class { name_index })
}

fn print_method_ref_cp_info(
    reader: &mut impl Read,
    constant_pool_index: U2,
) -> Result<CpInfo, Error> {
    let name_root = format!("/constant_pool[{constant_pool_index}]/methodref");
    let class_index = print_u2(reader, &format!("{name_root}/class_index"))?;
    let name_and_type_index = print_u2(reader, &format!("{name_root}/name_and_type_index"))?;

    Ok(CpInfo::MethodrefInfo {
        class_index,
        name_and_type_index,
    })
}

fn print_name_and_type_cp_info(
    reader: &mut impl Read,
    constant_pool_index: U2,
) -> Result<CpInfo, Error> {
    let name_root = format!("/constant_pool[{constant_pool_index}]/name_and_type");
    let name_index = print_u2(reader, &format!("{name_root}/name_index"))?;
    let descriptor_index = print_u2(reader, &format!("{name_root}/descriptor_index"))?;

    Ok(CpInfo::NameAndType {
        name_index,
        descriptor_index,
    })
}

pub fn print_constant_pool(reader: &mut impl Read) -> Result<Vec<CpInfo>, Error> {
    let constant_pool_count = print_constant_pool_count(reader)?;
    let mut constant_pool = Vec::with_capacity((constant_pool_count - 1).into());
    for constant_pool_index in 1..constant_pool_count {
        constant_pool.push(print_cp_item(reader, constant_pool_index)?);
    }

    Ok(constant_pool)
}

pub enum CpInfo {
    Utf8 {
        length: U2,
        bytes: Vec<U1>,
    },

    Class {
        name_index: U2,
    },

    MethodrefInfo {
        class_index: U2,
        name_and_type_index: U2,
    },

    NameAndType {
        name_index: U2,
        descriptor_index: U2,
    },
}
