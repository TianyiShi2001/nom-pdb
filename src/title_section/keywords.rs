//! Parses KEYWDS record which is a multiline continuation record. Contains comma-seperated list of
//! keywords relevant to pdb entry.If successfull returns [Record](../ast/types/enum.Record.html)
//! variant containing [KEYWDS](../ast/types/struct.Keywds.html) instance.
//!
//! # Record structure
//!
//! | COLUMNS | DATA  TYPE   | FIELD        | DEFINITION                                   |
//! |---------|--------------|--------------|----------------------------------------------|
//! | 1 -  6  | Record name  | KEYWDS       |                                              |
//! | 9 - 10  | Continuation | continuation | Allows concatenation of records if necessary.|
//! | 11 - 79 | List         | keywds       | Comma-separated list of keywords relevant    |
//! |         |              |              | to the entry.                                |
use crate::common::parser::parse_multiline_list;

type Keywords = Vec<String>;
pub fn parse_kaywords(inp: &str) -> nom::IResult<&str, Keywords> {
    parse_multiline_list(inp)
}
