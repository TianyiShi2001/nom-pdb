//! OBSLTE appears in entries that have been removed from public distribution.
//!
//! This record acts as a flag in an entry that has been removed (“obsoleted”)
//! from the PDB's full release. It indicates which, if any, new entries have
//! replaced the entry that was obsoleted. The format allows for the case of
//! multiple new entries replacing one existing entry.
//!
//! # Record Format
//!
//! |COLUMNS   |   DATA  TYPE   |  FIELD      |  DEFINITION                              |
//! |----------|----------------|-------------|------------------------------------------|
//! | 1 -  6   |   Record name  | "OBSLTE"    |                                          |
//! | 9 - 10   |   Continuation | continuation| Allows concatenation of multiple records |
//! |12 - 20   |   Date         | repDate     | Date that this entry was replaced.       |
//! |22 - 25   |   IDcode       | idCode      | ID code of this entry.                   |
//! |32 - 35   |   IDcode       | rIdCode     | ID code of entry that replaced this one. |
//! |37 - 40   |   IDcode       | rIdCode     | ID code of entry that replaced this one. |
//! |42 - 45   |   IDcode       | rIdCode     | ID code of entry  that replaced this one.|
//! |47 - 50   |   IDcode       | rIdCode     | ID code of entry that replaced this one. |
//! |52 - 55   |   IDcode       | rIdCode     | ID code of entry that replaced this one. |
//! |57 - 60   |   IDcode       | rIdCode     | ID code of entry that replaced this one. |
//! |62 - 65   |   IDcode       | rIdCode     | ID code of entry that replaced this one. |
//! |67 - 70   |   IDcode       | rIdCode     | ID code of entry that replaced this one. |
//! |72 - 75   |   IDcode       | rIdCode     | ID code of entry that replaced this one. |

use crate::common::parser::parse_date;
use chrono::NaiveDate;
use nom::{
    bytes::complete::take,
    character::complete::multispace1,
    combinator::{map, map_res},
    IResult,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Header<'a> {
    pub date_replaced: NaiveDate,
    pub id_code: &'a [u8],
    pub id_code_replaced: Vec<&'a [u8]>,
}
