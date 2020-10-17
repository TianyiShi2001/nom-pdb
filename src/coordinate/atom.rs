//! # Overview
//!
//! The ATOM records present the atomic coordinates for standard amino acids and nucleotides. They
//! also present the occupancy and temperature factor for each atom. Non-polymer chemical
//! coordinates use the HETATM record type. The element symbol is always present on each ATOM
//! record; charge is optional. Changes in ATOM/HETATM records result from the standardization atom
//! and residue nomenclature. This nomenclature is described in the [Chemical Component Dictionary](ftp://ftp.wwpdb.org/pub/pdb/data/monomers).
//!
//! # Record Format
//!
//! |COLUMNS        |DATA  TYPE   | FIELD       | DEFINITION                                |
//! |---------------|-------------|-------------|-------------------------------------------|
//! | 1 -  6        |Record name  | "ATOM  "    |                                           |
//! | 7 - 11        |Integer      | serial      | Atom  serial number.                      |
//! |13 - 16        |Atom         | name        | Atom name.                                |
//! |17             |Character    | altLoc      | Alternate location indicator.             |
//! |18 - 20        |Residue name | resName     | Residue name.                             |
//! |22             |Character    | chainID     | Chain identifier.                         |
//! |23 - 26        |Integer      | resSeq      | Residue sequence number.                  |
//! |27             |AChar        | iCode       | Code for insertion of residues.           |
//! |31 - 38        |Real(8.3)    | x           | Orthogonal coordinates for X in Angstroms.|
//! |39 - 46        |Real(8.3)    | y           | Orthogonal coordinates for Y in Angstroms.|
//! |47 - 54        |Real(8.3)    | z           | Orthogonal coordinates for Z in Angstroms.|
//! |55 - 60        |Real(6.2)    | occupancy   | Occupancy.                                |
//! |61 - 66        |Real(6.2)    | tempFactor  | Temperature  factor.                      |
//! |77 - 78        |LString(2)   | element     | Element symbol, right-justified.          |
//! |79 - 80        |LString(2)   | charge      | Charge  on the atom.                      |
//!
//! # Details
//!
//! ATOM records for proteins are listed from amino to carboxyl terminus.
//! Nucleic acid residues are listed from the 5' to the 3' terminus.
//! Alignment of one-letter atom name such as C starts at column 14, while two-letter atom name such
//! as FE starts at column 13. Atom nomenclature begins with atom type.
//! No ordering is specified for polysaccharides.
//! Non-blank alphanumerical character is used for chain identifier.
//! The list of ATOM records in a chain is terminated by a TER record.
//! If more than one model is present in the entry, each model is delimited by MODEL and ENDMDL
//! records. AltLoc is the place holder to indicate alternate conformation. The alternate
//! conformation can be in the entire polymer chain, or several residues or partial residue (several
//! atoms within one residue). If an atom is provided in more than one position, then a non-blank
//! alternate location indicator must be used for each of the atomic positions. Within a residue,
//! all atoms that are associated with each other in a given conformation are assigned the same
//! alternate position indicator. There are two ways of representing alternate conformation- either
//! at atom level or at residue level (see examples). For atoms that are in alternate sites
//! indicated by the alternate site indicator, sorting of atoms in the ATOM/HETATM list uses the
//! following general rules:
//!
//! - In the simple case that involves a few  atoms or a few residues with alternate sites, the
//!   coordinates occur one after  the other in the entry.
//! - In the case of a large heterogen groups  which are disordered, the atoms for each conformer
//!   are listed together.
//!
//! Alphabet letters are commonly used for insertion code. The insertion code is used when two
//! residues have the same numbering. The combination of residue numbering and insertion code
//! defines the unique residue. If the depositor provides the data, then the isotropic B value is
//! given for the temperature factor. If there are neither isotropic B values from the depositor,
//! nor anisotropic temperature factors in ANISOU, then the default value of 0.0 is used for the
//! temperature factor. Columns 79 - 80 indicate any charge on the atom, e.g., 2+, 1-. In most
//! cases, these are blank. For refinements with program REFMAC prior 5.5.0042 which use TLS
//! refinement, the values of B may include only the TLS contribution to the isotropic temperature
//! factor rather than the full isotropic value.

use super::{Atom, GenericAtomParser};
use crate::common::parser::FieldParser;
use nom::IResult;

pub struct AtomParser;

impl FieldParser for AtomParser {
    type Output = Atom;
    fn parse(inp: &str) -> IResult<&str, Atom> {
        GenericAtomParser::parse(inp, false)
    }
}
