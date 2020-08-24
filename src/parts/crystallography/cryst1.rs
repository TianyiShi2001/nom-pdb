use crate::common::{parse_right_f32, parse_right_u8};
/// Parsing the [Cryst1](www.wwpdb.org/documentation/file-format-content/format33/sect8.html#CRYST1)
/// The CRYST1 record presents the unit cell parameters, space group, and Z value. If the structure was not determined by crystallographic means, CRYST1 simply provides the unitary values, with an appropriate REMARK.
///
/// Record Format
///
/// COLUMNS DATA TYPE FIELD DEFINITION
/// ------------------------------------------------------------
/// 1  - 6  Record    name   "CRYST1"
/// 7  - 15 Real(9.3) a      a (Angstroms).
/// 16 - 24 Real(9.3) b      b (Angstroms).
/// 25 - 33 Real(9.3) c      c (Angstroms).
/// 34 - 40 Real(7.2) alpha  alpha (degrees).
/// 41 - 47 Real(7.2) beta   beta (degrees).
/// 48 - 54 Real(7.2) gamma  gamma (degrees).
/// 56 - 66 LString   sGroup Space group.
/// 67 - 70 Integer   z      Z value.
use nom::{
    bytes::complete::{tag, take},
    character::complete::{anychar, line_ending},
    IResult,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GroupAxis(pub u32, pub u32);

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SpaceGroup(pub GroupAxis, pub Option<GroupAxis>, pub Option<GroupAxis>);

#[derive(Debug, Clone, PartialEq)]
pub struct Cryst1 {
    pub a: f32,
    pub b: f32,
    pub c: f32,
    pub alpha: f32,
    pub beta: f32,
    pub gamma: f32,
    pub space_group: SpaceGroup,
    pub z: u8,
}

fn parse_cryst1(i: &str) -> IResult<&str, Cryst1> {
    let (i, a) = parse_right_f32(i, 9)?; // 7 - 15
    let (i, b) = parse_right_f32(i, 9)?; // 16 - 24
    let (i, c) = parse_right_f32(i, 9)?; // 25 - 33
    let (i, alpha) = parse_right_f32(i, 7)?; // 34 - 40
    let (i, beta) = parse_right_f32(i, 7)?; // 41 - 47
    let (i, gamma) = parse_right_f32(i, 7)?; // 48 - 54
    let (i, space_group) = parse_space_group(i)?; // 56 - 66
    let (i, z) = parse_right_u8(i, 4)?; // 67 - 70
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
            space_group,
            z,
        },
    ))
}

fn parse_space_group(i: &str) -> IResult<&str, SpaceGroup> {
    let (i, _) = tag(" P ")(i)?; // 66 - 57
    let (i, a) = parse_group_axis(i)?; // 58 - 60
    let (i, b) = parse_group_axis(i)?; // 61 - 63
    let (i, c) = parse_group_axis(i)?; // 64 - 66
    Ok((i, SpaceGroup(a.unwrap(), b, c)))
}

fn parse_group_axis(i: &str) -> IResult<&str, Option<GroupAxis>> {
    let (i, a) = anychar(i)?;
    let (i, b) = anychar(i)?;
    let (i, _) = anychar(i)?;
    let r: Option<GroupAxis>;
    if a == ' ' {
        r = None;
    } else {
        if b == ' ' {
            r = Some(GroupAxis(a.to_digit(10).unwrap(), 1u32));
        } else {
            r = Some(GroupAxis(a.to_digit(10).unwrap(), b.to_digit(10).unwrap()));
        }
    }
    Ok((i, r))
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_parse_cryst1() {
        let i = "   41.980   41.980   88.920  90.00  90.00  90.00 P 43 21 2     8          
ORIGX1      1.000000  0.000000  0.000000        0.00000                         ";
        let (i, r) = parse_cryst1(i).unwrap();
        assert_eq!(
            i.to_owned(),
            "ORIGX1      1.000000  0.000000  0.000000        0.00000                         "
                .to_owned()
        );
    }
}
