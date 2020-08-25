pub mod common;
pub mod complete;
pub mod coordinate;
pub mod crystallography;
pub mod primary_structure;
pub mod title_section;
use crate::complete::Pdb;
use common::parser::FieldParserComplete;
use primary_structure::seqres::SeqResParserComplete;
// use crate::common::parser::parse_multiline_string;
use std::str::from_utf8_unchecked;

fn main() {
    // .unwrap();
    // println!("{:?},{:?}", i, r);
    let data = include_bytes!("../assets/3SE5.pdb");
    unsafe {
        let data = from_utf8_unchecked(data);
        let (data, r) = Pdb::parse(data).unwrap();
        println!("{:?}, {:?}", data, r);
    }
}
