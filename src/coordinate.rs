pub mod anisou;
pub use anisou::AnisouParser;
pub mod atom;
pub use atom::AtomParser;
pub mod hetatom;
pub use hetatom::HetAtomParser;

use crate::common::parser::{parse_amino_acid, parse_right};

use nom::{bytes::complete::take, character::complete::anychar, combinator::map, IResult};
use protein_core::types::{
    atom::{AminoAcidAtomName, Atom},
    element::Element,
};

use std::str::FromStr;

pub struct GenericAtomParser;

impl GenericAtomParser {
    fn parse(inp: &str, hetatom: bool) -> IResult<&str, Atom> {
        let (inp, id) = parse_right::<u32>(inp, 5)?;
        let (inp, _) = take(1usize)(inp)?;
        let (inp, name) = map(map(take(4usize), str::trim), |x| {
            AminoAcidAtomName::from_str(x).unwrap()
        })(inp)?;
        let (inp, id1) = anychar(inp)?;
        let (inp, residue) = parse_amino_acid(inp)?;
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
                id1,
                name,
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
                hetatom,
            },
        ))
    }
}
