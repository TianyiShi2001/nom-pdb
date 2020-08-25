use crate::common::parser::{parse_multiline_string, FieldParserComplete};
pub type Title = String;
pub struct TitleParserComplete;
impl FieldParserComplete for TitleParserComplete {
    type Output = Title;
    fn parse(inp: &str) -> nom::IResult<&str, Self::Output> {
        parse_multiline_string(inp, "TITLE ")
    }
}
