use crate::common::parser::{parse_multiline_string, FieldParser};
use crate::types::*;
pub struct TitleParser;
impl FieldParser for TitleParser {
    type Output = Title;
    fn parse(inp: &[u8]) -> nom::IResult<&[u8], Self::Output> {
        parse_multiline_string(inp, b"TITLE ")
    }
}
