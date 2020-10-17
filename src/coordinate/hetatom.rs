//! # Overview
//!
//! http://www.wwpdb.org/documentation/file-format-content/format33/sect9.html#HETATM
//!
//! Non-polymer or other “non-standard” chemical coordinates, such as water molecules or atoms presented in HET groups use the HETATM record type. They also present the occupancy and temperature factor for each atom. The ATOM records present the atomic coordinates for standard residues. The element symbol is always present on each HETATM record; charge is optional.
//!
//! Changes in ATOM/HETATM records will require standardization in atom and residue nomenclature. This nomenclature is described in the Chemical Component Dictionary, ftp://ftp.wwpdb.org/pub/pdb/data/monomers.
//!
//! # Record Format
//!
//! | COLUMNS | DATA  TYPE   | FIELD      | DEFINITION                       |
//! | ------- | ------------ | ---------- | -------------------------------- |
//! | 1 - 6   | Record name  | "HETATM"   |                                  |
//! | 7 - 11  | Integer      | serial     | Atom serial number.              |
//! | 13 - 16 | Atom         | name       | Atom name.                       |
//! | 17      | Character    | altLoc     | Alternate location indicator.    |
//! | 18 - 20 | Residue name | resName    | Residue name.                    |
//! | 22      | Character    | chainID    | Chain identifier.                |
//! | 23 - 26 | Integer      | resSeq     | Residue sequence number.         |
//! | 27      | AChar        | iCode      | Code for insertion of residues.  |
//! | 31 - 38 | Real(8.3)    | x          | Orthogonal coordinates for X.    |
//! | 39 - 46 | Real(8.3)    | y          | Orthogonal coordinates for Y.    |
//! | 47 - 54 | Real(8.3)    | z          | Orthogonal coordinates for Z.    |
//! | 55 - 60 | Real(6.2)    | occupancy  | Occupancy.                       |
//! | 61 - 66 | Real(6.2)    | tempFactor | Temperature factor.              |
//! | 77 - 78 | LString(2)   | element    | Element symbol; right-justified. |
//! | 79 - 80 | LString(2)   | charge     | Charge on the atom.              |
//!
//! # Details
//!
//! The x, y, z coordinates are in Angstrom units.
//! No ordering is specified for polysaccharides.
//! See the HET section of this document regarding naming of heterogens. See the Chemical Component Dictionary for residue names, formulas, and topology of the HET groups that have appeared so far in the PDB (see ftp://ftp.wwpdb.org/pub/pdb/data/monomers ).
//! If the depositor provides the data, then the isotropic B value is given for the temperature factor.
//! If there are neither isotropic B values provided by the depositor, nor anisotropic temperature factors in ANISOU, then the default value of 0.0 is used for the temperature factor.
//! Insertion codes and element naming are fully described in the ATOM section of this document.

use super::{Atom, GenericAtomParser};
use crate::common::parser::FieldParser;
use nom::IResult;

pub struct HetAtomParser;

impl FieldParser for HetAtomParser {
    type Output = Atom;
    fn parse(inp: &str) -> IResult<&str, Atom> {
        GenericAtomParser::parse(inp, true)
    }
}
