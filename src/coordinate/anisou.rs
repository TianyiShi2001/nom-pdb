//! The [ANISOU](http://www.wwpdb.org/documentation/file-format-content/format33/sect9.html#ANISOU) records present the anisotropic temperature factors.
//!
//! # Record Format
//!
//! | COLUMNS | DATA  TYPE   | FIELD    | DEFINITION                       |
//! | ------- | ------------ | -------- | -------------------------------- |
//! | 1 - 6   | Record name  | "ANISOU" |                                  |
//! | 7 - 11  | Integer      | serial   | Atom serial number.              |
//! | 13 - 16 | Atom         | name     | Atom name.                       |
//! | 17      | Character    | altLoc   | Alternate location indicator     |
//! | 18 - 20 | Residue name | resName  | Residue name.                    |
//! | 22      | Character    | chainID  | Chain identifier.                |
//! | 23 - 26 | Integer      | resSeq   | Residue sequence number.         |
//! | 27      | AChar        | iCode    | Insertion code.                  |
//! | 29 - 35 | Integer      | u[0][0]  | U(1,1)                           |
//! | 36 - 42 | Integer      | u[1][1]  | U(2,2)                           |
//! | 43 - 49 | Integer      | u[2][2]  | U(3,3)                           |
//! | 50 - 56 | Integer      | u[0][1]  | U(1,2)                           |
//! | 57 - 63 | Integer      | u[0][2]  | U(1,3)                           |
//! | 64 - 70 | Integer      | u[1][2]  | U(2,3)                           |
//! | 77 - 78 | LString(2)   | element  | Element symbol, right-justified. |
//! | 79 - 80 | LString(2)   | charge   | Charge on the atom.              |

// 13-27 redundant

use crate::common::parser::parse_right;
use crate::common::parser::FieldParser;
use crate::common::types::AminoAcid;
use nom::bytes::complete::take;
use nom::character::complete::anychar;
use nom::combinator::map;
use nom::IResult;
use std::str::FromStr;
#[derive(Debug, Clone)]
pub struct Anisou {
    pub id: u32,
    pub u11: i32,
    pub u22: i32,
    pub u33: i32,
    pub u12: i32,
    pub u13: i32,
    pub u23: i32,
}
pub struct AnisouParser;

impl FieldParser for AnisouParser {
    type Output = Anisou;
    fn parse(inp: &str) -> IResult<&str, Anisou> {
        let (inp, id) = parse_right::<u32>(inp, 5)?;
        let (inp, _) = take(17usize)(inp)?; // 12 - 28

        let (inp, u11) = parse_right::<i32>(inp, 7)?;
        let (inp, u22) = parse_right::<i32>(inp, 7)?;
        let (inp, u33) = parse_right::<i32>(inp, 7)?;
        let (inp, u12) = parse_right::<i32>(inp, 7)?;
        let (inp, u13) = parse_right::<i32>(inp, 7)?;
        let (inp, u23) = parse_right::<i32>(inp, 7)?;
        let (inp, _) = take(10usize)(inp)?;
        let (inp, _) = nom::character::complete::line_ending(inp)?;
        Ok((
            inp,
            Anisou {
                id,
                u11,
                u22,
                u33,
                u12,
                u13,
                u23,
            },
        ))
    }
}
