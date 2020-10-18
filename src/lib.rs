//! # nom-pdb
//!
//! A PDB (Protein Data Bank) file parser implemented with nom.
//!
//! See [github repository](https://github.com/TianyiShi2001/nom-pdb) for examples.

pub mod aux;
pub mod common;
pub mod complete;
pub mod coordinate;
pub mod crystallography;
pub mod parserbuilder;
pub mod primary_structure;
pub mod remark;
pub mod secondary_structure;
pub mod title_section;
pub(crate) mod types;

// use std::str::from_utf8_unchecked;

// use std::fs::read;
// use std::fs::read_to_string;
// use std::fs::File;

// pub enum Record<'a> {
//     Header(title_section::header::Header<'a>),
//     Authors(Vec<&'a str>),
//     Keywords(Vec<&'a str>),
//     Cryst1(crystallography::cryst1::Cryst1),
// }

// use memmap::MmapOptions;
// pub unsafe fn apply_file_content_unsafe<F, T>(fp: &str, parser: F) -> Result<T, std::io::Error>
// where
//     F: FnOnce(&str) -> T,
// {
//     let file = File::open(fp)?;
//     let mmap = MmapOptions::new().map(&file)?;
//     let data = from_utf8_unchecked(&mmap[..]);
//     let res = parser(data);
//     Ok(res)
// }

// pub fn apply_file_content<F, T>(fp: &str, parser: F) -> Result<T, std::io::Error>
// where
//     F: FnOnce(&str) -> T,
// {
//     let bytes = read(fp)?;
//     let data = unsafe { from_utf8_unchecked(&bytes) };
//     let res = parser(data);
//     Ok(res)
// }
