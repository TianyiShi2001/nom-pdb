pub mod anisou;
pub use anisou::AnisouParser;
pub mod atom;
pub mod conect;
pub mod hetatom;
pub use conect::ConectParser;

use crate::common::parser::{parse_residue, parse_right};

use crate::types::{
    AminoAcidAtomName, Atom, AtomName, AtomSerial, Element, NucleotideAtomName, Residue, Structure,
};
use nom::{bytes::complete::take, character::complete::anychar, combinator::map, IResult};

pub struct GenericAtomParser;

impl GenericAtomParser {
    fn parse<'a, 'b>(inp: &'a [u8], structure: &'b Structure) -> IResult<&'a [u8], Atom<'b>> {
        let (inp, id) = parse_right::<AtomSerial>(inp, 5)?;
        let (inp, _) = take(1usize)(inp)?;
        let (inp, name) = take(4usize)(inp)?;
        let (inp, id1) = anychar(inp)?;

        let (inp, residue) = parse_residue(inp, &structure)?;

        let name = match &residue {
            Residue::AminoAcid(_) => {
                AtomName::AminoAcid(AminoAcidAtomName::from_bytes_fixed4(name))
            }
            Residue::Nucleotide(_) => {
                AtomName::Nucleotide(NucleotideAtomName::from_bytes_uppercase_fixed4(name))
            }
            _ => AtomName::Other(unsafe { std::str::from_utf8_unchecked(name).to_owned() }),
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
        let (inp, element) = map(take(2usize), Element::from_bytes_uppercase_fixed2)(inp)?;
        let (inp, charge) = map(take(2usize), |x: &[u8]| match x {
            b"  " => 0,
            _ => {
                let x = unsafe { std::str::from_utf8_unchecked(x) };
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

// pub struct AtomParser;

// impl FieldParser for AtomParser {
//     type Output = Atom;
//     fn parse(inp: &[u8]) -> IResult<&[u8], Atom> {
//         GenericAtomParser::parse(inp, false)
//     }
// }
