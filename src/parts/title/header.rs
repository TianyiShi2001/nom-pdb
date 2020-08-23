/// Parsing the [Header](http://www.wwpdb.org/documentation/file-format-content/format33/sect2.html#HEADER)
use chrono::{
    format::{strftime::StrftimeItems, Parsed},
    NaiveDate,
};
use nom::{
    bytes::complete::take,
    character::complete::{line_ending, multispace1, space0},
    combinator::{map, map_res},
    IResult,
};

fn parse_month(i: &str) -> IResult<&str, u32> {
    map_res(take(3usize), |s: &str| -> Result<u32, ()> {
        let mut parsed = Parsed::new();
        chrono::format::parse(&mut parsed, s, StrftimeItems::new("%b"))
            .expect("Failed to parse month");
        Ok(parsed.month.unwrap())
    })(i)
}
pub fn parse_date(i: &str) -> IResult<&str, NaiveDate> {
    let (i, day) = take(2usize)(i)?;
    let (i, _) = take(1usize)(i)?;
    let (i, month) = parse_month(i)?;
    let (i, _) = take(1usize)(i)?;
    let (i, year) = take(2usize)(i)?;
    let mut year = year.parse::<i32>().unwrap();
    if year < 50i32 {
        year += 2000
    } else {
        year += 1900
    }
    Ok((
        i,
        NaiveDate::from_ymd(year, month, day.parse::<u32>().unwrap()),
    ))
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Header<'a> {
    ///     The HEADER record uniquely identifies a PDB entry through the idCode field.
    /// This record also provides a classification for the entry. Finally, it contains
    /// the date when the coordinates were deposited to the PDB archive.
    /// # Record Format
    /// COLUMNS       DATA  TYPE     FIELD             DEFINITION
    /// ------------------------------------------------------------------------------------
    ///  1 -  6       Record name    "HEADER"
    /// 11 - 50       String(40)     classification    Classifies the molecule(s).
    /// 51 - 59       Date           depDate           Deposition date. This is the date the
    ///                                                coordinates  were received at the PDB.
    /// 63 - 66       IDcode         idCode            This identifier is unique within the PDB.
    pub classification: &'a str,
    pub deposition_date: NaiveDate,
    pub id_code: &'a str,
}
fn parse_header(i: &str) -> IResult<&str, Header> {
    let (i, _) = take(4usize)(i)?;
    let (i, classification) = map(take(40usize), str::trim)(i)?;
    let (i, deposition_date) = parse_date(i)?;
    let (i, _) = take(3usize)(i)?;
    let (i, id_code) = take(4usize)(i)?;
    let (i, _) = multispace1(i)?;
    Ok((
        i,
        Header {
            classification,
            deposition_date,
            id_code,
        },
    ))
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_parse_header() {
        let i = "    VIRAL PROTEIN                           27-MAR-98   1A8O              \nTITLE     HIV CAPSID C-TERMINAL DOMAIN                                          ";
        let (i, r) = parse_header(i).unwrap();
        assert_eq!(
            i.to_owned(),
            "TITLE     HIV CAPSID C-TERMINAL DOMAIN                                          "
                .to_owned()
        );
        assert_eq!(
            r,
            Header {
                classification: "VIRAL PROTEIN",
                deposition_date: NaiveDate::from_ymd(1998i32, 3u32, 27u32),
                id_code: "1A8O"
            }
        )
    }
}
