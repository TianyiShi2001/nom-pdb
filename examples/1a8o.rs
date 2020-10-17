use nom_pdb::complete::Parser;
use std::str::from_utf8_unchecked;

fn main() {
    let data = include_bytes!("../assets/1A8O.pdb");
    unsafe {
        let data = from_utf8_unchecked(data);
        let (data, r) = Parser::parse(data).unwrap();
        println!("{:?}, {:?}", data, r);
    }
}
