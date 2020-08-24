/// Parses AUTHOR record which is a multiline continuation record.
/// Contains comma-seperated list of author names. If successfull
/// returns [Record](../ast/types/enum.Record.html) variant containing
/// [AUTHORS](../ast/types/struct.Authors.html) instance.
/// Record structure :
/// | COLUMNS | DATA  TYPE   | FIELD        | DEFINITION                                   |
/// |---------|--------------|--------------|----------------------------------------------|
/// | 1 -  6  | Record name  | AUTHOR       |                                              |
/// | 9 - 10  | Continuation | continuation | Allows concatenation of multiple records.    |
/// | 11 - 79 | List         | authorList   | List of the author names, separated          |
/// |         |              |              | by commas.                                   |
use nom::{
    bytes::complete::{is_not, take, take_while},
    character::complete::{anychar, char, line_ending, multispace1},
    combinator::peek,
    IResult,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Authors<'a> {
    pub names: Vec<&'a str>,
}
pub fn parse_authors(inp: &str) -> IResult<&str, Authors> {
    // ! need improvement
    let (mut inp, _) = take(4usize)(inp)?; // 7 - 10
    let mut names: Vec<&str> = Vec::new();
    loop {
        let (i, name) = take_while(|c| c != ',' && c != '\n' && c != '\r')(inp)?;
        names.push(name.trim());
        let (i, comma_or_newline) = anychar(i)?; // consume \r or \n if newline
        if comma_or_newline == ',' {
            let (i, char_after_comma) = peek(anychar)(i)?;
            if char_after_comma == ' ' {
                // newline
                inp = multispace1(i)?.0;
                inp = take(10usize)(inp)?.0; // 1 - 10
            } else {
                inp = i;
            }
        } else {
            // end
            inp = take_while(|x| x == '\n')(i)?.0;
            return Ok((inp, Authors { names }));
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_parse_authors() {
        let (i, r) = parse_authors(
            "    T.R.GAMBLE,S.YOO,F.F.VAJDOS,U.K.VON SCHWEDLER,                        
AUTHOR   2 D.K.WORTHYLAKE,H.WANG,J.P.MCCUTCHEON,W.I.SUNDQUIST,C.P.HILL          
REVDAT   5   03-NOV-09 1A8O    1       SEQADV                                   ",
        )
        .unwrap();
        assert_eq!(
            i,
            "REVDAT   5   03-NOV-09 1A8O    1       SEQADV                                   "
        );
        assert_eq!(
            r,
            Authors {
                names: vec![
                    "T.R.GAMBLE",
                    "S.YOO",
                    "F.F.VAJDOS",
                    "U.K.VON SCHWEDLER",
                    "D.K.WORTHYLAKE",
                    "H.WANG",
                    "J.P.MCCUTCHEON",
                    "W.I.SUNDQUIST",
                    "C.P.HILL"
                ]
            }
        )
    }
}
