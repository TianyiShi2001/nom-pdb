use std::str::FromStr;
mod non_standard_aa;
pub use non_standard_aa::NonstandardAminoAcid;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub enum AminoAcid {
    Ala,
    Arg,
    Asn,
    Asp,
    Cys,
    Gln,
    Glu,
    Gly,
    His,
    Ile,
    Leu,
    Lys,
    Met,
    Phe,
    Pro,
    Ser,
    Thr,
    Trp,
    Tyr,
    Val,
    Mse,
    Pyl, // https://www.wwpdb.org/news/news?year=2014#5764490799cccf749a90cddf
    Sec, // https://www.wwpdb.org/news/news?year=2014#5764490799cccf749a90cddf
    Other(usize),
    Nonstandard(NonstandardAminoAcid),
    Custom(String),
}

impl FromStr for AminoAcid {
    type Err = String;
    fn from_str(inp: &str) -> Result<Self, <Self as FromStr>::Err> {
        match inp {
            "ALA" => Ok(Self::Ala),
            "ARG" => Ok(Self::Arg),
            "ASN" => Ok(Self::Asn),
            "ASP" => Ok(Self::Asp),
            "CYS" => Ok(Self::Cys),
            "GLN" => Ok(Self::Gln),
            "GLU" => Ok(Self::Glu),
            "GLY" => Ok(Self::Gly),
            "HIS" => Ok(Self::His),
            "ILE" => Ok(Self::Ile),
            "LEU" => Ok(Self::Leu),
            "LYS" => Ok(Self::Lys),
            "MET" => Ok(Self::Met),
            "PHE" => Ok(Self::Phe),
            "PRO" => Ok(Self::Pro),
            "SER" => Ok(Self::Ser),
            "THR" => Ok(Self::Thr),
            "TRP" => Ok(Self::Trp),
            "TYR" => Ok(Self::Tyr),
            "VAL" => Ok(Self::Val),
            "PYL" => Ok(Self::Pyl),
            "SEC" => Ok(Self::Sec),
            _ => Err("Not a standard amino acid!".to_string()),
        }
    }
}

impl AminoAcid {
    pub fn parse(inp: &str) -> Self {
        if let Ok(aa) = Self::from_str(inp) {
            aa
        } else if let Ok(aa) = NonstandardAminoAcid::from_str(inp) {
            Self::Nonstandard(aa)
        } else {
            Self::Custom(inp.to_owned())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_amino_acid_parse() {
        let aa_s = vec!["SER", "IZO", "YOO", "LEU"];
        let aa: Vec<AminoAcid> = aa_s.into_iter().map(AminoAcid::parse).collect();
        assert_eq!(aa[0], AminoAcid::Ser);
        assert_eq!(aa[1], AminoAcid::Nonstandard(NonstandardAminoAcid::Izo));
        // assert_eq!(aa[1].description(), "(2S)-2-AMINOHEX-5-YNOIC ACID");
        assert_eq!(aa[2], AminoAcid::Custom("YOO".to_string()));
        assert_eq!(aa[3], AminoAcid::Leu);
    }
}
