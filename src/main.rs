pub mod common;
pub mod complete;
pub mod coordinate;
pub mod crystallography;
pub mod primary_structure;
pub mod title_section;
use crate::complete::Pdb;
use common::parser::FieldParser;
use primary_structure::seqres::SeqResParser;
// use crate::common::parser::parse_multiline_string;
use std::str::from_utf8_unchecked;

use nom_pdb::{apply_file_content, apply_file_content_unsafe};
use std::fs::read_to_string;

fn main() {
    // .unwrap();
    // println!("{:?},{:?}", i, r);
    // let data = include_bytes!("../assets/4F7I.pdb");
    // unsafe {
    //     let data = from_utf8_unchecked(data);
    //     let (data, r) = Pdb::parse(data).unwrap();
    //     println!("{:?}, {:?}", data, r);
    // }
    unsafe {
        apply_file_content_unsafe("assets/4F7I.pdb", |x| {
            println!("{:?}", x);
        })
        .unwrap();
    }
}
