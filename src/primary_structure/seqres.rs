//! SEQRES records contain a listing of the consecutive chemical components covalently linked in a linear fashion to form a polymer. The chemical components included in this listing may be standard or modified amino acid and nucleic acid residues. It may also include other residues that are linked to the standard backbone in the polymer. Chemical components or groups covalently linked to side-chains (in peptides) or sugars and/or bases (in nucleic acid polymers) will not be listed here.
//! # Record Format
//! | COLUMNS | DATA TYPE    | FIELD    | DEFINITION                                                                                                                        |
//! | ------- | ------------ | -------- | --------------------------------------------------------------------------------------------------------------------------------- |
//! | 1 -  6  | Record name  | "SEQRES" |                                                                                                                                   |
//! | 8 - 10  | Integer      | serNum   | Serial number of the SEQRES record for the current chain. Starts at 1 and increments by one each line. Reset to 1 for each chain. |
//! | 12      | Character    | chainID  | Chain identifier. This may be any single legal character, including a blank which is used if there is only one chain.             |
//! | 14 - 17 | Integer      | numRes   | Number of residues in the chain. This  value is repeated on every record.                                                         |
//! | 20 - 22 | Residue name | resName  | Residue name.                                                                                                                     |
//! | 24 - 26 | Residue name | resName  | Residue name.                                                                                                                     |
//! | 28 - 30 | Residue name | resName  | Residue name.                                                                                                                     |
//! | 32 - 34 | Residue name | resName  | Residue name.                                                                                                                     |
//! | 36 - 38 | Residue name | resName  | Residue name.                                                                                                                     |
//! | 40 - 42 | Residue name | resName  | Residue name.                                                                                                                     |
//! | 44 - 46 | Residue name | resName  | Residue name.                                                                                                                     |
//! | 48 - 50 | Residue name | resName  | Residue name.                                                                                                                     |
//! | 52 - 54 | Residue name | resName  | Residue name.                                                                                                                     |
//! | 56 - 58 | Residue name | resName  | Residue name.                                                                                                                     |
//! | 60 - 62 | Residue name | resName  | Residue name.                                                                                                                     |
//! | 64 - 66 | Residue name | resName  | Residue name.                                                                                                                     |
//! | 68 - 70 | Residue name | resName  | Residue name.                                                                                                                     |

use crate::common::parser::{parse_amino_acid, parse_right_u32, FieldParser};
use crate::common::types::AminoAcid;
use nom::bytes::complete::take;
use nom::character::complete::{anychar, multispace1};
use nom::combinator::peek;
use nom::IResult;

pub type SeqRes = Vec<(char, Vec<AminoAcid>)>;
pub struct SeqResParser;
impl FieldParser for SeqResParser {
    type Output = SeqRes;
    fn parse(inp: &str) -> IResult<&str, SeqRes> {
        let mut v: Vec<(char, Vec<AminoAcid>)> = Vec::new();
        let (mut inp, _) = take(6usize)(inp)?; // very first line
        loop {
            let (i, chain) = parse_chain(inp)?;
            v.push(chain);
            match peek(take(6usize))(i)?.1 {
                "SEQRES" => inp = take(6usize)(i)?.0,
                _ => return Ok((i, v)),
            }
        }
    }
}
pub fn parse_chain(inp: &str) -> IResult<&str, (char, Vec<AminoAcid>)> {
    let (inp, _) = take(5usize)(inp)?; // first line 7 - 11
    let (inp, chain) = anychar(inp)?; // first line 12
    let (inp, _) = take(1usize)(inp)?; // first line 13
    let (inp, n) = parse_right_u32(inp, 4)?; // first line 14 - 17
    let (inp, _) = take(2usize)(inp)?; // first line 18 - 19
    let lines = n / 13u32;
    let last_line_items = n % 13u32;
    let mut amino_acids: Vec<AminoAcid> = Vec::new();
    let mut inp = inp;
    for i in 0..lines {
        for j in 0..13 {
            let (inp1, aa) = parse_amino_acid(inp)?;
            amino_acids.push(aa);
            inp = take(1usize)(inp1)?.0;
        }
        inp = multispace1(inp)?.0;
        inp = take(19usize)(inp)?.0;
    }
    for i in 0..last_line_items {
        let (inp1, aa) = parse_amino_acid(inp)?;
        amino_acids.push(aa);
        inp = take(1usize)(inp1)?.0;
    }
    inp = multispace1(inp)?.0; // newline
    Ok((inp, (chain, amino_acids)))
}
