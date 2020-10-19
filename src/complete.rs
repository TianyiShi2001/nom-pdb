/// A simple single-thread parser.
use crate::{
    coordinate::*, crystallography::*, primary_structure::*, secondary_structure::*,
    title_section::*,
};

// use crate::common::error::PdbParseError;
use crate::common::parser::FieldParser;
use nom::bytes::complete::take;
use nom::character::complete::{line_ending, not_line_ending};
use nom::IResult;
use protein_core::{Model, ModifiedAminoAcid, ModifiedNucleotide};
// use nom::Err::Error;
use std::collections::HashMap;

use protein_core::{AminoAcid, Chain, Connect, Helix, HelixClass, Nucleotide, Sheet, Structure};

enum ParserState {
    FirstLine,
    Continue,
}

pub struct Parser {
    // state: ParserState,
    // remaining: &'a [u8],
    seqres_buffer: String,
}

impl Parser {
    pub fn parse(mut inp: &[u8]) -> nom::IResult<&[u8], Structure> {
        let mut chains_aa: Vec<Chain<AminoAcid>> = Default::default();
        let mut chains_nuc: Vec<Chain<Nucleotide>> = Default::default();
        let mut helices: Vec<Helix> = Vec::new();
        let mut sheets: Vec<Sheet> = Vec::new();

        let mut connect: Vec<Connect> = Vec::new();

        let mut models: Vec<Model> = vec![Model::default()];

        let mut modified_aa: HashMap<String, ModifiedAminoAcid> = Default::default();
        //let modified_aa_ptr = &mut modified_aa as *mut Vec<ModifiedAminoAcid>;
        let mut modified_nuc: HashMap<String, ModifiedNucleotide> = Default::default();
        //let modified_nuc_ptr = &mut modified_nuc as *mut HashMap<String, ModifiedNucleotide>;

        let mut model_idx = 0;

        loop {
            let (i, tag) = take(6usize)(inp)?;
            inp = match tag {
                // b"HEADER" => HeaderParser::parse_into(&i, &mut pdb.header),
                // b"TITLE " => TitleParser::parse_into(&i, &mut pdb.title),
                // b"AUTHOR" => AuthorsParser::parse_into(&i, &mut pdb.authors),
                // b"CRYST1" => Cryst1Parser::parse_into(&i, &mut pdb.cryst1),
                // b"SEQRES" => {
                //     //while &i[..6] == b"SEQRES"
                //     SeqResParser::parse_into(&i, &mut pdb.seqres)
                // }
                b"MODRES" => ModresParser::parse_into(&i, &mut modified_aa, &mut modified_nuc)?.0,
                // b"EXPDTA" => {
                //     ExperimentalTechniquesParser::parse_into(&i, &mut pdb.experimental_techniques)
                // }
                b"ATOM  " | b"HETATM" => {
                    let (i, atom) = GenericAtomParser::parse(&i, &modified_aa, &modified_nuc)?;
                    models[model_idx].atoms.push(atom);
                    i
                }
                b"ANISOU" => AnisouParser::parse_into_vec(&i, &mut models[model_idx].anisou),
                // b"CONECT" => {
                //     let (i, connect) = ConectParser::parse(&i)?;
                //     for bond in connect {
                //         if !pdb.models[0].connect.contains(&bond) {
                //             for model in &mut pdb.models {
                //                 // ! for multiple models, CONECT seems not to repeat
                //                 model.connect.push(bond); // ! is this reliable?
                //             }
                //         }
                //     }
                //     i
                // }
                b"MODEL " => {
                    if models.len() != 1 {
                        // * if there's one model, there would be no "MODEL"
                        models.push(Model::default());
                        model_idx += 1;
                    }
                    let (i, _) = not_line_ending(i)?;
                    let (i, _) = line_ending(i)?;
                    i
                }
                // b"SHEET " => {
                //     let (i, sheet) = SheetParser::parse(&i)?;
                //     for model in &mut pdb.models {
                //         // ! for multiple models, SHEET seems not to repeat
                //         model.sheets.push(sheet.clone()); // ! is this reliable? what about mmCIF?
                //     }
                //     i
                // }
                // b"HELIX " => {
                //     let (i, helix) = HelixParser::parse(&i)?;
                //     for model in &mut pdb.models {
                //         model.helices.push(helix.clone());
                //     }
                //     i
                // }
                b"END   " => {
                    inp = b"";
                    break;
                }
                _ => {
                    // new line
                    let (i, _) = not_line_ending(i)?;
                    let (i, _) = line_ending(i)?;
                    i
                } //panic!("Unkown field"),
            }
        }
        Ok((
            inp,
            Structure {
                chains_aa,
                chains_nuc,
                helices,
                sheets,
                modified_aa,
                modified_nuc,
                connect,
                models,
            },
        ))
    }
}
