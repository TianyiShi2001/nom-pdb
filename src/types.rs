pub use protein_core::metadata::*;
pub use protein_core::structure::*;
use std::collections::HashMap;

pub(crate) type ModifiedAminoAcidTable = HashMap<String, ModifiedAminoAcid>;
pub(crate) type ModifiedNucleotideTable = HashMap<String, ModifiedNucleotide>;

pub trait PdbAminoAcid {
    fn from_uppercase(inp: &str) -> Self;
    // if let Some(aa) = StandardAminoAcid::from_uppercase(inp) {
    //     Self::Standard(aa)
    // } else {
    //     Self::Modified(inp.to_owned())
    // }
    fn from_bytes_uppercase(inp: &[u8]) -> Self;
    // if let Some(aa) = StandardAminoAcid::from_bytes_uppercase(inp) {
    //     Self::Standard(aa)
    // } else {
    //     Self::Modified(unsafe { String::from_utf8_unchecked(inp.to_owned()) })
    // }
}

pub(crate) trait ParseFw2 {
    fn parse_fw2(inp: &[u8]) -> Self;
}

pub(crate) trait ParseFw3 {
    fn parse_fw3(inp: &[u8]) -> Self;
}

pub(crate) trait ParseFw4 {
    fn parse_fw4(inp: &[u8]) -> Self;
}

pub(crate) trait TryParseFw2
where
    Self: Sized,
{
    fn try_parse_fw2(inp: &[u8]) -> Option<Self>;
}

pub(crate) trait TryParseFw3
where
    Self: Sized,
{
    fn try_parse_fw3(inp: &[u8]) -> Option<Self>;
}

pub(crate) trait TryParseFw4
where
    Self: Sized,
{
    fn try_parse_fw4(inp: &[u8]) -> Option<Self>;
}

impl ParseFw2 for Element {
    fn parse_fw2(inp: &[u8]) -> Self {
        match inp {
            b" H" => Self::H,
            b" C" => Self::C,
            b" O" => Self::O,
            b" N" => Self::N,
            b" P" => Self::P,
            b" S" => Self::S,
            b"SE" => Self::Se,
            b"NA" => Self::Na,
            b"MG" => Self::Mg,
            b"CL" => Self::Cl,
            b" K" => Self::K,
            b"CA" => Self::Ca,
            b"FE" => Self::Fe,
            b"MN" => Self::Mn,
            b"CO" => Self::Co,
            b"CR" => Self::Cr,
            b" I" => Self::I,
            b"ZN" => Self::Zn,
            b"CU" => Self::Cu,
            b" F" => Self::F,
            b"AL" => Self::Al,
            b" V" => Self::V,
            b"X1" => Self::Unknown,
            _ => panic!(format!(
                "fail to parse element: {}",
                std::str::from_utf8(inp).unwrap()
            )),
        }
    }
}

impl TryParseFw3 for StandardAminoAcid {
    fn try_parse_fw3(inp: &[u8]) -> Option<Self> {
        match inp {
            b"ALA" => Some(Self::Ala),
            b"ARG" => Some(Self::Arg),
            b"ASN" => Some(Self::Asn),
            b"ASP" => Some(Self::Asp),
            b"CYS" => Some(Self::Cys),
            b"GLN" => Some(Self::Gln),
            b"GLU" => Some(Self::Glu),
            b"GLY" => Some(Self::Gly),
            b"HIS" => Some(Self::His),
            b"ILE" => Some(Self::Ile),
            b"LEU" => Some(Self::Leu),
            b"LYS" => Some(Self::Lys),
            b"MET" => Some(Self::Met),
            b"PHE" => Some(Self::Phe),
            b"PRO" => Some(Self::Pro),
            b"SER" => Some(Self::Ser),
            b"THR" => Some(Self::Thr),
            b"TRP" => Some(Self::Trp),
            b"TYR" => Some(Self::Tyr),
            b"VAL" => Some(Self::Val),
            b"PYL" => Some(Self::Pyl),
            b"SEC" => Some(Self::Sec),
            _ => None,
        }
    }
}

impl TryParseFw3 for StandardNucleotide {
    fn try_parse_fw3(inp: &[u8]) -> Option<Self> {
        match inp {
            b"  A" => Some(Self::A),
            b"  C" => Some(Self::C),
            b"  G" => Some(Self::G),
            b"  U" => Some(Self::U),
            b" DA" => Some(Self::DA),
            b" DC" => Some(Self::DC),
            b" DG" => Some(Self::DG),
            b" DT" => Some(Self::DT),
            _ => None,
        }
    }
}

impl ParseFw3 for AminoAcid {
    fn parse_fw3(inp: &[u8]) -> Self {
        if let Some(aa) = StandardAminoAcid::try_parse_fw3(inp) {
            Self::Standard(aa)
        } else if inp == b"UNK" {
            Self::Unknown
        } else {
            Self::Modified(unsafe { String::from_utf8_unchecked(inp.to_owned()) })
        }
    }
}

impl ParseFw3 for Nucleotide {
    fn parse_fw3(inp: &[u8]) -> Self {
        if let Some(nuc) = StandardNucleotide::try_parse_fw3(inp) {
            Self::Standard(nuc)
        } else if inp == b"  N" {
            Self::Unknown
        } else {
            Self::Modified(unsafe { String::from_utf8_unchecked(inp.to_owned()) })
        }
    }
}

impl ParseFw4 for AminoAcidAtomName {
    fn parse_fw4(inp: &[u8]) -> Self {
        match inp {
            b" N  " => Self::N,
            b" CA " => Self::CA,
            b" C  " => Self::C,
            b" O  " => Self::O,
            _ => Self::Other(unsafe { std::str::from_utf8_unchecked(inp).trim_start().to_owned() }),
        }
    }
}

impl ParseFw4 for NucleotideAtomName {
    fn parse_fw4(inp: &[u8]) -> Self {
        match inp {
            b" OP1" => Self::OP1,
            b" OP2" => Self::OP2,
            b" O5 " => Self::O5,
            b" O4 " => Self::O4,
            b" O3 " => Self::O3,
            b" O2 " => Self::O2,
            b" C5 " => Self::C5,
            b" C4 " => Self::C4,
            b" C3 " => Self::C3,
            b" C2 " => Self::C2,
            b" C1 " => Self::C1,
            b" N9 " => Self::N9,
            b" N7 " => Self::N7,
            b" N6 " => Self::N6,
            b" N4 " => Self::N4,
            b" N3 " => Self::N3,
            b" N2 " => Self::N2,
            b" N1 " => Self::N1,
            b" P  " => Self::P,
            _ => Self::Other(unsafe { std::str::from_utf8_unchecked(inp).trim_start().to_owned() }),
        }
    }
}
