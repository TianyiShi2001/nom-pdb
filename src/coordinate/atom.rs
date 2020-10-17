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

use crate::common::parser::FieldParser;
use crate::common::parser::{parse_amino_acid, parse_right_f32, parse_right_i8, parse_right_u32};
use crate::common::types::AminoAcid;
use nom::bytes::complete::take;
use nom::character::complete::anychar;
use nom::combinator::map;
use nom::IResult;
use std::str::FromStr;
#[derive(Debug, Clone)]
pub struct Atom {
    pub id: u32,
    pub name: AminoAcidAtomName,
    pub id1: char,
    pub residue: AminoAcid,
    pub chain: char,
    pub sequence_number: u32,
    pub insertion_code: char,
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub occupancy: f32,
    pub temperature_factor: f32,
    pub element: Element,
    pub charge: i8,
}
pub struct AtomParser;

impl FieldParser for AtomParser {
    type Output = Atom;
    fn parse(inp: &str) -> IResult<&str, Atom> {
        let (inp, id) = parse_right_u32(inp, 5)?;
        let (inp, _) = take(1usize)(inp)?;
        // ! to be implemented
        let (inp, name) = map(map(take(4usize), str::trim), |x| {
            AminoAcidAtomName::from_str(x).unwrap()
        })(inp)?;
        // let (inp, _) = take(4usize)(inp)?;
        let (inp, id1) = anychar(inp)?;
        let (inp, residue) = parse_amino_acid(inp)?;
        let (inp, _) = take(1usize)(inp)?;
        let (inp, chain) = anychar(inp)?;
        let (inp, sequence_number) = parse_right_u32(inp, 4)?;
        let (inp, insertion_code) = anychar(inp)?;
        let (inp, _) = take(3usize)(inp)?;
        let (inp, x) = parse_right_f32(inp, 8)?;
        let (inp, y) = parse_right_f32(inp, 8)?;
        let (inp, z) = parse_right_f32(inp, 8)?;
        let (inp, occupancy) = parse_right_f32(inp, 6)?;
        let (inp, temperature_factor) = parse_right_f32(inp, 6)?;
        let (inp, _) = take(10usize)(inp)?;
        let (inp, element) = map(map(take(2usize), str::trim_start), |x| {
            Element::from_str(x).unwrap()
        })(inp)?;
        let (inp, charge) = map(take(2usize), |x| match x {
            "  " => 0,
            _ => x.parse::<i8>().unwrap(),
        })(inp)?;
        let (inp, _) = nom::character::complete::line_ending(inp)?;
        Ok((
            inp,
            Atom {
                id,
                name,
                id1,
                residue,
                chain,
                sequence_number,
                insertion_code,
                x,
                y,
                z,
                occupancy,
                temperature_factor,
                element,
                charge,
            },
        ))
    }
}

// ! to be implemented
#[derive(Debug, Clone)]
pub enum AminoAcidAtomName {
    C,
    CA,
    CB,
    CD,
    CD1,
    CD2,
    CE,
    CE1,
    CE2,
    CG,
    CG1,
    CG2,
    CZ,
    O,
    OG,
    OD1,
    OD2,
    OE,
    OE1,
    OE2,
    N,
    NE,
    NH,
    NH1,
    NH2,
    NZ,
    SD,
}

impl FromStr for AminoAcidAtomName {
    type Err = String;
    fn from_str(inp: &str) -> std::result::Result<Self, <Self as std::str::FromStr>::Err> {
        match inp {
            "C" => Ok(AminoAcidAtomName::C),
            "CA" => Ok(AminoAcidAtomName::CA),
            "CB" => Ok(AminoAcidAtomName::CB),
            "CD" => Ok(AminoAcidAtomName::CD),
            "CD1" => Ok(AminoAcidAtomName::CD1),
            "CD2" => Ok(AminoAcidAtomName::CD2),
            "CE" => Ok(AminoAcidAtomName::CE),
            "CE1" => Ok(AminoAcidAtomName::CE1),
            "CE2" => Ok(AminoAcidAtomName::CE2),
            "CG" => Ok(AminoAcidAtomName::CG),
            "CG1" => Ok(AminoAcidAtomName::CG1),
            "CG2" => Ok(AminoAcidAtomName::CG2),
            "CZ" => Ok(AminoAcidAtomName::CZ),
            "O" => Ok(AminoAcidAtomName::O),
            "OD1" => Ok(AminoAcidAtomName::OD1),
            "OD2" => Ok(AminoAcidAtomName::OD2),
            "OG" => Ok(AminoAcidAtomName::OG),
            "OE" => Ok(AminoAcidAtomName::OE),
            "OE1" => Ok(AminoAcidAtomName::OE1),
            "OE2" => Ok(AminoAcidAtomName::OE2),
            "N" => Ok(AminoAcidAtomName::N),
            "NE" => Ok(AminoAcidAtomName::NE),
            "NH" => Ok(AminoAcidAtomName::NH),
            "NH1" => Ok(AminoAcidAtomName::NH1),
            "NH2" => Ok(AminoAcidAtomName::NH2),
            "NZ" => Ok(AminoAcidAtomName::NZ),
            "SD" => Ok(AminoAcidAtomName::SD),
            _ => Err(format!("Unknown atom name {}", inp)),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Element {
    H,
    C,
    O,
    N,
    P,
    S,
    Na,
    Mg,
    Cl,
    K,
    Ca,
    Fe,
    Mn,
    Co,
    Cr,
    I,
    Zn,
    Cu,
    F,
    Al,
}

impl FromStr for Element {
    type Err = String;
    fn from_str(inp: &str) -> std::result::Result<Self, <Self as std::str::FromStr>::Err> {
        match inp {
            "H" => Ok(Element::H),
            "C" => Ok(Element::C),
            "O" => Ok(Element::O),
            "N" => Ok(Element::N),
            "P" => Ok(Element::P),
            "S" => Ok(Element::S),
            "Na" => Ok(Element::Na),
            "Mg" => Ok(Element::Mg),
            "Cl" => Ok(Element::Cl),
            "K" => Ok(Element::K),
            "Ca" => Ok(Element::Ca),
            "Fe" => Ok(Element::Fe),
            "Mn" => Ok(Element::Mn),
            "Co" => Ok(Element::Co),
            "Cr" => Ok(Element::Cr),
            "I" => Ok(Element::I),
            "Zn" => Ok(Element::Zn),
            "Cu" => Ok(Element::Cu),
            "F" => Ok(Element::F),
            "Al" => Ok(Element::Al),
            _ => Err(format!("Unknown atom name {}", inp)),
        }
    }
}
