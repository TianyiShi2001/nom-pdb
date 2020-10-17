use nom_pdb::complete::Parser;
use serde::{Deserialize, Serialize};
use serde_json;
use std::env;
use std::fs;
use std::str::from_utf8_unchecked;

fn main() {
    let id = env::args().skip(1).next();
    match id {
        None => panic!("Please specify a filename!"),
        Some(id) => match id.as_str() {
            "1a8o" | "7znf" => {
                let data = fs::read(&format!("assets/{}.pdb", id)).unwrap();
                unsafe {
                    let data = from_utf8_unchecked(&data);
                    let (_, res) = Parser::parse(data).unwrap();
                    // println!("{:?}, {:?}", data, r);
                    let pretty = serde_json::to_string_pretty(&res).unwrap();
                    println!("{}", pretty);
                }
            }
            _ => panic!(format!(
                "{} is not a sample file in the `assets/` directory.",
                id
            )),
        },
    }
}
