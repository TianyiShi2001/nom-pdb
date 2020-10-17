// //! The [ANISOU](http://www.wwpdb.org/documentation/file-format-content/format33/sect9.html#ANISOU) records present the anisotropic temperature factors.
// //!
// //! # Record Format
// //!
// //! | COLUMNS | DATA  TYPE   | FIELD    | DEFINITION                       |
// //! | ------- | ------------ | -------- | -------------------------------- |
// //! | 1 - 6   | Record name  | "ANISOU" |                                  |
// //! | 7 - 11  | Integer      | serial   | Atom serial number.              |
// //! | 13 - 16 | Atom         | name     | Atom name.                       |
// //! | 17      | Character    | altLoc   | Alternate location indicator     |
// //! | 18 - 20 | Residue name | resName  | Residue name.                    |
// //! | 22      | Character    | chainID  | Chain identifier.                |
// //! | 23 - 26 | Integer      | resSeq   | Residue sequence number.         |
// //! | 27      | AChar        | iCode    | Insertion code.                  |
// //! | 29 - 35 | Integer      | u[0][0]  | U(1,1)                           |
// //! | 36 - 42 | Integer      | u[1][1]  | U(2,2)                           |
// //! | 43 - 49 | Integer      | u[2][2]  | U(3,3)                           |
// //! | 50 - 56 | Integer      | u[0][1]  | U(1,2)                           |
// //! | 57 - 63 | Integer      | u[0][2]  | U(1,3)                           |
// //! | 64 - 70 | Integer      | u[1][2]  | U(2,3)                           |
// //! | 77 - 78 | LString(2)   | element  | Element symbol, right-justified. |
// //! | 79 - 80 | LString(2)   | charge   | Charge on the atom.              |

// // 13-27 redundant

// use crate::common::parser::FieldParser;
// use crate::common::parser::{pa, parse_amino_acid, parse_right, parse_right::<f32>, parse_right::<u32>};
// use crate::common::types::AminoAcid;
// use nom::bytes::complete::take;
// use nom::character::complete::anychar;
// use nom::combinator::map;
// use nom::IResult;
// use std::str::FromStr;
// #[derive(Debug, Clone)]
// pub struct Anisou {
//     pub id: u32,
//     pub u11: i32,
// }
// pub struct AnisouParser;

// impl FieldParser for AnisouParser {
//     type Output = Anisou;
//     fn parse(inp: &str) -> IResult<&str, Anisou> {
//         let (inp, id) = parse_right::<u32>(inp, 5)?;
//         let (inp, _) = take(1usize)(inp)?;

//         let (inp, id1) = anychar(inp)?;
//         let (inp, residue) = parse_amino_acid(inp)?;
//         let (inp, _) = take(1usize)(inp)?;
//         let (inp, chain) = anychar(inp)?;
//         let (inp, sequence_number) = parse_right::<u32>(inp, 4)?;
//         let (inp, insertion_code) = anychar(inp)?;
//         let (inp, _) = take(3usize)(inp)?;
//         let (inp, x) = parse_right::<f32>(inp, 8)?;
//         let (inp, y) = parse_right::<f32>(inp, 8)?;
//         let (inp, z) = parse_right::<f32>(inp, 8)?;
//         let (inp, occupancy) = parse_right::<f32>(inp, 6)?;
//         let (inp, temperature_factor) = parse_right::<f32>(inp, 6)?;
//         let (inp, _) = take(10usize)(inp)?;
//         let (inp, element) = map(map(take(2usize), str::trim_start), |x| {
//             Element::from_str(x).unwrap()
//         })(inp)?;
//         let (inp, charge) = map(take(2usize), |x| match x {
//             "  " => 0,
//             _ => x.parse::<i8>().unwrap(),
//         })(inp)?;
//         let (inp, _) = nom::character::complete::line_ending(inp)?;
//         Ok((
//             inp,
//             Atom {
//                 id,
//                 id1,
//                 residue,
//                 chain,
//                 sequence_number,
//                 insertion_code,
//                 x,
//                 y,
//                 z,
//                 occupancy,
//                 temperature_factor,
//                 element,
//                 charge,
//             },
//         ))
//     }
// }
