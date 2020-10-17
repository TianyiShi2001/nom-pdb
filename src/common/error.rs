// use nom::error::ErrorKind;
// use nom::error::ParseError;
// use nom::Err::Error;
// use nom::IResult;

// #[derive(Debug, PartialEq)]
// pub enum PdbParseError<I> {
//     Ascii,
//     UnknownField,
//     Other,
//     Nom(I, ErrorKind),
// }

// impl<I> ParseError<I> for PdbParseError<I> {
//     fn from_error_kind(input: I, kind: ErrorKind) -> Self {
//         PdbParseError::Nom(input, kind)
//     }

//     fn append(_: I, _: ErrorKind, other: Self) -> Self {
//         other
//     }
// }

// fn parse(input: &str) -> IResult<&str, &str, PdbParseError<&str>> {
//     Err(Error(PdbParseError::Other))
// }

// #[cfg(test)]
// mod tests {
//     use super::parse;
//     use super::PdbParseError;
//     use nom::Err::Error;

//     #[test]
//     fn it_works() {
//         let err = parse("").unwrap_err();
//         match err {
//             Error(e) => assert_eq!(e, PdbParseError::Other),
//             _ => panic!("Unexpected error: {:?}", err),
//         }
//     }
// }
