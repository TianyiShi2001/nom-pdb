pub mod common;
pub mod complete;
pub mod coordinate;
pub mod crystallography;
pub mod title_section;
use crate::complete::Pdb;
// use crate::common::parser::parse_multiline_string;
use std::str::from_utf8_unchecked;

fn main() {
    //     let (i, r) = parse_multiline_string(
    //         "    ACID PROTEINASE (PENICILLOPEPSIN) (E.C.3.4.23.20) COMPLEX WITH
    // TITLE    2 PHOSPHONATE INHIBITOR: METHYL CYCLO[(2S)-2-[[(1R)-1-(N-(L-N-(3-
    // TITLE    3 METHYLBUTANOYL)VALYL-L-ASPARTYL)AMINO)-3-METHYLBUT YL]
    // TITLE    4 HYDROXYPHOSPHINYLOXY]-3-(3-AMINOMETHYL) PHENYLPROPANOATE
    // COMPND    MOL_ID: 1;                                                            ",
    //         "TITLE ",
    //     )
    //     .unwrap();
    //     println!("{:?},{:?}", i, r);
    let data = include_bytes!("../assets/4F7I.pdb");
    unsafe {
        let data = from_utf8_unchecked(data);
        let (data, r) = complete::Pdb::parse(data).unwrap();
        println!("{:?}, {:?}", data, r);
    }
}
