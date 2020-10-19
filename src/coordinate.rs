// Copyright (c) 2020 Tianyi Shi
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use crate::common::parser::{jump_newline, parse_residue, parse_right, FieldParser};

use crate::types::{
    AminoAcidAtomName, Anisou, Atom, AtomName, AtomSerial, Connect, Element,
    ModifiedAminoAcidTable, ModifiedNucleotideTable, NucleotideAtomName, ParseFw2, ParseFw4,
    Residue,
};
use nom::{bytes::complete::take, character::complete::anychar, combinator::map, IResult};
use std::str::from_utf8_unchecked;

/// # ATOM
///
/// ## Overview
///
/// The ATOM records present the atomic coordinates for standard amino acids and nucleotides. They
/// also present the occupancy and temperature factor for each atom. Non-polymer chemical
/// coordinates use the HETATM record type. The element symbol is always present on each ATOM
/// record; charge is optional. Changes in ATOM/HETATM records result from the standardization atom
/// and residue nomenclature. This nomenclature is described in the [Chemical Component Dictionary](ftp://ftp.wwpdb.org/pub/pdb/data/monomers).
///
/// ## Record Format
///
/// |COLUMNS        |DATA  TYPE   | FIELD       | DEFINITION                                |
/// |---------------|-------------|-------------|-------------------------------------------|
/// | 1 -  6        |Record name  | "ATOM  "    |                                           |
/// | 7 - 11        |Integer      | serial      | Atom  serial number.                      |
/// |13 - 16        |Atom         | name        | Atom name.                                |
/// |17             |Character    | altLoc      | Alternate location indicator.             |
/// |18 - 20        |Residue name | resName     | Residue name.                             |
/// |22             |Character    | chainID     | Chain identifier.                         |
/// |23 - 26        |Integer      | resSeq      | Residue sequence number.                  |
/// |27             |AChar        | iCode       | Code for insertion of residues.           |
/// |31 - 38        |Real(8.3)    | x           | Orthogonal coordinates for X in Angstroms.|
/// |39 - 46        |Real(8.3)    | y           | Orthogonal coordinates for Y in Angstroms.|
/// |47 - 54        |Real(8.3)    | z           | Orthogonal coordinates for Z in Angstroms.|
/// |55 - 60        |Real(6.2)    | occupancy   | Occupancy.                                |
/// |61 - 66        |Real(6.2)    | tempFactor  | Temperature  factor.                      |
/// |77 - 78        |LString(2)   | element     | Element symbol, right-justified.          |
/// |79 - 80        |LString(2)   | charge      | Charge  on the atom.                      |
///
/// ## Details
///
/// ATOM records for proteins are listed from amino to carboxyl terminus.
/// Nucleic acid residues are listed from the 5' to the 3' terminus.
/// Alignment of one-letter atom name such as C starts at column 14, while two-letter atom name such
/// as FE starts at column 13. Atom nomenclature begins with atom type.
/// No ordering is specified for polysaccharides.
/// Non-blank alphanumerical character is used for chain identifier.
/// The list of ATOM records in a chain is terminated by a TER record.
/// If more than one model is present in the entry, each model is delimited by MODEL and ENDMDL
/// records. AltLoc is the place holder to indicate alternate conformation. The alternate
/// conformation can be in the entire polymer chain, or several residues or partial residue (several
/// atoms within one residue). If an atom is provided in more than one position, then a non-blank
/// alternate location indicator must be used for each of the atomic positions. Within a residue,
/// all atoms that are associated with each other in a given conformation are assigned the same
/// alternate position indicator. There are two ways of representing alternate conformation- either
/// at atom level or at residue level (see examples). For atoms that are in alternate sites
/// indicated by the alternate site indicator, sorting of atoms in the ATOM/HETATM list uses the
/// following general rules:
///
/// - In the simple case that involves a few  atoms or a few residues with alternate sites, the
///   coordinates occur one after  the other in the entry.
/// - In the case of a large heterogen groups  which are disordered, the atoms for each conformer
///   are listed together.
///
/// Alphabet letters are commonly used for insertion code. The insertion code is used when two
/// residues have the same numbering. The combination of residue numbering and insertion code
/// defines the unique residue. If the depositor provides the data, then the isotropic B value is
/// given for the temperature factor. If there are neither isotropic B values from the depositor,
/// nor anisotropic temperature factors in ANISOU, then the default value of 0.0 is used for the
/// temperature factor. Columns 79 - 80 indicate any charge on the atom, e.g., 2+, 1-. In most
/// cases, these are blank. For refinements with program REFMAC prior 5.5.0042 which use TLS
/// refinement, the values of B may include only the TLS contribution to the isotropic temperature
/// factor rather than the full isotropic value.
///
/// # HETATOM
///
/// ## Overview
///
/// http://www.wwpdb.org/documentation/file-format-content/format33/sect9.html#HETATM
///
/// Non-polymer or other “non-standard” chemical coordinates, such as water molecules or atoms presented in HET groups use the HETATM record type. They also present the occupancy and temperature factor for each atom. The ATOM records present the atomic coordinates for standard residues. The element symbol is always present on each HETATM record; charge is optional.
///
/// Changes in ATOM/HETATM records will require standardization in atom and residue nomenclature. This nomenclature is described in the Chemical Component Dictionary, ftp://ftp.wwpdb.org/pub/pdb/data/monomers.
///
/// ## Record Format
///
/// | COLUMNS | DATA  TYPE   | FIELD      | DEFINITION                       |
/// | ------- | ------------ | ---------- | -------------------------------- |
/// | 1 - 6   | Record name  | "HETATM"   |                                  |
/// | 7 - 11  | Integer      | serial     | Atom serial number.              |
/// | 13 - 16 | Atom         | name       | Atom name.                       |
/// | 17      | Character    | altLoc     | Alternate location indicator.    |
/// | 18 - 20 | Residue name | resName    | Residue name.                    |
/// | 22      | Character    | chainID    | Chain identifier.                |
/// | 23 - 26 | Integer      | resSeq     | Residue sequence number.         |
/// | 27      | AChar        | iCode      | Code for insertion of residues.  |
/// | 31 - 38 | Real(8.3)    | x          | Orthogonal coordinates for X.    |
/// | 39 - 46 | Real(8.3)    | y          | Orthogonal coordinates for Y.    |
/// | 47 - 54 | Real(8.3)    | z          | Orthogonal coordinates for Z.    |
/// | 55 - 60 | Real(6.2)    | occupancy  | Occupancy.                       |
/// | 61 - 66 | Real(6.2)    | tempFactor | Temperature factor.              |
/// | 77 - 78 | LString(2)   | element    | Element symbol; right-justified. |
/// | 79 - 80 | LString(2)   | charge     | Charge on the atom.              |
///
/// ## Details
///
/// The x, y, z coordinates are in Angstrom units.
/// No ordering is specified for polysaccharides.
/// See the HET section of this document regarding naming of heterogens. See the Chemical Component Dictionary for residue names, formulas, and topology of the HET groups that have appeared so far in the PDB (see ftp://ftp.wwpdb.org/pub/pdb/data/monomers ).
/// If the depositor provides the data, then the isotropic B value is given for the temperature factor.
/// If there are neither isotropic B values provided by the depositor, nor anisotropic temperature factors in ANISOU, then the default value of 0.0 is used for the temperature factor.
/// Insertion codes and element naming are fully described in the ATOM section of this document.
pub struct GenericAtomParser;

impl GenericAtomParser {
    pub fn parse<'a, 'b>(
        inp: &'a [u8],
        modified_aa: &'b ModifiedAminoAcidTable,
        modified_nuc: &'b ModifiedNucleotideTable,
    ) -> IResult<&'a [u8], Atom> {
        let (inp, id) = parse_right::<AtomSerial>(inp, 5)?;
        let (inp, _) = take(1usize)(inp)?;
        let (inp, name) = take(4usize)(inp)?;
        let (inp, id1) = anychar(inp)?;

        let (inp, residue) = parse_residue(inp, modified_aa, modified_nuc)?;

        let name = match &residue {
            Residue::AminoAcid(_) => AtomName::AminoAcid(AminoAcidAtomName::parse_fw4(name)),
            Residue::Nucleotide(_) => AtomName::Nucleotide(NucleotideAtomName::parse_fw4(name)),
            Residue::Water => AtomName::WaterO,
            _ => AtomName::Other(unsafe { from_utf8_unchecked(name).trim().to_owned() }),
        };

        let (inp, _) = take(1usize)(inp)?;
        let (inp, chain) = anychar(inp)?;
        let (inp, sequence_number) = parse_right::<u32>(inp, 4)?;
        let (inp, insertion_code) = anychar(inp)?;
        let (inp, _) = take(3usize)(inp)?;
        let (inp, x) = parse_right::<f32>(inp, 8)?;
        let (inp, y) = parse_right::<f32>(inp, 8)?;
        let (inp, z) = parse_right::<f32>(inp, 8)?;
        let (inp, occupancy) = parse_right::<f32>(inp, 6)?;
        let (inp, temperature_factor) = parse_right::<f32>(inp, 6)?;
        let (inp, _) = take(10usize)(inp)?;
        let (inp, element) = map(take(2usize), Element::parse_fw2)(inp)?;
        let (inp, charge) = map(take(2usize), |x: &[u8]| match x {
            b"  " => 0,
            _ => {
                let x = unsafe { from_utf8_unchecked(x) };
                x.parse::<i8>().unwrap()
            }
        })(inp)?;
        let (inp, _) = nom::character::complete::line_ending(inp)?;
        Ok((
            inp,
            Atom {
                id,
                id1,
                name,
                residue,
                chain,
                sequence_number,
                insertion_code,
                coord: [x, y, z],
                occupancy,
                temperature_factor,
                element,
                charge,
            },
        ))
    }
}

/// # ANISOU
///
/// The [ANISOU](http://www.wwpdb.org/documentation/file-format-content/format33/sect9.html#ANISOU) records present the anisotropic temperature factors.
///
/// ## Record Format
///
/// | COLUMNS | DATA  TYPE   | FIELD    | DEFINITION                       |
/// | ------- | ------------ | -------- | -------------------------------- |
/// | 1 - 6   | Record name  | "ANISOU" |                                  |
/// | 7 - 11  | Integer      | serial   | Atom serial number.              |
/// | 13 - 16 | Atom         | name     | Atom name.                       |
/// | 17      | Character    | altLoc   | Alternate location indicator     |
/// | 18 - 20 | Residue name | resName  | Residue name.                    |
/// | 22      | Character    | chainID  | Chain identifier.                |
/// | 23 - 26 | Integer      | resSeq   | Residue sequence number.         |
/// | 27      | AChar        | iCode    | Insertion code.                  |
/// | 29 - 35 | Integer      | u[0][0]  | U(1,1)                           |
/// | 36 - 42 | Integer      | u[1][1]  | U(2,2)                           |
/// | 43 - 49 | Integer      | u[2][2]  | U(3,3)                           |
/// | 50 - 56 | Integer      | u[0][1]  | U(1,2)                           |
/// | 57 - 63 | Integer      | u[0][2]  | U(1,3)                           |
/// | 64 - 70 | Integer      | u[1][2]  | U(2,3)                           |
/// | 77 - 78 | LString(2)   | element  | Element symbol, right-justified. |
/// | 79 - 80 | LString(2)   | charge   | Charge on the atom.              |
pub struct AnisouParser;

impl FieldParser for AnisouParser {
    type Output = Anisou;
    fn parse(inp: &[u8]) -> IResult<&[u8], Anisou> {
        let (inp, id) = parse_right::<AtomSerial>(inp, 5)?;
        let (inp, _) = take(17usize)(inp)?; // 12 - 28

        let (inp, u11) = parse_right::<i32>(inp, 7)?;
        let (inp, u22) = parse_right::<i32>(inp, 7)?;
        let (inp, u33) = parse_right::<i32>(inp, 7)?;
        let (inp, u12) = parse_right::<i32>(inp, 7)?;
        let (inp, u13) = parse_right::<i32>(inp, 7)?;
        let (inp, u23) = parse_right::<i32>(inp, 7)?;
        let (inp, _) = take(10usize)(inp)?;
        let (inp, _) = nom::character::complete::line_ending(inp)?;
        Ok((
            inp,
            Anisou {
                id,
                u11,
                u22,
                u33,
                u12,
                u13,
                u23,
            },
        ))
    }
}

/// # Overview
///
/// The CONECT records specify connectivity between atoms for which coordinates are supplied. The connectivity is described using the atom serial number as shown in the entry. CONECT records are mandatory for HET groups (excluding water) and for other Connect not specified in the standard residue connectivity table. These records are generated automatically.
///
/// # Record Format
///
/// COLUMNS    |  DATA  TYPE    |  FIELD   |     DEFINITION
/// -----------|----------------|----------|-----------------------------------
///  1 -  6    |   Record name  |  "CONECT"|
///  7 - 11    |  Integer       | serial   |    Atom  serial number
/// 12 - 16    |   Integer      |  serial  |     Serial number of bonded atom
/// 17 - 21    |   Integer      |  serial  |     Serial  number of bonded atom
/// 22 - 26    |   Integer      |  serial  |     Serial number of bonded atom
/// 27 - 31    |   Integer      |  serial  |     Serial number of bonded atom
///
/// Details
///
/// CONECT records are present for:
///
/// - Intra-residue connectivity within  non-standard (HET) residues (excluding water).
/// - Inter-residue connectivity of HET  groups to standard groups (including water) or to other HET groups.
/// - Disulfide bridges specified in the  SSBOND records have corresponding records.
///
/// - No differentiation is made between atoms with delocalized charges (excess negative or positive charge).
/// - Atoms specified in the CONECT records have the same numbers as given in the coordinate section.
/// - All atoms connected to the atom with serial number in columns 7 - 11 are listed in the remaining fields of the record.
/// - If more than four fields are required for non-hydrogen and non-salt bridges, a second CONECT record with the same atom serial number in columns 7 - 11 will be used.
/// - These CONECT records occur in increasing order of the atom serial numbers they carry in columns 7 - 11. The target-atom serial numbers carried on these records also occur in increasing order.
/// - The connectivity list given here is redundant in that each bond indicated is given twice, once with each of the two atoms involved specified in columns 7 - 11.
/// - For hydrogen Connect, when the hydrogen atom is present in the coordinates, a CONECT record between the hydrogen atom and its acceptor atom is generated.
/// - For NMR entries, CONECT records for one model are generated describing heterogen connectivity and others for LINK records assuming that all models are homogeneous models.
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
