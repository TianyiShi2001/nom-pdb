// Copyright (c) 2020 Tianyi Shi
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

//! # nom-pdb
//!
//! A PDB (Protein Data Bank) file parser implemented with nom.
//!
//! See [github repository](https://github.com/TianyiShi2001/nom-pdb) for examples.

pub mod common;
pub mod complete;
pub mod coordinate;
pub mod crystallography;
pub mod het;
pub mod primary_structure;
pub mod remark;
pub mod secondary_structure;
pub mod title_section;
pub(crate) mod types;

pub use complete::Parser;

// /// http://www.wwpdb.org/documentation/file-format-content/format33/sect1.html
// #[derive(Eq, PartialEq, Debug, Ord, PartialOrd)]
// enum Fields {
//     Header,  // M
//     Obslte,  // O : Mandatory in entries that have been replaced by a newer entry.
//     Title,   // M
//     Split, //   O : Mandatory when  large macromolecular complexes are split into multiple PDB entries.
//     Caveat, // O : Mandatory when there are outstanding errors such as chirality.
//     Compnd, // M
//     Source, // M
//     Keywds, // M
//     Expdta, // M
//     Nummdl, // O : Mandatory for  NMR ensemble entries.
//     Mdltyp, // O : Mandatory for  NMR minimized average Structures or when the entire  polymer chain contains C alpha or P atoms only.
//     Author, // M
//     Revdat, // M
//     Sprsde, // O : Mandatory for a replacement entry.
//     Jrnl,   //   O: Mandatory for a publication describes the experiment.
//     Remark0, // O : Mandatory for a re-refined structure
//     Remark1, // O
//     Remark2, // M
//     Remark3, // M
//     RemarkN, // O : Mandatory under certain conditions
//     Dbref,  //  O : Mandatory for all polymers.
//     Dbref1,
//     Dbref2, // O : Mandatory when certain sequence database accession and/or sequence numbering does not fit preceding DBREF format.
//     SeqAdv, // O : Mandatory if sequence  conflict exists.
//     SeqRes, // O : Mandatory if ATOM records exist.
//     Modres, // O : Mandatory if modified group exists in the coordinates.
//     Het,    // O : Mandatory if a non-standard group other than water appears in the coordinates.
//     Hetnam, // O : Mandatory if a non-standard group otherthan  water appears in the coordinates.
//     Hetsyn, // O
//     Formul, // O : Mandatory if a non-standard group or water appears in the coordinates.
//     Helix,  // O
//     Sheet,  // O
//     Ssbond, // O : Mandatory if a  disulfide bond is present.
//     Link,   // O : Mandatory if  non-standard residues appear in a polymer
//     Cispep, // O
//     Site,   // O
//     Cryst1, // M
//     Origx1, // M
//     Origx2, // M
//     Origx3, // M
//     Scale1, // M
//     Scale2, // M
//     Scale3, // M
//     Mtrix1, // O Mandatory if  the complete asymmetric unit
//     Mtrix2, // O must  be generated from the given coordinates
//     Mtrix3, // O using non-crystallographic symmetry.
//     Model,  // O : Mandatory if more than one model is present in the entry.
//     Atom,   // O : Mandatory if standard residues exist.
//     Anisou, // O
//     Ter,    // O : Mandatory if ATOM records exist.
//     Hetatm, // O : Mandatory if non-standard group exists.
//     Endmdl, // O : Mandatory if MODEL appears.
//     Conect, // O : Mandatory if non-standard group appears and if LINK or SSBOND records exist.
//     Master, // M
//     End,    // M
// }

// #[derive(Eq, PartialEq, Debug, Ord, PartialOrd)]
// enum Section {
//     Title, // HEADER,  OBSLTE, TITLE, SPLIT, CAVEAT, COMPND, SOURCE, KEYWDS, EXPDTA, NUMMDL, MDLTYP, AUTHOR, REVDAT, PRSDE, JRNL
//     Remark, // REMARKs  0-999
//     PrimaryStructure, // DBREF, SEQADV, SEQRES MODRES
//     Heterogen, // HET, HETNAM, HETSYN, FORMUL
//     SecondaryStructure, // HELIX, SHEET
//     Connectivity, // CONECT, SSBOND, LINK, CISPEP
//     Misc,  // SITE
//     Crystallography, // CRYST1
//     CoordinateTransformation, // ORIGXn,  SCALEn, MTRIXn,
//     Coordinate, // MODEL, ATOM, ANISOU, TER, HETATM, ENDMDL
// }
// /// Commas, colons, and semi-colons are used as list delimiters in records that have one of the following data types:
// ///
// /// - List
// /// - SList
// /// - Specification List
// /// - Specification
// ///
// /// If a comma, colon, or semi-colon is used in any context other than as a delimiting character, then the character must be escaped, i.e., immediately preceded by a backslash, "\".
// enum Dtype {
//     // To interpret a String, concatenate the contents of all continued fields together, collapse all sequences of multiple blanks to a single blank, and remove any leading and trailing blanks. This permits very long strings to be properly reconstructed.
//     List,              // A String that is composed of text separated with commas.
//     SList,             // A String that is composed of text separated with semi-colons.
//     Specification, // A String composed of a token and its  associated value separated by a colon.
//     SpecificationList, // A sequence of Specifications, separated by semi-colons.
//     String,
//     Oneline,
// }

// use std::str::from_utf8_unchecked;

// use std::fs::read;
// use std::fs::read_to_string;
// use std::fs::File;

// pub enum Record<'a> {
//     Header(title_section::header::Header<'a>),
//     Authors(Vec<&'a [u8]>),
//     Keywords(Vec<&'a [u8]>),
//     Cryst1(crystallography::cryst1::Cryst1),
// }

// use memmap::MmapOptions;
// pub unsafe fn apply_file_content_unsafe<F, T>(fp: &[u8], parser: F) -> Result<T, std::io::Error>
// where
//     F: FnOnce(&[u8]) -> T,
// {
//     let file = File::open(fp)?;
//     let mmap = MmapOptions::new().map(&file)?;
//     let data = from_utf8_unchecked(&mmap[..]);
//     let res = parser(data);
//     Ok(res)
// }

// pub fn apply_file_content<F, T>(fp: &[u8], parser: F) -> Result<T, std::io::Error>
// where
//     F: FnOnce(&[u8]) -> T,
// {
//     let bytes = read(fp)?;
//     let data = unsafe { from_utf8_unchecked(&bytes) };
//     let res = parser(data);
//     Ok(res)
// }
