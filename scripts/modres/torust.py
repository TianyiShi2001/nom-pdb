import os
import json

STANDARD_AA = ["ALA",
               "ARG",
               "ASN",
               "ASP",
               "CYS",
               "GLN",
               "GLU",
               "GLY",
               "HIS",
               "ILE",
               "LEU",
               "LYS",
               "MET",
               "PHE",
               "PRO",
               "SER",
               "THR",
               "TRP",
               "TYR",
               "VAL",
               "PYL",
               "SEC"]

MODRES = os.path.join(
    os.path.dirname(__file__), "modres.json")

MODRES_OUT = os.path.join(
    os.path.dirname(__file__), "modres.rs")

with open(MODRES) as f:
    modres = json.load(f)


enum_s = ""
std_s = ""
desc_s = ""
from_str_s = ""
for k, v in modres.items():
    if v[0] and v[0][0] == " ":
        # nucleotide
        continue
    if v[2] < 5:
        # threshold
        continue
    if not k[0].isalpha():
        continue

    uc = k
    tc = uc.title()
    [std, desc, n] = v

    if not std in STANDARD_AA:
        continue

    enum_s += "    " + tc + ",\n"
    std_s += "            Self::" + tc + " => "
    if std:
        std_s += f"Some(AminoAcid::{std.title()}),\n"
    else:
        std_s += "None,\n"
    desc_s += f'            Self::{tc} => "{desc}",\n'
    from_str_s += f'            "{uc}" => Ok(Self::{tc}),\n'


s = """use protein_core::types::AminoAcid;
use std::str::FromStr;
use serde::{Deserialize, Serialize};
"""

s += """#[derive(Debug, Clone, Eq, PartialEq, Deserialize, Serialize)]
pub enum NonstandardAminoAcid {""" + enum_s + "\n}"

s += """impl NonstandardAminoAcid {
    pub fn standard_res(&self) -> Option<AminoAcid> {
        match &self {""" + std_s + """\
        }
    }
    pub fn description(&self) -> &'static str {
        match &self {""" + desc_s + """\
        }
    }
}
"""

s += """impl FromStr for NonstandardAminoAcid {
    type Err = String;
    fn from_str(inp: & str) -> Result<Self,<Self as FromStr >::Err> {
        match inp {""" + from_str_s + """\
            _ => Err("not a known non-standard amino acid".to_owned()),
        }
    }
}
"""


with open(MODRES_OUT, "w") as f:
    f.write(s)
