pub mod anisou;
pub use anisou::{Anisou, AnisouParser};
pub mod atom;
pub use atom::AtomParser;
pub mod hetatom;
pub use hetatom::HetAtomParser;

use crate::common::{
    parser::{parse_amino_acid, parse_right},
    types::AminoAcid,
};

use nom::{bytes::complete::take, character::complete::anychar, combinator::map, IResult};
use serde::{Deserialize, Serialize};
use std::str::FromStr;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Model {
    pub atoms: Vec<Atom>,
    pub anisou: Vec<Anisou>,
}

pub type Models = Vec<Model>;

#[derive(Debug, Clone, Serialize, Deserialize)]
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
    pub hetatom: bool,
}
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AminoAcidAtomName {
    N,
    CA,
    C,
    O,
    Other(String), // TODO: a 'lossy' version?
}

impl FromStr for AminoAcidAtomName {
    type Err = String;
    fn from_str(inp: &str) -> std::result::Result<Self, <Self as std::str::FromStr>::Err> {
        match inp {
            "C" => Ok(AminoAcidAtomName::C),
            "CA" => Ok(AminoAcidAtomName::CA),
            "O" => Ok(AminoAcidAtomName::O),
            "N" => Ok(AminoAcidAtomName::N),
            _ => Ok(AminoAcidAtomName::Other(inp.to_owned())),
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
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
    Se,
    V,
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
            "Se" => Ok(Element::Se),
            "V" => Ok(Element::V),
            _ => Err(format!("Unknown atom name {}", inp)),
        }
    }
}
