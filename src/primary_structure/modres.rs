//! The [MODRES](http://www.wwpdb.org/documentation/file-format-content/format33/sect3.html#MODRES)
//! record provides descriptions of modifications (e.g., chemical or post-translational) to protein and nucleic acid residues. Included are correlations between residue names given
//! in a PDB entry and standard residues.
//! # Record Format
//! | COLUMNS | DATA TYPE    | FIELD    | DEFINITION                               |
//! | ------- | ------------ | -------- | ---------------------------------------- |
//! | 1 -  6  | Record name  | "MODRES" |                                          |
//! | 8 - 11  | IDcode       | idCode   | ID code of this entry.                   |
//! | 13 - 15 | Residue name | resName  | Residue name used in this entry.         |
//! | 17      | Character    | chainID  | Chain identifier.                        |
//! | 19 - 22 | Integer      | seqNum   | Sequence number.                         |
//! | 23      | AChar        | iCode    | Insertion code.                          |
//! | 25 - 27 | Residue name | stdRes   | Standard residue name.                   |
//! | 30 - 70 | String       | comment  | Description of the residue modification. |

// * MODRES comes after SEQRES, thus non-standard residue names in SEQRES cannot be identified directly


use crate::common::parser::FieldParserComplete;
use crate::common::parser::{parse_amino_acid, parse_right_f32, parse_right_i8, parse_right_u32};
use crate::common::types::AminoAcid;
use nom::bytes::complete::take;
use nom::character::complete::anychar;
use nom::combinator::map;
use nom::IResult;
use std::str::FromStr;
#[derive(Debug, Clone)]
pub struct Modres {
    name: String,
    chain: char,
    sequence_number: u32,
    insertion_code: char,
    standard_name: String,
    description: String,
}

pub struct ModresParserComplete;

impl FieldParserComplete for ModresParserComplete {
    type Output = Modres;
    fn parse(inp: &str) -> IResult<&str, Modres> {
        let (inp, _) = take(6)(inp)?;
        let (inp, name) = map(take(3), str::to_owned)(inp)?;
        let (inp, _) = take(1)(inp)?;
        let (inp, chain) = anychar(inp)?;
        let (inp, _) = take(1)(inp)?;
        let (inp, sequence_number) = parse_right_u32(inp, 4)?;
        let (inp, insertion_code) = anychar(inp)?;
        let (inp, standard_name)
    }