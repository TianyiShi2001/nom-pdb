use nom_pdb::complete::Parser;
use serde_json;
use std::fs;

#[test]
fn main() {
    let entries = fs::read_dir("assets").unwrap().map(|f| f.unwrap().path());
    for f in entries {
        let data = fs::read(f).unwrap();

        let (_, res) = Parser::parse(&data).unwrap();
        let pretty = serde_json::to_string_pretty(&res).unwrap();
        println!("{}", pretty);
    }
}
