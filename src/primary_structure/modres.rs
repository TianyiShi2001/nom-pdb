// Copyright (c) 2020 Tianyi Shi
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

//! The [MODRES](http://www.wwpdb.org/documentation/file-format-content/format33/sect3.html#MODRES)
//! record provides descriptions of modifications (e.g., chemical or post-translational) to protein
//! and nucleic acid residues. Included are correlations between residue names given in a PDB entry
//! and standard residues.
//!
//! # Record Format
//!
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

//
// * MODRES comes after SEQRES, thus non-standard residue names in SEQRES cannot be identified
//   directly

use crate::{
    common::parser::parse_right,
    types::{
        ModifiedAminoAcid, ModifiedAminoAcidTable, ModifiedNucleotide, ModifiedNucleotideTable,
        StandardAminoAcid, StandardNucleotide, TryParseFw3,
    },
};
use nom::{
    bytes::complete::take,
    character::complete::{anychar, line_ending},
    IResult,
};

pub struct ModresParser;

// impl  ModresParser {
//     fn parse_into_structure(inp: &[u8], structure: &mut Structure) -> IResult<&[u8], ()> {
//         let mut res = HashMap::new();
//         let mut inp = inp;
//         loop {
//             let (i, _) = Self::parse_oneline(inp, &mut res)?;
//             if peek(take(6usize))(i)?.1 != "MODRES" {
//                 return Ok((i, res));
//             }
//             let (i, _) = take(6usize)(i)?;
//             inp = i;
//         }
//     }
// }

impl ModresParser {
    pub fn parse_into<'a>(
        inp: &'a [u8],
        modified_aa: &mut ModifiedAminoAcidTable,
        modified_nuc: &mut ModifiedNucleotideTable,
    ) -> IResult<&'a [u8], ()> {
        let inp = &inp[6..];
        let (inp, name) = take(3usize)(inp)?;
        let name = unsafe { std::str::from_utf8_unchecked(name).to_owned() };
        let inp = &inp[1..];
        let (inp, _chain) = anychar(inp)?;
        let inp = &inp[1..];
        let (inp, _sequence_number) = parse_right::<u32>(inp, 4usize)?;
        let (inp, _insertion_code) = anychar(inp)?;
        let inp = &inp[1..];
        let (inp, standard_res) = take(3usize)(inp)?;

        let inp = &inp[2..];
        let (inp, description) = take(51usize)(inp)?;
        let description = unsafe {
            std::str::from_utf8_unchecked(description)
                .trim_end()
                .to_owned()
        };
        if let Some(standard) = StandardAminoAcid::try_parse_fw3(standard_res) {
            modified_aa.insert(
                name,
                ModifiedAminoAcid {
                    standard,
                    description,
                },
            );
        } else if let Some(standard) = StandardNucleotide::try_parse_fw3(standard_res) {
            modified_nuc.insert(
                name,
                ModifiedNucleotide {
                    standard,
                    description,
                },
            );
        } else {
            panic!(format!("Mapping modified residue to standard residue, but encountered invalid standard residue: {:?}", std::str::from_utf8(standard_res).unwrap()))
        }
        let (inp, _) = line_ending(inp)?;

        Ok((inp, ()))
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//     #[test]
//     fn test_modres() {
//         let inp = " 1A8O MSE A  151  MET  SELENOMETHIONINE
// MODRES 1A8O MSE A  185  MET  SELENOMETHIONINE
// MODRES 1A8O FOO A  214  MET  FOOBARBAZATONINE
// MODRES 1A8O FOO A  215  MET  FOOBARBAZATONINE
// XXXXXX ...";
//         let (i, modres) = ModresParser::parse(inp).unwrap();
//         assert_eq!("XXXXXX ...", i);
//         assert_eq!(modres.get("FOO").unwrap().occurence.len(), 2usize);
//         assert_eq!(&modres.get("FOO").unwrap().description, "FOOBARBAZATONINE");
//     }
// }
