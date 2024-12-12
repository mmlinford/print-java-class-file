use std::fmt::{Display, Formatter};
use std::io::Read;

use crate::error::Error;
use crate::modified_utf8::ModifiedUtf8String;
use crate::primitives::{print_u1, print_u2, U1, U2};
use crate::print_buffer;

pub const MIN_CONST_POOL_COUNT: U2 = 1;

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
    let constant_kind: ConstantKind = tag.try_into().map_err(Error::TryConstantKindFromU1)?;
    println!("/constant_pool[{constant_pool_index}] -> {constant_kind}");

    use ConstantKind::*;
    match constant_kind {
        Utf8 => print_utf8_cp_info(reader, constant_pool_index),
        Class => print_class_cp_info(reader, constant_pool_index),
        Methodref => print_method_ref_cp_info(reader, constant_pool_index),
        NameAndType => print_name_and_type_cp_info(reader, constant_pool_index),
    }
}

pub fn print_utf8_cp_info(
    reader: &mut impl Read,
    constant_pool_index: U2,
) -> Result<CpInfo, Error> {
    let name_root = format!("/constant_pool[{constant_pool_index}]/utf8");
    let length = print_u2(reader, &format!("{name_root}/length"))?;
    let bytes = print_buffer(reader, &format!("{name_root}/bytes"), length.into())?;

    let modified_utf8 = bytes.try_into().map_err(Error::InvalidUtf8Constant)?;
    println!("{name_root} -> {modified_utf8}");

    Ok(CpInfo::Utf8(modified_utf8))
}

pub fn print_class_cp_info(
    reader: &mut impl Read,
    constant_pool_index: U2,
) -> Result<CpInfo, Error> {
    let name_index = print_u2(
        reader,
        &format!("/constant_pool[{constant_pool_index}]/class/name_index"),
    )?;

    Ok(CpInfo::Class { name_index })
}

pub fn print_method_ref_cp_info(
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

pub fn print_name_and_type_cp_info(
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
    Utf8(ModifiedUtf8String),

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

pub const UTF8_CP_INFO_TAG: U1 = 1;
pub const CLASS_CP_INFO_TAG: U1 = 7;
pub const METHOD_REF_CP_INFO_TAG: U1 = 10;
pub const NAME_AND_TYPE_CP_INFO_TAG: U1 = 12;

#[derive(Clone, Copy)]
pub enum ConstantKind {
    Utf8,
    Class,
    Methodref,
    NameAndType,
}

impl ConstantKind {
    pub const fn tag(self) -> U1 {
        use ConstantKind::*;
        match self {
            Utf8 => UTF8_CP_INFO_TAG,
            Class => CLASS_CP_INFO_TAG,
            Methodref => METHOD_REF_CP_INFO_TAG,
            NameAndType => NAME_AND_TYPE_CP_INFO_TAG,
        }
    }
}

impl Display for ConstantKind {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        use ConstantKind::*;
        let as_str = match self {
            Utf8 => "Utf8",
            Class => "Class",
            Methodref => "Methodref",
            NameAndType => "NameAndType",
        };

        f.write_str(as_str)
    }
}

impl TryFrom<U1> for ConstantKind {
    type Error = TryConstantKindFromU1Error;

    fn try_from(value: U1) -> Result<Self, Self::Error> {
        use ConstantKind::*;
        match value {
            UTF8_CP_INFO_TAG => Ok(Utf8),
            CLASS_CP_INFO_TAG => Ok(Class),
            METHOD_REF_CP_INFO_TAG => Ok(Methodref),
            NAME_AND_TYPE_CP_INFO_TAG => Ok(NameAndType),
            unsupported => Err(TryConstantKindFromU1Error(unsupported)),
        }
    }
}

#[derive(Debug, thiserror::Error)]
#[error("Unrecognized or unsupported constant tag: {0}")]
pub struct TryConstantKindFromU1Error(U1);
