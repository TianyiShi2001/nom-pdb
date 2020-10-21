use nom_pdb::complete::Parser;
use serde_json;
use std::env;
use std::fs;

fn main() {
    let id = env::args().skip(1).next();
    match id {
        None => panic!("Please specify a filename!"),
        Some(id) => {
            let data = fs::read(&format!("assets/{}.pdb", id)).unwrap();
            let res = Parser::parse(&data).unwrap();
            let pretty = serde_json::to_string_pretty(&res).unwrap();
            println!("{}", pretty);
        }
    }
}
