use std::fs::File;
use std::io::{BufReader, Read};

use mml_cli_main::impl_cli_main;

mod access_flags;
mod args;
mod attributes;
mod constant_pool;
mod error;
mod fields;
mod java_version;
mod methods;
mod modified_utf8;
mod primitives;

use crate::args::*;
use crate::attributes::*;
use crate::constant_pool::*;
use crate::error::*;
use crate::fields::*;
use crate::java_version::*;
use crate::methods::*;
use crate::modified_utf8::*;
use crate::primitives::*;

const CORRECT_MAGIC: U4 = 0xCAFEBABE;

impl_cli_main!();

pub fn run(args: &Args) -> Result<(), Error> {
    println!("input_file_path: {}", args.input_file_path.display());
    let file = File::open(&args.input_file_path).map_err(Error::FileOpen)?;
    println!("file opened");
    let mut reader = BufReader::new(file);

    println!();

    let magic = read_u4(&mut reader, "/magic")?;
    println!("/magic: {:#04X?}", magic);
    if magic != CORRECT_MAGIC {
        return Err(Error::IncorrectMagicNumber(magic));
    }
    println!("magic number was correct ({CORRECT_MAGIC:#04X?})");

    let minor_version = print_u2(&mut reader, "/minor_version")?;

    let major_version = print_u2(&mut reader, "/major_version")?;
    let java_version = JavaVersion::try_from(major_version).map_err(Error::TryJavaVersionFromU2)?;
    println!("Java version (derived from clas file major version): {java_version}");

    let constant_pool = print_constant_pool(&mut reader)?;
    let access_flags = print_u2(&mut reader, "/access_flags")?;
    let this_class = print_u2(&mut reader, "/this_class")?;
    let super_class = print_u2(&mut reader, "/super_class")?;
    let interfaces = print_interfaces(&mut reader)?;
    let fields = print_fields(&mut reader)?;
    let methods = print_methods(&mut reader)?;
    let (attributes_count, attributes) = print_attributes(&mut reader, "")?;

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
            buffer_name: buffer_name.to_string(),
            source: e,
        })?;

    print!("{buffer_name}: [");
    for byte in &buffer {
        print!(" {byte:#04X?}");
    }
    println!(" ]");

    Ok(buffer)
}

fn print_interfaces(reader: &mut impl Read) -> Result<Vec<U1>, Error> {
    let interfaces_count = print_u2(reader, "/interfaces_count")?;

    print_buffer(reader, "/interfaces", interfaces_count.into())
}
