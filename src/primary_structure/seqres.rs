// //! SEQRES records contain a listing of the consecutive chemical components covalently linked
// //! in a linear fashion to form a polymer. The chemical components included in this listing may
// //! be standard or modified amino acid and nucleic acid residues. It may also include other residues
// //! that are linked to the standard backbone in the polymer. Chemical components or groups covalently
// //! linked to side-chains (in peptides) or sugars and/or bases (in nucleic acid polymers) will not be
// //! listed here.
// //!
// //! # Record Format
// //!
// //! | COLUMNS | DATA TYPE    | FIELD    | DEFINITION                                                                                                                        |
// //! | ------- | ------------ | -------- | --------------------------------------------------------------------------------------------------------------------------------- |
// //! | 1 -  6  | Record name  | "SEQRES" |                                                                                                                                   |
// //! | 8 - 10  | Integer      | serNum   | Serial number of the SEQRES record for the current chain. Starts at 1 and increments by one each line. Reset to 1 for each chain. |
// //! | 12      | Character    | chainID  | Chain identifier. This may be any single legal character, including a blank which is used if there is only one chain.             |
// //! | 14 - 17 | Integer      | numRes   | Number of residues in the chain. This  value is repeated on every record.                                                         |
// //! | 20 - 22 | Residue name | resName  | Residue name.                                                                                                                     |
// //! | 24 - 26 | Residue name | resName  | Residue name.                                                                                                                     |
// //! | 28 - 30 | Residue name | resName  | Residue name.                                                                                                                     |
// //! | 32 - 34 | Residue name | resName  | Residue name.                                                                                                                     |
// //! | 36 - 38 | Residue name | resName  | Residue name.                                                                                                                     |
// //! | 40 - 42 | Residue name | resName  | Residue name.                                                                                                                     |
// //! | 44 - 46 | Residue name | resName  | Residue name.                                                                                                                     |
// //! | 48 - 50 | Residue name | resName  | Residue name.                                                                                                                     |
// //! | 52 - 54 | Residue name | resName  | Residue name.                                                                                                                     |
// //! | 56 - 58 | Residue name | resName  | Residue name.                                                                                                                     |
// //! | 60 - 62 | Residue name | resName  | Residue name.                                                                                                                     |
// //! | 64 - 66 | Residue name | resName  | Residue name.                                                                                                                     |
// //! | 68 - 70 | Residue name | resName  | Residue name.                                                                                                                     |

use crate::common::parser::{
    jump_newline, parse_residue, parse_right, FieldParserWithModifiedTable,
};
use nom::bytes::complete::take;
use nom::character::complete::{anychar, line_ending, multispace1, not_line_ending};
use nom::combinator::map;
use nom::IResult;
use protein_core::types::{
    AminoAcid, Chain, ModifiedAminoAcid, ModifiedNucleotide, Nucleotide, Residue,
};
use std::collections::HashMap;

type SeqRes = Vec<(char, Vec<Residue>)>;
pub struct SeqResParser;
impl SeqResParser {
    pub fn parse<'a>(
        inp: &'a [u8],
        modified_aa: &HashMap<String, ModifiedAminoAcid>,
        modified_nuc: &HashMap<String, ModifiedNucleotide>,
    ) -> IResult<(), (Vec<Chain<AminoAcid>>, Vec<Chain<Nucleotide>>)> {
        // let (mut inp, _) = take(6usize)(inp)?; // very first line
        let mut inp = inp;
        // loop {
        //     let (i, chain) = Self::parse_chain(inp, modified_aa, modified_nuc)?;
        //     v.push(chain);
        //     match peek(take(6usize))(i)?.1 {
        //         b"SEQRES" => inp = take(6usize)(i)?.0, // TODO: CHANGE
        //         _ => return Ok((i, v)),
        //     }
        // }
        let mut chains_aa: Vec<Chain<AminoAcid>> = Vec::new();
        let mut chains_nuc: Vec<Chain<Nucleotide>> = Vec::new();
        while inp.len() > 0 {
            let (new_inp, _) = Self::parse_chain(
                inp,
                modified_aa,
                modified_nuc,
                &mut chains_aa,
                &mut chains_nuc,
            )
            .unwrap(); // TODO: remove unwrap
            inp = new_inp;
        }
        Ok(((), (chains_aa, chains_nuc)))
    }
}

impl SeqResParser {
    pub fn parse_chain<'a>(
        inp: &'a [u8],
        modified_aa: &HashMap<String, ModifiedAminoAcid>,
        modified_nuc: &HashMap<String, ModifiedNucleotide>,
        chains_aa: &mut Vec<Chain<AminoAcid>>,
        chains_nuc: &mut Vec<Chain<Nucleotide>>,
    ) -> IResult<&'a [u8], ()> {
        // println!("{}", unsafe { std::str::from_utf8_unchecked(inp) });
        let (inp, _) = take(5usize)(inp)?; // first line 7 - 11
        let (inp, chain) = anychar(inp)?; // first line 12
        let (inp, _) = take(1usize)(inp)?; // first line 13
        let (inp, n) = parse_right::<u32>(inp, 4)?; // first line 14 - 17
        let (inp, _) = take(2usize)(inp)?; // first line 18 - 19
        let lines = n / 13u32;
        let last_line_items = n % 13u32;
        let mut inp = inp;

        let first_res = &inp[..3];
        //println!("{:?}", (chain as char, n, lines));
        // try parse
        match parse_residue(inp, modified_aa, modified_nuc)?.1 {
            Residue::AminoAcid(_) => {
                let mut aas: Vec<AminoAcid> = Vec::new();
                for _i in 0..lines {
                    for _j in 0..13 {
                        let (inp1, res) = map(take(3usize), AminoAcid::from_bytes_uppercase)(inp)?;
                        aas.push(res);
                        inp = take(1usize)(inp1)?.0;
                    }
                    //inp = multispace1(inp)?.0;
                    inp = jump_newline(inp)?.0;
                    inp = take(13usize)(inp)?.0;
                }
                for i in 0..last_line_items {
                    let (inp1, res) = map(take(3usize), AminoAcid::from_bytes_uppercase)(inp)?;
                    aas.push(res);
                    inp = take(1usize)(inp1)?.0;
                }
                //inp = multispace1(inp)?.0; // newline
                inp = jump_newline(inp)?.0;
                chains_aa.push(Chain {
                    id: chain,
                    seq: aas,
                });
                return Ok((inp, ()));
            }
            Residue::Nucleotide(_) => {
                let mut nucs: Vec<Nucleotide> = Vec::new();
                for _i in 0..lines {
                    for _j in 0..13 {
                        let (inp1, res) =
                            map(take(3usize), Nucleotide::from_bytes_uppercase_fixed3)(inp)?;
                        nucs.push(res);
                        inp = take(1usize)(inp1)?.0;
                    }
                    //inp = multispace1(inp)?.0;
                    inp = jump_newline(inp)?.0;
                    inp = take(13usize)(inp)?.0;
                }
                for i in 0..last_line_items {
                    let (inp1, res) =
                        map(take(3usize), Nucleotide::from_bytes_uppercase_fixed3)(inp)?;
                    nucs.push(res);
                    inp = take(1usize)(inp1)?.0;
                }
                //inp = multispace1(inp)?.0; // newline
                inp = jump_newline(inp)?.0;
                chains_nuc.push(Chain {
                    id: chain,
                    seq: nucs,
                });
                return Ok((inp, ()));
            }
            _ => panic!(format!("Invalid residue in chain: {}", unsafe {
                std::str::from_utf8_unchecked(first_res)
            })),
        }
    }
    pub fn buffer_seqres<'a>(inp: &'a [u8], buffer: &mut Vec<u8>) -> IResult<&'a [u8], ()> {
        let (inp, first_line) = not_line_ending(inp)?;
        let (mut inp, _) = line_ending(inp)?;
        buffer.extend_from_slice(&first_line);
        buffer.push(b'\n');
        while inp[..6] == b"SEQRES"[..] {
            let (new_inp, ln) = not_line_ending(inp)?;
            let (new_inp, _) = line_ending(new_inp)?;
            buffer.extend_from_slice(&ln[6..]);
            buffer.push(b'\n');
            inp = new_inp;
        }
        Ok((inp, ()))
    }
}
