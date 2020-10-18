//! # Overview
//!
//! HELIX records are used to identify the position of helices in the molecule. Helices are named, numbered, and classified by type. The residues where the helix begins and ends are noted, as well as the total length.
//!
//! # Record Format
//!
//! |COLUMNS  |      DATA  TYPE    | FIELD        | DEFINITION
//! |---------|--------------------|--------------|----------------------------------------
//! | 1 -  6  |      Record name   | "HELIX "     |                                         
//! | 8 - 10  |      Integer       | serNum       | Serial number of the helix. This starts
//! |         |                    |              | at 1  and increases incrementally.
//! |12 - 14  |      LString(3)    | helixID      | Helix  identifier. In addition to a serial
//! |         |                    |              | number, each helix is given an
//! |         |                    |              | alphanumeric character helix identifier.
//! |16 - 18  |      Residue name  | initResName  | Name of the initial residue.
//! |20       |      Character     | initChainID  | Chain identifier for the chain containing
//! |         |                    |              | this  helix.
//! |22 - 25  |      Integer       | initSeqNum   | Sequence number of the initial residue.
//! |26       |      AChar         | initICode    | Insertion code of the initial residue.
//! |28 - 30  |      Residue  name | endResName   | Name of the terminal residue of the helix.
//! |32       |      Character     | endChainID   | Chain identifier for the chain containing
//! |         |                    |              | this  helix.
//! |34 - 37  |      Integer       | endSeqNum    | Sequence number of the terminal residue.
//! |38       |      AChar         | endICode     | Insertion code of the terminal residue.
//! |39 - 40  |      Integer       | helixClass   | Helix class (see below).
//! |41 - 70  |      String        | comment      | Comment about this helix.
//! |72 - 76  |      Integer       | length       | Length of this helix.
//!
//! # Details
//!
//! Additional HELIX records with different serial numbers and identifiers occur if more than one helix is present.
//! The initial residue of the helix is the N-terminal residue.
//! Helices are classified as follows:
//!
//! |                                |     CLASS NUMBER             |           
//! |TYPE OF  HELIX                  |   (COLUMNS 39 - 40)          |              
//! |--------------------------------|------------------------------|
//! |Right-handed alpha (default)    |            1                 |       
//! |Right-handed omega              |            2                 |       
//! |Right-handed pi                 |            3                 |       
//! |Right-handed gamma              |            4                 |       
//! |Right-handed 3 - 10             |            5                 |       
//! |Left-handed alpha               |            6                 |       
//! |Left-handed omega               |            7                 |       
//! |Left-handed gamma               |            8                 |       
//! |2 - 7 ribbon/helix              |            9                 |       
//! |Polyproline                     |           10                 |       

use crate::common::parser::FieldParser;
use crate::common::parser::{jump_newline, parse_right, take_trim_own};
use crate::types::{Helix, HelixClass, ResidueSerial};
use nom::{bytes::complete::take, character::complete::anychar, combinator::map, IResult};

pub struct HelixParser;

impl FieldParser for HelixParser {
    type Output = Helix;
    fn parse(inp: &[u8]) -> IResult<&[u8], Self::Output> {
        let (inp, _) = take(5usize)(inp)?; // 7; 8 - 10; 11
        let (inp, id) = take(3usize)(inp)?; // 12 - 14
        let (inp, _) = take(5usize)(inp)?; // 15; 16 - 18; 19
        let (inp, start_chain) = anychar(inp)?; // 20
        let (inp, _) = take(1usize)(inp)?; // 21
        let (inp, start_serial) = parse_right::<ResidueSerial>(inp, 4)?; // 22 - 25
        let (inp, _start_icode) = anychar(inp)?; // 26
        let (inp, _) = take(5usize)(inp)?; // 27; 28 - 30; 31
        let (inp, end_chain) = anychar(inp)?; // 32
        let (inp, _) = take(1usize)(inp)?; // 33
        let (inp, end_serial) = parse_right::<ResidueSerial>(inp, 4)?; // 34 - 37
        let (inp, _end_icode) = anychar(inp)?; // 38
        let (inp, class) = Self::parse_helix_class(inp)?; // 39 - 40
        let (inp, comment) = take(30usize)(inp)?; // 41 - 70
        let (inp, _) = jump_newline(inp)?;
        let helix = Helix {
            id: unsafe { std::str::from_utf8_unchecked(id).trim().to_owned() },
            class,
            start: (start_chain, start_serial),
            end: (end_chain, end_serial),
            comment: unsafe { std::str::from_utf8_unchecked(comment).trim().to_owned() },
        };
        Ok((inp, helix))
    }
}

impl HelixParser {
    pub fn parse_helix_class(inp: &[u8]) -> IResult<&[u8], HelixClass> {
        use HelixClass::*;
        let (inp, code) = parse_right::<usize>(inp, 2)?;

        let class = if code < 11 {
            [
                RightHandedAlpha,
                RightHandedOmega,
                RightHandedPi,
                RightHandedGamma,
                RightHanded310,
                LeftHandedAlpha,
                LeftHandedOmega,
                LeftHandedGamma,
                TwoSevenRibbonHelix,
                Polyproline,
            ][code]
        } else {
            Unknown
        };
        Ok((inp, class))
    }
}
