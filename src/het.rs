// Copyright (c) 2020 Tianyi Shi
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use crate::common::parser::FieldParser;
use crate::common::parser::{
    jump_newline, parse_residue, parse_right, take_trim_start_own, FieldParserWithModifiedTable,
};
use crate::types::{
    AminoAcidAtomName, AtomName, Helix, HelixClass, ModifiedAminoAcid, ModifiedNucleotide,
    Registration, ResidueSerial, SecondaryStructureSerial, Sense, Sheet, Strand,
};
use nom::{bytes::complete::take, character::complete::anychar, combinator::map, IResult};
use std::collections::HashMap;

/// HET records are used to describe non-standard residues, such as prosthetic groups, inhibitors, solvent molecules, and ions for which coordinates are supplied. Groups are considered HET if they are not part of a biological polymer described in SEQRES and considered to be a molecule bound to the polymer, or they are a chemical species that constitute part of a biological polymer and is not one of the following:
///
/// - standard amino acids, or
/// - standard nucleic acids (C, G, A, U, I, DC, DG, DA, DU, DT and DI), or
/// - unknown amino acid (UNK) or nucleic acid (N) where UNK and N are used to indicate the unknown residue name.
///
/// HET records also describe chemical components for which the chemical identity is unknown, in which case the group is assigned the hetID UNL (Unknown Ligand).
///
/// The heterogen section of a PDB formatted file contains the complete description of non-standard residues in the entry.
///
/// ## Record Format
///
/// | COLUMNS | DATA  TYPE  | FIELD                 | DEFINITION                             |
/// | ------- | ----------- | --------------------- | -------------------------------------- |
/// | 1 -  6  | Record name | "HET   "              |                                        |
/// | 8 - 10  | LString(3)  | hetID                 | Het identifier, right-justified.       |
/// | 13      | Character   | ChainID               | Chain  identifier.                     |
/// | 14 - 17 | Integer     | seqNum                | Sequence  number.                      |
/// | 18      | AChar       | iCode                 | Insertion  code.                       |
/// | 21 - 25 | Integer     | numHetAtoms           | Number of HETATM records for the group |
/// |         |             | present in the entry. |                                        |
/// | 31 - 70 | String      | text                  | Text describing Het group.             |
///
/// ## Details
///
/// - Each HET group is assigned a hetID of not more than three (3) alphanumeric characters. The sequence number, chain identifier, insertion code, and number of coordinate records are given for each occurrence of the HET group in the entry. The chemical name of the HET group is given in the HETNAM record and synonyms for the chemical name are given in the HETSYN records, see ftp://ftp.wwpdb.org/pub/pdb/data/monomers .
/// - There is a separate HET record for each occurrence of the HET group in an entry.
/// - A particular HET group is represented in the PDB archive with a unique hetID.
/// - PDB entries do not have HET records for water molecules, deuterated water, or methanol (when used as solvent).
/// - Unknown atoms or ions will be represented as UNX with the chemical formula X1.  Unknown ligands are UNL; unknown amino acids are UNK.
///
/// ## Verification/Validation/Value Authority Control
///
/// For each het group that appears in the entry, the wwPDB checks that the corresponding HET, HETNAM, HETSYN, FORMUL, HETATM, and CONECT records appear, if applicable. The HET record is generated automatically using the Chemical Component Dictionary and information from the HETATM records.
///
/// Each unique hetID represents a unique molecule.
///
/// ## Relationships to Other Record Types
///
/// For each het group that appears in the entry, there must be corresponding HET, HETNAM, HETSYN, FORMUL,HETATM, and CONECT records. LINK records may also be created.
///
/// Example
///
/// ```ignore
///          1         2         3         4         5         6         7         8
/// 12345678901234567890123456789012345678901234567890123456789012345678901234567890
/// HET     ZN  A  31       1
/// HET    TRS  B 975       8
///
/// HET    UDP  A1457      25
/// HET    B3P  A1458      19
///
/// HET    NAG  Y   3      15
/// HET    FUC  Y   4      10
/// HET    NON  Y   5      12
/// HET    UNK  A 161       1
/// ```
struct HetParser; // ? this this useful?
                  // impl FieldParserWithModifiedTable for HetParser {
                  //     type Output = ();
                  //     fn parse<'a>(inp: &'a [u8], modified_aa: &ModifiedAminoAcidTable,
                  //         modified_nuc: &ModifiedNucleotideTable,) -> IResult<&'a [u8], ()> {
                  //         let (inp, _) = take(1usize)(inp)?;
                  //         let (inp, res) = parse_residue(inp, &modified_aa, &modified_nuc)
                  //     }
                  // }

/// # HETNAM
///
/// ## Overview
///
/// This record gives the chemical name of the compound with the given hetID.
///
/// Record Format
///
/// | COLUMNS | DATA  TYPE   | FIELD        | DEFINITION                                |
/// | ------- | ------------ | ------------ | ----------------------------------------- |
/// | 1 -  6  | Record name  | "HETNAM"     |                                           |
/// | 9 - 10  | Continuation | continuation | Allows concatenation of multiple records. |
/// | 12 - 14 | LString(3)   | hetID        | Het identifier, right-justified.          |
/// | 16 - 70 | String       | text         | Chemical name.                            |
///
/// ## Details
///
/// - Each hetID is assigned a unique chemical name for the HETNAM record, see ftp://ftp.wwpdb.org/pub/pdb/data/monomers.
/// - Other names for the group are given on HETSYN records.
/// - PDB entries follow IUPAC/IUB naming conventions to describe groups systematically.
/// - The special character “~” is used to indicate superscript in a heterogen name. For example: N6 will be listed in the HETNAM section as N~6~, with the ~ character indicating both the start and end of the superscript in the name, e.g., `N-(BENZYLSULFONYL)SERYL-N~1~-{4-[AMINO(IMINO)METHYL]BENZYL}GLYCINAMIDE`
///
/// Continuation of chemical names onto subsequent records is allowed.
/// Only one HETNAM record is included for a given hetID, even if the same hetID appears on more than one HET record.
///
/// Verification/Validation/Value Authority Control
///
/// For each het group that appears in the entry, the corresponding HET, HETNAM, FORMUL, HETATM, and CONECT records must appear. The HETNAM record is generated automatically using the Chemical Component Dictionary and information from HETATM records.
///
/// Relationships to Other Record Types
///
/// For each het group that appears in the entry, there must be corresponding HET, HETNAM, FORMUL, HETATM, and CONECT records. HETSYN and LINK records may also be created.
///
/// ## Example
///
/// ```ignore
///          1         2         3         4         5         6         7         8
/// 12345678901234567890123456789012345678901234567890123456789012345678901234567890
/// HETNAM     NAG N-ACETYL-D-GLUCOSAMINE
/// HETNAM     SAD BETA-METHYLENE SELENAZOLE-4-CARBOXAMIDE ADENINE
/// HETNAM  2  SAD DINUCLEOTIDE
///
/// HETNAM     UDP URIDINE-5'-DIPHOSPHATE
///
/// HETNAM     UNX UNKNOWN ATOM OR ION
/// HETNAM     UNL UNKNOWN LIGAND
///
/// HETNAM     B3P 2-[3-(2-HYDROXY-1,1-DIHYDROXYMETHYL-ETHYLAMINO)-                 
/// HETNAM   2 B3P  PROPYLAMINO]-2-HYDROXYMETHYL-PROPANE-1,3-DIOL   
/// ```
pub struct HetnamParser;

pub struct FormulParser;
