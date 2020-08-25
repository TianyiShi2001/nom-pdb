/// Parses EXPDTA records which is a continuation type of record which may span multi-lines.
/// Record contains list of `;` seperated experimental techniques. If seuccesfull returns
/// [Record](../ast/types/enum.Record.html) variant containing [ExperimentalTechniques](../ast/types/struct.Experimental.html)
/// # Record structure:
/// | COLUMNS | DATA TYPE     | FIELD        | DEFINITION                                |
/// |---------|---------------|--------------|-------------------------------------------|
/// | 1 -  6  | Record name   | EXPDTA       |                                           |
/// | 9 - 10  | Continuation  | continuation | Allows concatenation of multiple records. |
/// | 11 - 79 | SList         | technique    | The experimental technique(s) with        |
/// |         |                              | optional comment desc                     |
use crate::common::parser::{parse_multiline_list, FieldParserComplete};
pub type ExperimentalTechniques = Vec<ExperimentalTechnique>;
pub struct ExperimentalTechniquesParser;
impl FieldParserComplete for ExperimentalTechniquesParser {
    type Output = Vec<ExperimentalTechnique>;
    fn parse(inp: &str) -> nom::IResult<&str, ExperimentalTechniques> {
        let (inp, techniques_as_str) = parse_multiline_list(inp)?;
        let techniques: Vec<ExperimentalTechnique> = techniques_as_str
            .into_iter()
            .map(|s| {
                s.parse::<ExperimentalTechnique>()
                    .expect("Failed to parse experimental techniques")
            })
            .collect();
        Ok((inp, techniques))
    }
}

#[derive(Debug, Clone)]
pub enum ExperimentalTechnique {
    XRayDiffraction,
    ElectronMicroscopy,
    SolidStateNmr,
    SolutionNmr,
    NeutronDiffraction,
    ElectronCrystallography,
    SolutionScattering,
    FiberDiffraction,
}

impl std::str::FromStr for ExperimentalTechnique {
    type Err = String;
    fn from_str(inp: &str) -> std::result::Result<Self, <Self as std::str::FromStr>::Err> {
        match inp {
            "X-RAY DIFFRACTION" => Ok(ExperimentalTechnique::XRayDiffraction),
            "ELECTRON MICROSCOPY" => Ok(ExperimentalTechnique::ElectronMicroscopy),
            "SOLID-STATE NMR" => Ok(ExperimentalTechnique::SolidStateNmr),
            "SOLUTION NMR" => Ok(ExperimentalTechnique::SolutionNmr),
            "NEUTRON DIFFRACTION" => Ok(ExperimentalTechnique::NeutronDiffraction),
            "ELECTRON CRYSTALLOGRAPHY" => Ok(ExperimentalTechnique::ElectronCrystallography),
            "SOLUTION SCATTERING" => Ok(ExperimentalTechnique::SolutionScattering),
            "FIBER DIFFRACTION" => Ok(ExperimentalTechnique::FiberDiffraction),
            _ => Err(format!("Unknown experimental result {}", inp)),
        }
    }
}
