use crate::common::parser::parse_multiline_list;
/// Parses AUTHOR record which is a multiline continuation record.
/// Contains comma-seperated list of author names. If successfull
/// returns [Record](../ast/types/enum.Record.html) variant containing
/// [AUTHORS](../ast/types/struct.Authors.html) instance.
/// # Record structure
/// | COLUMNS | DATA  TYPE   | FIELD        | DEFINITION                                   |
/// |---------|--------------|--------------|----------------------------------------------|
/// | 1 -  6  | Record name  | AUTHOR       |                                              |
/// | 9 - 10  | Continuation | continuation | Allows concatenation of multiple records.    |
/// | 11 - 79 | List         | authorList   | List of the author names, separated          |
/// |         |              |              | by commas.                                   |
use crate::common::parser::FieldParser;
use crate::types::*;
pub struct AuthorsParser;
impl FieldParser for AuthorsParser {
    type Output = Authors;
    fn parse(inp: &[u8]) -> nom::IResult<&[u8], Authors> {
        let (inp, names) = parse_multiline_list(inp)?;
        Ok((inp, names))
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//     #[test]
//     fn test_parse_authors() {
//         let (i, r) = AuthorsParser::parse(
//             "    T.R.GAMBLE,S.YOO,F.F.VAJDOS,U.K.VON SCHWEDLER,
// AUTHOR   2 D.K.WORTHYLAKE,H.WANG,J.P.MCCUTCHEON,W.I.SUNDQUIST,C.P.HILL
// REVDAT   5   03-NOV-09 1A8O    1       SEQADV                                   ",
//         )
//         .unwrap();
//         assert_eq!(
//             i,
//             "REVDAT   5   03-NOV-09 1A8O    1       SEQADV                                   "
//         );
//         assert_eq!(
//             r,
//             vec![
//                 "T.R.GAMBLE".to_owned(),
//                 "S.YOO".to_owned(),
//                 "F.F.VAJDOS".to_owned(),
//                 "U.K.VON SCHWEDLER".to_owned(),
//                 "D.K.WORTHYLAKE".to_owned(),
//                 "H.WANG".to_owned(),
//                 "J.P.MCCUTCHEON".to_owned(),
//                 "W.I.SUNDQUIST".to_owned(),
//                 "C.P.HILL".to_owned()
//             ]
//         )
//     }
// }
