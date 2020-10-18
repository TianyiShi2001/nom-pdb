//! Parses EXPDTA records which is a continuation type of record which may span multi-lines.
//! Record contains list of `;` seperated experimental techniques. If seuccesfull returns
//! [Record](../ast/types/enum.Record.html) variant containing
//! [ExperimentalTechniques](../ast/types/struct.Experimental.html)
//!
//! # Record structure
//!
//! | COLUMNS | DATA TYPE     | FIELD        | DEFINITION                                |
//! |---------|---------------|--------------|-------------------------------------------|
//! | 1 -  6  | Record name   | EXPDTA       |                                           |
//! | 9 - 10  | Continuation  | continuation | Allows concatenation of multiple records. |
//! | 11 - 79 | SList         | technique    | The experimental technique(s) with        |
//! |         |                              | optional comment desc                     |
use crate::common::parser::{parse_multiline_list, FieldParser};
use crate::types::*;

pub struct ExperimentalTechniquesParser;
impl FieldParser for ExperimentalTechniquesParser {
    type Output = Vec<ExperimentalTechnique>;
    fn parse(inp: &[u8]) -> nom::IResult<&[u8], Vec<ExperimentalTechnique>> {
        let (inp, techniques_as_str) = parse_multiline_list(inp)?;
        let techniques: Vec<ExperimentalTechnique> = techniques_as_str
            .into_iter()
            .map(|s| {
                s.parse::<ExperimentalTechnique>()
                    .expect("Failed to parse experimental techniques")
            })
            .collect();
        Ok((inp, techniques))
    }
}
