// Copyright (c) 2020 Tianyi Shi
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

/// A simple single-thread parser.
use crate::{
    coordinate::*, crystallography::*, primary_structure::*, secondary_structure::*,
    title_section::*,
};

// use crate::common::error::PdbParseError;
use crate::common::parser::FieldParser;
use nom::bytes::complete::take;
use nom::character::complete::{line_ending, not_line_ending};

use crate::types::{
    Connect, Helix, Model, ModifiedAminoAcidTable, ModifiedNucleotideTable, Sheet, Structure,
};

use protein_core::metadata::*;

pub struct Parser {}

impl Parser {
    pub fn parse(mut inp: &[u8]) -> nom::IResult<&[u8], Structure> {
        let mut metadata = Metadata::default();

        let mut seqres_buffer: Vec<u8> = Default::default();

        let mut helices: Vec<Helix> = Vec::new();
        let mut sheets: Vec<Sheet> = Vec::new();

        let mut connect: Vec<Connect> = Vec::new();

        let mut models: Vec<Model> = vec![Model::default()];

        let mut modified_aa: ModifiedAminoAcidTable = Default::default();
        let mut modified_nuc: ModifiedNucleotideTable = Default::default();

        let mut model_idx = 0;

        loop {
            let (i, tag) = take(6usize)(inp)?;
            inp = match tag {
                b"HEADER" => HeaderParser::parse_into_option(&i, &mut metadata.header),
                b"TITLE " => TitleParser::parse_into_option(&i, &mut metadata.title),
                b"AUTHOR" => AuthorsParser::parse_into_option(&i, &mut metadata.authors),
                b"CRYST1" => Cryst1Parser::parse_into_option(&i, &mut metadata.cryst1),
                b"SEQRES" => SeqResParser::buffer_seqres(&i, &mut seqres_buffer)?.0,
                b"MODRES" => ModresParser::parse_into(&i, &mut modified_aa, &mut modified_nuc)?.0,
                b"EXPDTA" => ExperimentalTechniquesParser::parse_into_option(
                    &i,
                    &mut metadata.experimental_techniques,
                ),
                b"ATOM  " | b"HETATM" => {
                    let (i, atom) = GenericAtomParser::parse(&i, &modified_aa, &modified_nuc)?;
                    models[model_idx].atoms.push(atom);
                    i
                }
                b"ANISOU" => AnisouParser::parse_into_vec(&i, &mut models[model_idx].anisou),
                b"CONECT" => {
                    let (i, cnct) = ConectParser::parse(&i)?;
                    for c in cnct {
                        if !connect.contains(&c) {
                            connect.push(c); // ! is this reliable?
                        }
                    }
                    i
                }
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
                b"SHEET " => SheetParser::parse_into_vec(&i, &mut sheets),
                b"HELIX " => HelixParser::parse_into_vec(&i, &mut helices),
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
        let (_, (chains_aa, chains_nuc)) =
            SeqResParser::parse(&seqres_buffer, &modified_aa, &modified_nuc).unwrap();
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
                metadata: Some(metadata),
            },
        ))
    }
}
