use crate::common::parser::{parse_right, FieldParser};
use nom::{
    bytes::complete::{tag, take},
    character::complete::{anychar, line_ending},
    IResult,
};
use serde::{Deserialize, Serialize};

// type Resolution = Option<f32>;

// pub struct Remark2Parser {};
// impl FieldParser for Remark2Parser {
//     fn parse(i: &str) -> IResult<&str, Cryst1> {
// }

// 6 + 4

// #[cfg(test)]
// mod tests {
//     const REMARK2_1: &'static str "
// REMARK   2 RESOLUTION.    1.70 ANGSTROMS.
// REMARK   3                                                                      ";

// }