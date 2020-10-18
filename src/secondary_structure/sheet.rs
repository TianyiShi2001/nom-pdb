//! # Overview
//!
//! SHEET records are used to identify the position of sheets in the molecule. Sheets are both named and numbered. The residues where the sheet begins and ends are noted.
//!
//! # Record Format
//!
//! | COLUMNS | DATA  TYPE   | FIELD       | DEFINITION                                        |
//! | ------- | ------------ | ----------- | ------------------------------------------------- |
//! | 1 -  6  | Record name  | "SHEET "    |                                                   |
//! | 8 - 10  | Integer      | strand      | Strand  number which starts at 1 for each         |
//! |         |              |             | strand within a sheet and increases by one.       |
//! | 12 - 14 | LString(3)   | sheetID     | Sheet  identifier.                                |
//! | 15 - 16 | Integer      | numStrands  | Number  of strands in sheet.                      |
//! | 18 - 20 | Residue name | initResName | Residue  name of initial residue.                 |
//! | 22      | Character    | initChainID | Chain identifier of initial residue               |
//! |         |              |             | in strand.                                        |
//! | 23 - 26 | Integer      | initSeqNum  | Sequence number of initial residue                |
//! |         |              |             | in strand.                                        |
//! | 27      | AChar        | initICode   | Insertion code of initial residue                 |
//! |         |              |             | in  strand.                                       |
//! | 29 - 31 | Residue name | endResName  | Residue name of terminal residue.                 |
//! | 33      | Character    | endChainID  | Chain identifier of terminal residue.             |
//! | 34 - 37 | Integer      | endSeqNum   | Sequence number of terminal residue.              |
//! | 38      | AChar        | endICode    | Insertion code of terminal residue.               |
//! | 39 - 40 | Integer      | sense       | Sense of strand with respect to previous          |
//! |         |              |             | strand in the sheet. 0 if first strand,           |
//! |         |              |             | 1 if  parallel,and -1 if anti-parallel.           |
//! | 42 - 45 | Atom         | curAtom     | Registration.  Atom name in current strand.       |
//! | 46 - 48 | Residue name | curResName  | Registration.  Residue name in current strand     |
//! | 50      | Character    | curChainId  | Registration. Chain identifier in                 |
//! |         |              |             | current strand.                                   |
//! | 51 - 54 | Integer      | curResSeq   | Registration.  Residue sequence number            |
//! |         |              |             | in current strand.                                |
//! | 55      | AChar        | curICode    | Registration. Insertion code in                   |
//! |         |              |             | current strand.                                   |
//! | 57 - 60 | Atom         | prevAtom    | Registration.  Atom name in previous strand.      |
//! | 61 - 63 | Residue name | prevResName | Registration.  Residue name in                    |
//! |         |              |             | previous strand.                                  |
//! | 65      | Character    | prevChainId | Registration.  Chain identifier in                |
//! |         |              |             | previous  strand.                                 |
//! | 66 - 69 | Integer      | prevResSeq  | Registration. Residue sequence number             |
//! |         |              |             | in previous strand.                               |
//! | 70      | AChar        | prevICode   | Registration.  Insertion code in previous strand. |
//!
//! # Details
//!
//! - The initial residue for a strand is its N-terminus. Strand registration information is provided in columns 39 - 70. Strands are listed starting with one edge of the sheet and continuing to the spatially adjacent strand.
//! - The sense in columns 39 - 40 indicates whether strand n is parallel (sense = 1) or anti-parallel (sense = -1) to strand n-1. Sense is equal to zero (0) for the first strand of a sheet.
//! - The registration (columns 42 - 70) of strand n to strand n-1 may be specified by one hydrogen bond between each such pair of strands. This is done by providing the hydrogen bonding between the current and previous strands. No register information should be provided for the first strand.
//! - Split strands, or strands with two or more runs of residues from discontinuous parts of the amino acid sequence, are explicitly listed. Detail description can be included in the REMARK 700 .

use crate::common::parser::FieldParser;
use crate::common::parser::{jump_newline, parse_right, take_trim_start_own};
use crate::types::{
    AminoAcidAtomName, AtomName, Registration, ResidueSerial, SecondaryStructureSerial, Sense,
    Sheet, Strand,
};
use nom::{bytes::complete::take, character::complete::anychar, combinator::map, IResult};

pub struct SheetParser;

impl FieldParser for SheetParser {
    type Output = Sheet;
    fn parse(inp: &[u8]) -> IResult<&[u8], Self::Output> {
        Self::parse_sheet(inp)
    }
}

impl SheetParser {
    fn parse_sheet(inp: &[u8]) -> IResult<&[u8], Sheet> {
        let mut sheet = Sheet::default();
        // first line
        let (inp, _) = take(5usize)(inp)?; // 7 - 11
        let (inp, id) = unsafe { take_trim_start_own(inp, 3usize)? }; // 12 - 14
        sheet.id = id;
        let (inp, num_strands) = parse_right::<SecondaryStructureSerial>(inp, 2)?; // 15 - 16
        let (inp, _) = take(1usize)(inp)?; // 17
        let (inp, first_strand) = Self::parse_first_line(inp)?;
        sheet.strands.push(first_strand);
        let mut i = 1 as SecondaryStructureSerial;
        let mut last_inp = inp;
        while i < num_strands {
            let (inp, _) = take(7usize)(last_inp)?; // 1 - 7
            let (inp, idx) = parse_right::<SecondaryStructureSerial>(inp, 3)?; // 8 - 10
            i = idx;
            let (inp, _) = take(7usize)(inp)?; // 11 - 17
            let (inp, (strand, registration)) = Self::parse_line(inp)?;
            sheet.strands.push(strand);
            sheet.registration.push(registration);
            last_inp = inp;
        }
        Ok((last_inp, sheet))
    }

    fn parse_first_line(inp: &[u8]) -> IResult<&[u8], Strand> {
        let (inp, res) = Self::parse_strand(inp)?;
        let (inp, _) = jump_newline(inp)?;
        Ok((inp, res))
    }

    fn parse_line(inp: &[u8]) -> IResult<&[u8], (Strand, Registration)> {
        let (inp, strand) = Self::parse_strand(inp)?;
        let (inp, _) = take(1usize)(inp)?;
        let (inp, registration) = Self::parse_registration(inp)?;
        Ok((inp, (strand, registration)))
    }

    fn parse_strand(inp: &[u8]) -> IResult<&[u8], Strand> {
        // let (inp, _start_res) = map(take(3usize), parse_amino_acid)(inp)?;
        let (inp, _) = take(3usize)(inp)?; // 18 - 20
        let (inp, _) = take(1usize)(inp)?; //           21
        let (inp, start_chain) = anychar(inp)?; // 22
        let (inp, start_serial) = parse_right::<ResidueSerial>(inp, 4)?; // 23 - 26
        let (inp, _start_icode) = anychar(inp)?; // 27
        let (inp, _) = take(1usize)(inp)?; // 28
                                           // let (inp, _end_res) = map(take(3usize), parse_amino_acid)(inp)?;
        let (inp, _) = take(3usize)(inp)?; // 29 - 31
        let (inp, _) = take(1usize)(inp)?; //      32
        let (inp, end_chain) = anychar(inp)?; // 33
        let (inp, end_serial) = parse_right::<ResidueSerial>(inp, 4)?; // 34 - 37
        let (inp, _end_icode) = anychar(inp)?; // 38
        let (inp, sense) = Self::parse_sense(inp)?;
        let strand = Strand {
            start: (start_chain, start_serial),
            end: (end_chain, end_serial),
            sense,
        };
        Ok((inp, strand))
    }
    fn parse_registration(inp: &[u8]) -> IResult<&[u8], Registration> {
        // | 42 - 45 | Atom         | curAtom     | Registration.  Atom name in current strand.       |
        // | 46 - 48 | Residue name | curResName  | Registration.  Residue name in current strand     |
        // | 50      | Character    | curChainId  | Registration. Chain identifier in                 |
        // |         |              |             | current strand.                                   |
        // | 51 - 54 | Integer      | curResSeq   | Registration.  Residue sequence number            |
        // |         |              |             | in current strand.                                |
        // | 55      | AChar        | curICode    | Registration. Insertion code in                   |
        // |         |              |             | current strand.                                   |
        // | 57 - 60 | Atom         | prevAtom    | Registration.  Atom name in previous strand.      |
        // | 61 - 63 | Residue name | prevResName | Registration.  Residue name in                    |
        // |         |              |             | previous strand.                                  |
        // | 65      | Character    | prevChainId | Registration.  Chain identifier in                |
        // |         |              |             | previous  strand.                                 |
        // | 66 - 69 | Integer      | prevResSeq  | Registration. Residue sequence number             |
        // |         |              |             | in previous strand.                               |
        // | 70      | AChar        | prevICode   | Registration.  Insertion code in previous strand. |
        let (inp, cur_atom) = map(take(4usize), |s| {
            AtomName::AminoAcid(AminoAcidAtomName::from_bytes_fixed4(s))
        })(inp)?; // 42 - 45
        let (inp, _) = take(4usize)(inp)?; // 46 - 48; 49
        let (inp, cur_chain) = anychar(inp)?; // 50
        let (inp, cur_serial) = parse_right::<ResidueSerial>(inp, 4)?; // 51 - 54
        let (inp, _) = take(2usize)(inp)?; // 55; 56
        let (inp, prev_atom) = map(take(4usize), |s| {
            AtomName::AminoAcid(AminoAcidAtomName::from_bytes_fixed4(s))
        })(inp)?; // 57 - 60
        let (inp, _) = take(4usize)(inp)?; // 61 - 63; 64
        let (inp, prev_chain) = anychar(inp)?; // 65
        let (inp, prev_serial) = parse_right::<ResidueSerial>(inp, 4)?; // 66 - 69
        let (inp, _) = jump_newline(inp)?;
        let registration = Registration {
            curr: (cur_atom, cur_chain, cur_serial),
            prev: (prev_atom, prev_chain, prev_serial),
        };
        Ok((inp, registration))
    }

    fn parse_sense(inp: &[u8]) -> IResult<&[u8], Sense> {
        let (inp, sense) = take(2usize)(inp)?;
        let sense = match sense {
            b" 1" => Sense::Parallel,
            b" 0" => Sense::Unknown,
            b"-1" => Sense::Antiparallel,
            _ => panic!("Error when parsing beta-strand sense!"),
        };
        Ok((inp, sense))
    }
}
