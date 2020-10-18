use chrono::{
    format::{strftime::StrftimeItems, Parsed},
    NaiveDate,
};
use nom::{
    bytes::complete::{is_not, tag, take, take_while},
    character::complete::{anychar, char, line_ending, multispace0, multispace1, not_line_ending},
    combinator::{map, map_res, peek},
    error::ParseError,
    sequence::preceded,
    IResult,
};
use protein_core::types::AminoAcid;
use std::str::FromStr;

pub trait FieldParser {
    type Output;
    fn parse(inp: &str) -> IResult<&str, Self::Output>;
    fn parse_into<'a, 'b>(inp: &'a str, dst: &'b mut Self::Output) -> &'a str {
        let (i, data) = Self::parse(inp).expect("parse error");
        *dst = data;
        i
    }
    fn parse_into_vec<'a>(inp: &'a str, dst: &mut Vec<Self::Output>) -> &'a str {
        let (i, data) = Self::parse(inp).expect("parse error");
        dst.push(data);
        i
    }
}

// fn ws<'a, F: 'a, O, E: ParseError<&'a str>>(inner: F) -> impl Fn(&'a str) -> IResult<&'a str, O, E>
// where
//     F: Fn(&'a str) -> IResult<&'a str, O, E>,
// {
//     preceded(multispace0, &inner)(i)
// }

pub(crate) fn jump_newline(inp: &str) -> IResult<&str, ()> {
    let (inp, _) = not_line_ending(inp)?;
    let (inp, _) = line_ending(inp)?;
    Ok((inp, ()))
}

fn char_is_space(c: char) -> bool {
    c == ' '
}

pub(crate) fn parse_date(i: &str) -> IResult<&str, NaiveDate> {
    let (i, day) = take(2usize)(i)?;
    let (i, _) = take(1usize)(i)?;
    let (i, month) = parse_month(i)?;
    let (i, _) = take(1usize)(i)?;
    let (i, year) = take(2usize)(i)?;
    let mut year = year.parse::<i32>().unwrap();
    if year < 50i32 {
        year += 2000
    } else {
        year += 1900
    }
    Ok((
        i,
        NaiveDate::from_ymd(year, month, day.parse::<u32>().unwrap()),
    ))
}

fn parse_month(i: &str) -> IResult<&str, u32> {
    map_res(take(3usize), |s: &str| -> Result<u32, ()> {
        let mut parsed = Parsed::new();
        chrono::format::parse(&mut parsed, s, StrftimeItems::new("%b"))
            .expect("Failed to parse month");
        Ok(parsed.month.unwrap())
    })(i)
}

pub(crate) fn parse_right<'a, T>(i: &'a str, length: usize) -> IResult<&'a str, T>
where
    T: FromStr + std::fmt::Debug,
{
    let (i, s) = take_while(char_is_space)(i)?;
    let l = s.len();
    if l >= length {
        panic!("Failed to parse int")
    }
    let (i, digit) = take(length - l)(i)?;
    match digit.parse() {
        Err(_) => Err(nom::Err::Error((i, nom::error::ErrorKind::Digit))),
        Ok(x) => Ok((i, x)),
    }
}

// * MULTILINE PARSERS ---------------------------------------------------------

pub(crate) fn parse_multiline_list(inp: &str) -> IResult<&str, Vec<String>> {
    // ! need improvement
    let (mut inp, _) = take(4usize)(inp)?; // 7 - 10
    let mut v: Vec<String> = Vec::new();
    loop {
        let (i, item) = take_while(|c| c != ',' && c != '\n' && c != '\r')(inp)?;
        v.push(item.trim().to_owned());
        let (i, comma_or_newline) = anychar(i)?; // consume \r or \n if newline
        if comma_or_newline == ',' {
            let (i, char_after_comma) = peek(anychar)(i)?;
            if char_after_comma == ' ' {
                let (i, second_char_after_comma) = peek(anychar)(anychar(i)?.0)?;
                if !second_char_after_comma.is_alphanumeric() {
                    // newline
                    inp = multispace1(i)?.0;
                    inp = take(10usize)(inp)?.0; // 1 - 10
                } else {
                    inp = i;
                }
            } else {
                inp = i;
            }
        } else {
            // end
            inp = take_while(|x| x == '\n')(i)?.0;
            return Ok((inp, v));
        }
    }
}

pub(crate) fn parse_multiline_string<'a>(
    inp: &'a str,
    record_identifier: &str,
) -> IResult<&'a str, String> {
    // ! need improvement
    let (mut inp, _) = take(4usize)(inp)?; // 7 - 10
    let mut s = String::new();
    loop {
        let (i, item) = not_line_ending(inp)?;
        s.push_str(item.trim_end());
        let (i, _) = line_ending(i)?;
        if peek(take(6usize))(i)?.1 != record_identifier {
            return Ok((i, s));
        }
        let (i, _) = take(10usize)(i)?;
        inp = i;
    }
}

pub(crate) fn parse_multiline<'a, T, F>(
    inp: &'a str,
    record_identifier: &str,
    continuation: bool,
    parse_oneline: F,
) -> IResult<&'a str, Vec<T>>
where
    F: Fn(&'a str) -> IResult<&'a str, T>,
{
    // ! need improvement
    let offset = if continuation { 10usize } else { 6usize };
    let (mut inp, _) = take(4usize)(inp)?; // 7 - 10
    let mut res = Vec::<T>::new();
    loop {
        let (i, item) = parse_oneline(inp)?;
        res.push(item);
        if peek(take(6usize))(i)?.1 != record_identifier {
            return Ok((i, res));
        }
        let (i, _) = take(offset)(i)?;
        inp = i;
    }
}

// pub(crate) fn parse_specification(inp: &str) -> IResult<&str, Token> {
//     let (mut inp, _) = take(4usize)(inp)?;
//     let (inp, token) = is_not(":")(inp)?;
// }

/// Represents keys of CMPND and SOURCE records
#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    MoleculeId(u32),
    Molecule(String),
    Chain { identifiers: Vec<String> },
    Fragment(String),
    Synonym { synonyms: Vec<String> },
    Ec { commission_numbers: Vec<String> },
    Engineered(bool),
    Mutation(bool),
    OtherDetails(String),
    Synthetic(String),
    OrganismScientific(String),
    OrganismCommon { organisms: Vec<String> },
    OrganismTaxId { id: Vec<u32> },
    Strain(String),
    Variant(String),
    CellLine(String),
    Atcc(u32),
    Organ(String),
    Tissue(String),
    Cell(String),
    Organelle(String),
    Secretion(String),
    CellularLocation(String),
    Plasmid(String),
    Gene { gene: Vec<String> },
    ExpressionSystem(String),
    ExpressionSystemCommon { systems: Vec<String> },
    ExpressionSystemTaxId { id: Vec<u32> },
    ExpressionSystemStrain(String),
    ExpressionSystemVariant(String),
    ExpressionSystemCellLine(String),
    ExpressionSystemAtcc(u32),
    ExpressionSystemOrgan(String),
    ExpressionSystemTissue(String),
    ExpressionSystemCell(String),
    ExpressionSystemOrganelle(String),
    ExpressionSystemCellularLocation(String),
    ExpressionSystemVectorType(String),
    ExpressionSystemVector(String),
    ExpressionSystemPlasmid(String),
    ExpressionSystemGene(String),
}

/// Represents a modification made to this pdb entry.
#[derive(Debug, Clone)]
pub struct Revdat {
    pub modification_number: u32,
    pub modification_date: NaiveDate,
    pub idcode: String,
    pub modification_type: ModificationType,
    pub modification_detail: Vec<String>,
}

/// modification type of REVDAT record
#[derive(Debug, Clone)]
pub enum ModificationType {
    /// initial release of the entry. Indicated as 0
    /// in a REVDAT record
    InitialRelease,
    /// modifications other than initial release
    /// Indicated with 1 in a REVDAT record.
    OtherModification,
    /// modification type other than 0 or 1
    UnknownModification,
}

/// Serial Number Type of a JRNL REFN record
#[derive(Debug, Clone, PartialEq)]
pub enum SerialNumber {
    Issn,
    Essn,
}

/// contains HEADER recor information
#[derive(Debug, Clone)]
pub struct Header {
    pub classification: String,
    pub deposition_date: NaiveDate,
    pub id_code: String,
}

impl std::default::Default for Header {
    fn default() -> Self {
        Header {
            classification: String::default(),
            deposition_date: NaiveDate::from_ymd(1900, 1, 1),
            id_code: String::default(),
        }
    }
}

/// result of a TITLE record
#[derive(Debug, Clone, Default)]
pub struct Title {
    pub title: String,
}

/// contains pdb entry ids which removed
/// this one from PDB
#[derive(Debug, Clone)]
pub struct Obslte {
    pub replacement_date: NaiveDate,
    pub replacement_ids: Vec<String>,
}

impl std::default::Default for Obslte {
    fn default() -> Self {
        Obslte {
            replacement_date: NaiveDate::from_ymd(1900, 1, 1),
            replacement_ids: Vec::new(),
        }
    }
}

/// if this entry is a part of bigger
/// structure, this struct holds ids of other
/// parts of the bigger structure
#[derive(Debug, Clone, Default)]
pub struct Split {
    pub id_codes: Vec<String>,
}

/// fallacies of this entry
#[derive(Debug, Clone, Default)]
pub struct Caveat {
    pub id_code: String,
    pub comment: String,
}

/// pdb entry ids made obsolete by this entry
#[derive(Debug, Clone)]
pub struct Sprsde {
    pub sprsde_date: NaiveDate,
    pub id_code: String,
    pub superseeded: Vec<String>,
}

impl std::default::Default for Sprsde {
    fn default() -> Self {
        Sprsde {
            sprsde_date: NaiveDate::from_ymd(1900, 1, 1),
            superseeded: Vec::new(),
            id_code: String::default(),
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct Seqres {
    pub chain_id: Option<char>,
    pub residues: Vec<String>,
}

/// model type of the entry
#[derive(Debug, Clone, Default)]
pub struct Mdltyp {
    pub structural_annotation: Vec<String>,
}

/// collection of revisions
#[derive(Debug, Clone, Default)]
pub struct Revdats {
    pub revdat: Vec<Revdat>,
}

/// collection of tokens in a CMPND record
#[derive(Debug, Clone, Default)]
pub struct Cmpnd {
    pub tokens: Vec<Token>,
}

/// collection of tokens in a SOURCE record
#[derive(Debug, Clone, Default)]
pub struct Source {
    pub tokens: Vec<Token>,
}

/// keywords related to the entry
#[derive(Debug, Clone, Default)]
pub struct Keywds {
    pub keywords: Vec<String>,
}

/// journal author collection
#[derive(Debug, Clone, Default)]
pub struct JournalAuthors<'a> {
    pub authors: Vec<&'a str>,
}

/// journal title
#[derive(Debug, Clone, Default)]
pub struct JournalTitle {
    pub title: String,
}

/// journal editor collection
#[derive(Debug, Clone, Default)]
pub struct JournalEditors<'a> {
    pub name: Vec<&'a str>,
}

/// journal reference
#[derive(Debug, Clone, Default)]
pub struct JournalReference {
    pub publication_name: String,
    pub volume: Option<u32>,
    pub page: Option<u32>,
    pub year: Option<u32>,
}

/// journal Citation fields
#[derive(Debug, Clone, Default)]
pub struct JournalCitation {
    pub serial_type: Option<SerialNumber>,
    pub serial: Option<String>,
}

/// journal publication fields
#[derive(Debug, Clone, Default)]
pub struct JournalPublication {
    pub publication: String,
}

/// journal PubMed id
#[derive(Debug, Clone, Default)]
pub struct JournalPubMedId {
    pub id: u32,
}

/// digital object identifier of related e-pub
#[derive(Debug, Clone, Default)]
pub struct JournalDoi {
    pub id: String,
}

/// number of models in this file
#[derive(Debug, Clone, Default)]
pub struct Nummdl {
    pub num: u32,
}

/// cross references to other sequence databases
#[derive(Debug, Clone, Default)]
pub struct Dbref {
    pub idcode: String,
    pub chain_id: char,
    pub seq_begin: u32,
    pub initial_sequence: Option<char>,
    pub seq_end: u32,
    pub ending_sequence: Option<char>,
    pub database: String,
    pub db_accession: String,
    pub db_idcode: String,
    pub db_seq_begin: u32,
    pub idbns_begin: Option<char>,
    pub db_seq_end: u32,
    pub dbins_end: Option<char>,
}

pub(crate) fn parse_amino_acid(inp: &str) -> IResult<&str, AminoAcid> {
    map(take(3usize), AminoAcid::parse)(inp)
}
