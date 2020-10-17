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

use crate::common::parser::{parse_amino_acid, parse_right, FieldParser};
use crate::types::*;
use nom::{
    bytes::complete::take,
    character::complete::{anychar, line_ending},
    combinator::{map, peek},
    IResult,
};

use std::collections::HashMap;

pub struct ModresParser;

impl FieldParser for ModresParser {
    type Output = Modres;
    fn parse(inp: &str) -> IResult<&str, Modres> {
        let mut res = HashMap::new();
        let mut inp = inp;
        loop {
            let (i, _) = Self::parse_oneline(inp, &mut res)?;
            if peek(take(6usize))(i)?.1 != "MODRES" {
                return Ok((i, res));
            }
            let (i, _) = take(6usize)(i)?;
            inp = i;
        }
    }
}

impl ModresParser {
    pub fn parse_oneline<'a>(
        inp: &'a str,
        hashmap: &mut HashMap<String, CustomAminoAcid>,
    ) -> IResult<&'a str, ()> {
        let (inp, _) = take(6usize)(inp)?;
        let (inp, name) = map(take(3usize), str::to_owned)(inp)?;
        let (inp, _) = take(1usize)(inp)?;
        let (inp, chain) = anychar(inp)?;
        let (inp, _) = take(1usize)(inp)?;
        let (inp, sequence_number) = parse_right::<u32>(inp, 4usize)?;
        let (inp, insertion_code) = anychar(inp)?;
        let (inp, _) = take(1usize)(inp)?;
        let (inp, standard_res) = parse_amino_acid(inp)?;
        let (inp, _) = take(2usize)(inp)?;
        let (inp, description) = map(map(take(51usize), str::trim_end), str::to_owned)(inp)?;
        let (inp, _) = line_ending(inp)?;

        let aa = hashmap.entry(name).or_insert(CustomAminoAcid {
            standard_res,
            description,
            occurence: Vec::new(),
        });
        aa.occurence.push((chain, sequence_number));

        Ok((inp, ()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_modres() {
        let inp = " 1A8O MSE A  151  MET  SELENOMETHIONINE                                   
MODRES 1A8O MSE A  185  MET  SELENOMETHIONINE                                   
MODRES 1A8O FOO A  214  MET  FOOBARBAZATONINE                                   
MODRES 1A8O FOO A  215  MET  FOOBARBAZATONINE                                   
XXXXXX ...";
        let (i, modres) = ModresParser::parse(inp).unwrap();
        assert_eq!("XXXXXX ...", i);
        assert_eq!(modres.get("FOO").unwrap().occurence.len(), 2usize);
        assert_eq!(&modres.get("FOO").unwrap().description, "FOOBARBAZATONINE");
    }
}
