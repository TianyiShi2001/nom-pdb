#[derive(Debug, Clone)]
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
}
use AminoAcid::*;
impl std::str::FromStr for AminoAcid {
    type Err = String;
    fn from_str(inp: &str) -> std::result::Result<Self, <Self as std::str::FromStr>::Err> {
        match inp {
            "ALA" => Ok(Ala),
            "ARG" => Ok(Arg),
            "ASN" => Ok(Asn),
            "ASP" => Ok(Asp),
            "CYS" => Ok(Cys),
            "GLN" => Ok(Gln),
            "GLU" => Ok(Glu),
            "GLY" => Ok(Gly),
            "HIS" => Ok(His),
            "ILE" => Ok(Ile),
            "LEU" => Ok(Leu),
            "LYS" => Ok(Lys),
            "MET" => Ok(Met),
            "PHE" => Ok(Phe),
            "PRO" => Ok(Pro),
            "SER" => Ok(Ser),
            "THR" => Ok(Thr),
            "TRP" => Ok(Trp),
            "TYR" => Ok(Tyr),
            "VAL" => Ok(Val),
            "MSE" => Ok(Mse),
            _ => Err(format!("Unknown amino acid {}", inp)),
        }
    }
}
