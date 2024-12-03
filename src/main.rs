use std::fs::File;
use std::io::{BufReader, Read};

use mml_cli_main::impl_cli_main;

mod args;
mod attributes;
mod constant_pool;
mod fields;
mod error;
mod methods;
mod primitives;

use crate::args::*;
use crate::attributes::print_attributes;
use crate::constant_pool::*;
use crate::error::*;
use crate::fields::print_fields;
use crate::methods::print_methods;
use crate::primitives::*;

const CORRECT_MAGIC: U4 = 0xCAFEBABE;

const MIN_MAJOR_VERSION: U2 = 45;
const MAX_MAJOR_VERSION: U2 = 61;

impl_cli_main!();

pub fn run(args: &Args) -> Result<(), Error> {
    println!("input_file_path: {}", args.input_file_path.display());
    let file = File::open(&args.input_file_path).map_err(Error::FileOpen)?;
    println!("file opened");
    let mut reader = BufReader::new(file);

    println!();

    let magic = print_u4(&mut reader, "/magic")?;
    if magic != CORRECT_MAGIC {
        return Err(Error::IncorrectMagicNumber(magic));
    }
    println!("magic number was correct (0x{CORRECT_MAGIC:X?})");

    let minor_version = print_u2(&mut reader, "/minor_version")?;

    let major_version = print_u2(&mut reader, "/major_version")?;
    if !(MIN_MAJOR_VERSION..=MAX_MAJOR_VERSION).contains(&major_version) {
        return Err(Error::UnsupportedMajorVersion(major_version));
    }

    let constant_pool = print_constant_pool(&mut reader)?;
    let access_flags = print_u2(&mut reader, "/access_flags")?;
    let this_class = print_u2(&mut reader, "/this_class")?;
    let super_class = print_u2(&mut reader, "/super_class")?;
    let interfaces = print_interfaces(&mut reader)?;
    let fields = print_fields(&mut reader)?;
    let methods = print_methods(&mut reader)?;
    let attributes = print_attributes(&mut reader, "")?;

    Ok(())
}

fn print_buffer(
    reader: &mut impl Read,
    buffer_name: &str,
    buffer_len: usize,
) -> Result<Vec<U1>, Error> {
    let mut buffer = vec![0; buffer_len];
    reader
        .read_exact(&mut buffer)
        .map_err(|e| Error::ReadBuffer {
            buffer_name: format!("{buffer_name}/{:?}", buffer),
            source: e,
        })?;
    println!("{buffer_name}: {:?}", &buffer);

    Ok(buffer)
}

fn print_interfaces(reader: &mut impl Read) -> Result<Vec<U1>, Error> {
    let interfaces_count = print_u2(reader, "/interfaces_count")?;

    print_buffer(reader, "/interfaces", interfaces_count.into())
}

#[cfg(test)]
mod tests;


