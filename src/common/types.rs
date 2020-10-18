// #[cfg(test)]
// mod tests {
//     use super::*;
//     #[test]
//     fn test_amino_acid_parse() {
//         let aa_s = vec!["SER", "IZO", "YOO", "LEU"];
//         let aa: Vec<AminoAcid> = aa_s.into_iter().map(AminoAcid::parse).collect();
//         assert_eq!(aa[0], AminoAcid::Ser);
//         assert_eq!(aa[1], AminoAcid::Nonstandard(NonstandardAminoAcid::Izo));
//         // assert_eq!(aa[1].description(), "(2S)-2-AMINOHEX-5-YNOIC ACID");
//         assert_eq!(aa[2], AminoAcid::Custom("YOO".to_string()));
//         assert_eq!(aa[3], AminoAcid::Leu);
//     }
// }
