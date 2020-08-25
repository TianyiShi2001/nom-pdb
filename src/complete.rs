/// A simple single-thread parser.
use crate::{coordinate::*, crystallography::*, primary_structure::*, title_section::*};

// use crate::common::error::PdbParseError;
use crate::common::parser::FieldParserComplete;
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

fn parse_single<'a, 'b, P: FieldParserComplete>(inp: &'a str, field: &'b mut P::Output) -> &'a str {
    let (i, data) = P::parse(inp).expect("parse error");
    *field = data;
    i
}
fn parse_into_vec<'a, 'b, P: FieldParserComplete>(
    inp: &'a str,
    field: &'b mut Vec<P::Output>,
) -> &'a str {
    let (i, data) = P::parse(inp).expect("parse error");
    field.push(data);
    i
}

impl Pdb {
    pub fn parse(mut inp: &str) -> nom::IResult<&str, Pdb> {
        let mut pdb = Pdb::default();
        loop {
            let (i, tag) = take(6usize)(inp)?;
            inp = match tag {
                "HEADER" => parse_single::<HeaderParserComplete>(&i, &mut pdb.header),
                "TITLE " => parse_single::<TitleParserComplete>(&i, &mut pdb.title),
                "AUTHOR" => parse_single::<AuthorsParserComplete>(&i, &mut pdb.authors),
                "CRYST1" => parse_single::<Cryst1ParserComplete>(&i, &mut pdb.cryst1),
                "SEQRES" => parse_single::<SeqResParserComplete>(&i, &mut pdb.seqres),
                "EXPDTA" => parse_single::<ExperimentalTechniquesParser>(
                    &i,
                    &mut pdb.experimental_techniques,
                ),
                "ATOM  " => parse_into_vec::<AtomParserComplete>(&i, &mut pdb.atoms),
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
