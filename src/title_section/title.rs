use crate::common::parser::{parse_multiline_string, FieldParser};
use crate::types::*;
pub struct TitleParser;
impl FieldParser for TitleParser {
    type Output = Title;
    fn parse(inp: &str) -> nom::IResult<&str, Self::Output> {
        parse_multiline_string(inp, "TITLE ")
    }
}
