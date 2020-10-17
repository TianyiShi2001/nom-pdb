/// A simple single-thread parser.
use crate::{coordinate::*, crystallography::*, primary_structure::*, title_section::*};

// use crate::common::error::PdbParseError;
use crate::common::parser::FieldParser;
use nom::bytes::complete::take;
use nom::character::complete::{line_ending, not_line_ending};
use nom::IResult;
// use nom::Err::Error;

#[derive(Debug, Clone, Default)]
pub struct Pdb {
    pub header: Header,
    pub title: Title,
    pub authors: Authors,
    pub experimental_techniques: ExperimentalTechniques,
    pub cryst1: Cryst1,
    pub seqres: SeqRes,
    pub atoms: Vec<Atom>,
}

impl Pdb {}

/// http://www.wwpdb.org/documentation/file-format-content/format33/sect1.html
#[derive(Eq, PartialEq, Debug, Ord, PartialOrd)]
enum Fields {
    Header,  // M
    Obslte,  // O : Mandatory in entries that have been replaced by a newer entry.
    Title,   // M
    Split, //   O : Mandatory when  large macromolecular complexes are split into multiple PDB entries.
    Caveat, // O : Mandatory when there are outstanding errors such as chirality.
    Compnd, // M
    Source, // M
    Keywds, // M
    Expdta, // M
    Nummdl, // O : Mandatory for  NMR ensemble entries.
    Mdltyp, // O : Mandatory for  NMR minimized average Structures or when the entire  polymer chain contains C alpha or P atoms only.
    Author, // M
    Revdat, // M
    Sprsde, // O : Mandatory for a replacement entry.
    Jrnl,   //   O: Mandatory for a publication describes the experiment.
    Remark0, // O : Mandatory for a re-refined structure
    Remark1, // O
    Remark2, // M
    Remark3, // M
    RemarkN, // O : Mandatory under certain conditions
    Dbref,  //  O : Mandatory for all polymers.
    Dbref1,
    Dbref2, // O : Mandatory when certain sequence database accession and/or sequence numbering does not fit preceding DBREF format.
    SeqAdv, // O : Mandatory if sequence  conflict exists.
    SeqRes, // O : Mandatory if ATOM records exist.
    Modres, // O : Mandatory if modified group exists in the coordinates.
    Het,    // O : Mandatory if a non-standard group other than water appears in the coordinates.
    Hetnam, // O : Mandatory if a non-standard group otherthan  water appears in the coordinates.
    Hetsyn, // O
    Formul, // O : Mandatory if a non-standard group or water appears in the coordinates.
    Helix,  // O
    Sheet,  // O
    Ssbond, // O : Mandatory if a  disulfide bond is present.
    Link,   // O : Mandatory if  non-standard residues appear in a polymer
    Cispep, // O
    Site,   // O
    Cryst1, // M
    Origx1, // M
    Origx2, // M
    Origx3, // M
    Scale1, // M
    Scale2, // M
    Scale3, // M
    Mtrix1, // O Mandatory if  the complete asymmetric unit
    Mtrix2, // O must  be generated from the given coordinates
    Mtrix3, // O using non-crystallographic symmetry.
    Model,  // O : Mandatory if more than one model is present in the entry.
    Atom,   // O : Mandatory if standard residues exist.
    Anisou, // O
    Ter,    // O : Mandatory if ATOM records exist.
    Hetatm, // O : Mandatory if non-standard group exists.
    Endmdl, // O : Mandatory if MODEL appears.
    Conect, // O : Mandatory if non-standard group appears and if LINK or SSBOND records exist.
    Master, // M
    End,    // M
}

#[derive(Eq, PartialEq, Debug, Ord, PartialOrd)]
enum Section {
    Title, // HEADER,  OBSLTE, TITLE, SPLIT, CAVEAT, COMPND, SOURCE, KEYWDS, EXPDTA, NUMMDL, MDLTYP, AUTHOR, REVDAT, PRSDE, JRNL
    Remark, // REMARKs  0-999
    PrimaryStructure, // DBREF, SEQADV, SEQRES MODRES
    Heterogen, // HET, HETNAM, HETSYN, FORMUL
    SecondaryStructure, // HELIX, SHEET
    Connectivity, // CONECT, SSBOND, LINK, CISPEP
    Misc,  // SITE
    Crystallography, // CRYST1
    CoordinateTransformation, // ORIGXn,  SCALEn, MTRIXn,
    Coordinate, // MODEL, ATOM, ANISOU, TER, HETATM, ENDMDL
}
/// Commas, colons, and semi-colons are used as list delimiters in records that have one of the following data types:
///
/// - List
/// - SList
/// - Specification List
/// - Specification
///
/// If a comma, colon, or semi-colon is used in any context other than as a delimiting character, then the character must be escaped, i.e., immediately preceded by a backslash, "\".
enum Dtype {
    // To interpret a String, concatenate the contents of all continued fields together, collapse all sequences of multiple blanks to a single blank, and remove any leading and trailing blanks. This permits very long strings to be properly reconstructed.
    List,              // A String that is composed of text separated with commas.
    SList,             // A String that is composed of text separated with semi-colons.
    Specification, // A String composed of a token and its  associated value separated by a colon.
    SpecificationList, // A sequence of Specifications, separated by semi-colons.
    String,
    Oneline,
}

enum ParserState {
    FirstLine,
    Continue,
}

pub struct Parser<'a> {
    state: ParserState,
    remaining: &'a str,
    buffer: String,
}

impl<'a> Parser<'a> {
    pub fn parse(mut inp: &str) -> nom::IResult<&str, Pdb> {
        let mut pdb = Pdb::default();
        loop {
            let (i, tag) = take(6usize)(inp)?;
            inp = match tag {
                "HEADER" => HeaderParser::parse_into(&i, &mut pdb.header),
                "TITLE " => TitleParser::parse_into(&i, &mut pdb.title),
                "AUTHOR" => AuthorsParser::parse_into(&i, &mut pdb.authors),
                "CRYST1" => Cryst1Parser::parse_into(&i, &mut pdb.cryst1),
                "SEQRES" => SeqResParser::parse_into(inp, &mut pdb.seqres),
                "EXPDTA" => {
                    ExperimentalTechniquesParser::parse_into(&i, &mut pdb.experimental_techniques)
                }
                "ATOM  " => AtomParser::parse_into_vec(&i, &mut pdb.atoms),
                "END   " => {
                    inp = "";
                    break;
                }
                _ => {
                    // new line
                    let (i, _) = not_line_ending(i)?;
                    let (i, _) = line_ending(i)?;
                    i
                } //panic!("Unkown field"),
            }
        }
        Ok((inp, pdb))
    }
}
