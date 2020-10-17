//! [Cryst1](www.wwpdb.org/documentation/file-format-content/format33/sect8.html#CRYST1)
//! The CRYST1 record presents the unit cell parameters, space group, and Z value. If the
//! structure was not determined by crystallographic means, CRYST1 simply provides the unitary
//! values, with an appropriate REMARK.
//!
//! # Record Format
//!
//! COLUMNS DATA TYPE FIELD DEFINITION
//! ------------------------------------------------------------
//! 1  - 6  Record    name   "CRYST1"
//! 7  - 15 Real(9.3) a      a (Angstroms).
//! 16 - 24 Real(9.3) b      b (Angstroms).
//! 25 - 33 Real(9.3) c      c (Angstroms).
//! 34 - 40 Real(7.2) alpha  alpha (degrees).
//! 41 - 47 Real(7.2) beta   beta (degrees).
//! 48 - 54 Real(7.2) gamma  gamma (degrees).
//! 56 - 66 LString   sGroup Space group.
//! 67 - 70 Integer   z      Z value.
//!
//! # Additional References
//!
//! - https://infogalactic.com/info/Hermann%E2%80%93Mauguin_notation
//! - https://enacademic.com/dic.nsf/enwiki/1879109

use crate::common::parser::parse_right;
use crate::common::parser::FieldParser;

use nom::{
    bytes::complete::{tag, take},
    character::complete::{anychar, line_ending},
    IResult,
};
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Cryst1 {
    pub a: f32,
    pub b: f32,
    pub c: f32,
    pub alpha: f32,
    pub beta: f32,
    pub gamma: f32,
    pub lattice_type: LatticeType,
    pub space_group: SpaceGroup,
    pub z: u8,
}

// Space groups can be defined by combining the point group identifier with the uppercase P, C, I,
// or F for primitive, side-centered, body-centered, or face-centered lattices.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LatticeType {
    Primitive,
    SideCentered,
    BodyCentered,
    FaceCentered,
    Unknown,
}

impl Default for LatticeType {
    fn default() -> Self {
        LatticeType::Unknown
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct GroupAxis(pub u32, pub u32);

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct SpaceGroup(pub GroupAxis, pub Option<GroupAxis>, pub Option<GroupAxis>);

pub struct Cryst1Parser;
impl FieldParser for Cryst1Parser {
    type Output = Cryst1;
    fn parse(i: &str) -> IResult<&str, Cryst1> {
        let (i, a) = parse_right::<f32>(i, 9)?; // 7 - 15
        let (i, b) = parse_right::<f32>(i, 9)?; // 16 - 24
        let (i, c) = parse_right::<f32>(i, 9)?; // 25 - 33
        let (i, alpha) = parse_right::<f32>(i, 7)?; // 34 - 40
        let (i, beta) = parse_right::<f32>(i, 7)?; // 41 - 47
        let (i, gamma) = parse_right::<f32>(i, 7)?; // 48 - 54
        let (i, lattice_type) = parse_lattice_type(i)?; // 55 - 57
        let (i, space_group) = parse_space_group(i)?; // 58 - 66
        let (i, z) = parse_right::<u8>(i, 4)?; // 67 - 70
        let (i, _) = take(10usize)(i)?; // 71 - 80
        let (i, _) = line_ending(i)?;
        Ok((
            i,
            Cryst1 {
                a,
                b,
                c,
                alpha,
                beta,
                gamma,
                lattice_type,
                space_group,
                z,
            },
        ))
    }
}

fn parse_lattice_type(i: &str) -> IResult<&str, LatticeType> {
    let (i, _) = take(1usize)(i)?; // 55
    let (i, c) = anychar(i)?;
    let lattice_type = match c {
        'P' => LatticeType::Primitive,
        'C' => LatticeType::SideCentered,
        'I' => LatticeType::BodyCentered,
        'F' => LatticeType::FaceCentered,
        _ => LatticeType::Unknown,
    };
    let (i, _) = take(1usize)(i)?; // 57
    Ok((i, lattice_type))
}

fn parse_space_group(i: &str) -> IResult<&str, SpaceGroup> {
    let (i, a) = parse_group_axis(i)?; // 58 - 60
    let (i, b) = parse_group_axis(i)?; // 61 - 63
    let (i, c) = parse_group_axis(i)?; // 64 - 66
    Ok((i, SpaceGroup(a.unwrap(), b, c)))
}

fn parse_group_axis(i: &str) -> IResult<&str, Option<GroupAxis>> {
    let (i, a) = anychar(i)?;
    let (i, b) = anychar(i)?;
    let (i, _) = anychar(i)?;
    let r: Option<GroupAxis> = match (a, b) {
        (' ', _) => None,
        (_, ' ') => Some(GroupAxis(a.to_digit(10).unwrap(), 1u32)),
        _ => Some(GroupAxis(a.to_digit(10).unwrap(), b.to_digit(10).unwrap())),
    };
    Ok((i, r))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_parse_cryst1() {
        let i = "   41.980   41.980   88.920  90.00  90.00  90.00 P 43 21 2     8          
ORIGX1      1.000000  0.000000  0.000000        0.00000                         ";
        let (i, r) = Cryst1Parser::parse(i).unwrap();
        assert_eq!(
            i.to_owned(),
            "ORIGX1      1.000000  0.000000  0.000000        0.00000                         "
                .to_owned()
        );
    }
}
