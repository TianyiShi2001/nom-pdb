pub mod common;
pub mod complete;
pub mod coordinate;
pub mod crystallography;
pub mod partial;
pub mod primary_structure;
pub mod title_section;
use std::str::from_utf8_unchecked;

use std::fs::read;
use std::fs::read_to_string;

use memmap::MmapOptions;
use std::fs::File;
use std::io::Write;

// pub enum Record<'a> {
//     Header(title_section::header::Header<'a>),
//     Authors(Vec<&'a str>),
//     Keywords(Vec<&'a str>),
//     Cryst1(crystallography::cryst1::Cryst1),
// }

pub unsafe fn apply_file_content_unsafe<F, T>(fp: &str, parser: F) -> Result<T, std::io::Error>
where
    F: FnOnce(&str) -> T,
{
    let file = File::open(fp)?;
    let mmap = MmapOptions::new().map(&file)?;
    let data = from_utf8_unchecked(&mmap[..]);
    let res = parser(data);
    Ok(res)
}

pub fn apply_file_content<F, T>(fp: &str, parser: F) -> Result<T, std::io::Error>
where
    F: FnOnce(&str) -> T,
{
    let bytes = read(fp)?;
    let data = unsafe { from_utf8_unchecked(&bytes) };
    let res = parser(data);
    Ok(res)
}
