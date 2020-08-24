use chrono::{
    format::{strftime::StrftimeItems, Parsed},
    NaiveDate,
};
use nom::{
    bytes::complete::{is_not, tag, take, take_while},
    character::complete::{anychar, char, multispace0},
    combinator::{map, map_res},
    error::ParseError,
    sequence::preceded,
    IResult,
};

// fn ws<'a, F: 'a, O, E: ParseError<&'a str>>(inner: F) -> impl Fn(&'a str) -> IResult<&'a str, O, E>
// where
//     F: Fn(&'a str) -> IResult<&'a str, O, E>,
// {
//     preceded(multispace0, &inner)(i)
// }

pub fn parse_right_f32<'a, 'b>(i: &'a str, length: usize) -> IResult<&'a str, f32> {
    let (i, s) = take_while(char_is_space)(i)?;
    let l = s.len();
    if l > length - 3 {
        panic!("Failed to parse float")
    }
    let (i, digit) = take(length - l)(i)?;
    let digit: f32 = digit.parse().unwrap();
    Ok((i, digit))
}
pub fn parse_right_u8<'a, 'b>(i: &'a str, length: usize) -> IResult<&'a str, u8> {
    let (i, s) = take_while(char_is_space)(i)?;
    let l = s.len();
    if l >= length {
        panic!("Failed to parse int")
    }
    let (i, digit) = take(length - l)(i)?;
    let digit: u8 = digit.parse().unwrap();
    Ok((i, digit))
}
pub fn parse_right_u32<'a, 'b>(i: &'a str, length: usize) -> IResult<&'a str, u32> {
    let (i, s) = take_while(char_is_space)(i)?;
    let l = s.len();
    if l >= length {
        panic!("Failed to parse int")
    }
    let (i, digit) = take(length - l)(i)?;
    let digit: u32 = digit.parse().unwrap();
    Ok((i, digit))
}

pub fn char_is_space(chr: char) -> bool {
    chr == ' '
}
