//! # Overview
//!
//! The CONECT records specify connectivity between atoms for which coordinates are supplied. The connectivity is described using the atom serial number as shown in the entry. CONECT records are mandatory for HET groups (excluding water) and for other Connect not specified in the standard residue connectivity table. These records are generated automatically.
//!
//! # Record Format
//!
//! COLUMNS    |  DATA  TYPE    |  FIELD   |     DEFINITION
//! -----------|----------------|----------|-----------------------------------
//!  1 -  6    |   Record name  |  "CONECT"|
//!  7 - 11    |  Integer       | serial   |    Atom  serial number
//! 12 - 16    |   Integer      |  serial  |     Serial number of bonded atom
//! 17 - 21    |   Integer      |  serial  |     Serial  number of bonded atom
//! 22 - 26    |   Integer      |  serial  |     Serial number of bonded atom
//! 27 - 31    |   Integer      |  serial  |     Serial number of bonded atom
//!
//! Details
//!
//! CONECT records are present for:
//!
//! - Intra-residue connectivity within  non-standard (HET) residues (excluding water).
//! - Inter-residue connectivity of HET  groups to standard groups (including water) or to other HET groups.
//! - Disulfide bridges specified in the  SSBOND records have corresponding records.
//!
//! - No differentiation is made between atoms with delocalized charges (excess negative or positive charge).
//! - Atoms specified in the CONECT records have the same numbers as given in the coordinate section.
//! - All atoms connected to the atom with serial number in columns 7 - 11 are listed in the remaining fields of the record.
//! - If more than four fields are required for non-hydrogen and non-salt bridges, a second CONECT record with the same atom serial number in columns 7 - 11 will be used.
//! - These CONECT records occur in increasing order of the atom serial numbers they carry in columns 7 - 11. The target-atom serial numbers carried on these records also occur in increasing order.
//! - The connectivity list given here is redundant in that each bond indicated is given twice, once with each of the two atoms involved specified in columns 7 - 11.
//! - For hydrogen Connect, when the hydrogen atom is present in the coordinates, a CONECT record between the hydrogen atom and its acceptor atom is generated.
//! - For NMR entries, CONECT records for one model are generated describing heterogen connectivity and others for LINK records assuming that all models are homogeneous models.

use crate::common::parser::FieldParser;
use crate::common::parser::{jump_newline, parse_right};
use crate::types::{AtomSerial, Connect};
use nom::IResult;

pub struct ConectParser;

impl FieldParser for ConectParser {
    type Output = Vec<Connect>;
    fn parse(inp: &[u8]) -> IResult<&[u8], Self::Output> {
        let mut res = Vec::new();
        let (inp, x) = parse_right::<AtomSerial>(inp, 5)?;
        let mut last_inp = inp;
        loop {
            let (inp, y) = parse_right::<AtomSerial>(last_inp, 5)?;
            if y > x {
                res.push([x, y]);
            } else {
                res.push([y, x]);
            }
            if inp[..5] == b"     "[..] {
                break;
            }
            last_inp = inp
        }
        let (inp, _) = jump_newline(last_inp)?;
        Ok((inp, res))
    }
}
