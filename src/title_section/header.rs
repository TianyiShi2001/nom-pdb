//! Parsing the [Header](http://www.wwpdb.org/documentation/file-format-content/format33/sect2.html#HEADER).
//! The HEADER record uniquely identifies a PDB entry through the idCode field.
//! This record also provides a classification for the entry. Finally, it contains
//! the date when the coordinates were deposited to the PDB archive.
//!
//! # Record Format
//!
//! | COLUMNS | DATA  TYPE   | FIELD          | DEFINITION                                |
//! |---------|--------------|----------------|-------------------------------------------|
//! | 1 -  6  | Record name  | HEADER         |                                           |
//! | 11 - 50 | String(40)/`String`   | `classification` | Classifies the molecule(s).               |
//! | 51 - 59 | Date/`chrono::NaiveDate`         | `deposition_date`        | Deposition date. This is the date the coordinates  were received at the PDB.   |
//! | 63 - 66 | IDcode/`String`      | `id_code`         | This identifier is unique within the PDB. |
use crate::common::parser::{parse_date, take_trim_own, FieldParser};
use crate::types::*;
use nom::{bytes::complete::take, character::complete::multispace1, IResult};

pub struct HeaderParser;

impl FieldParser for HeaderParser {
    type Output = Header;
    fn parse(inp: &[u8]) -> IResult<&[u8], Self::Output> {
        let (inp, _) = take(4usize)(inp)?;
        let (inp, classification) = unsafe { take_trim_own(inp, 40usize)? };
        let (inp, deposition_date) = parse_date(inp)?;
        let (inp, _) = take(3usize)(inp)?;
        let (inp, id_code) = take(4usize)(inp)?;
        let (inp, _) = multispace1(inp)?;
        Ok((
            inp,
            Header {
                classification,
                deposition_date,
                id_code: unsafe { std::str::from_utf8_unchecked(id_code).to_owned() },
            },
        ))
    }
}

pub struct HeaderParserStreaming;
pub struct HeaderParserParallel;

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use chrono::NaiveDate;
//     #[test]
//     fn test_parse_header() {
//         let i = "    VIRAL PROTEIN                           27-MAR-98   1A8O              \nTITLE     HIV CAPSID C-TERMINAL DOMAIN                                          ";
//         let (i, r) = HeaderParser::parse(i).unwrap();
//         assert_eq!(
//             i.to_owned(),
//             "TITLE     HIV CAPSID C-TERMINAL DOMAIN                                          "
//                 .to_owned()
//         );
//         assert_eq!(
//             r,
//             Header {
//                 classification: "VIRAL PROTEIN".to_owned(),
//                 deposition_date: NaiveDate::from_ymd(1998i32, 3u32, 27u32),
//                 id_code: "1A8O".to_owned()
//             }
//         )
//     }
// }
