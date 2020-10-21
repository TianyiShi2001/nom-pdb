#![feature(test)]

use criterion::{criterion_group, criterion_main, Criterion};

extern crate nom_pdb;
const PDB_7ZNF: &'static [u8] = include_bytes!("../assets/7znf.pdb"); // 6460 * 80 bytes (0.05168 MB)
                                                                      //use std::fs::read_to_string;

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("Read 7ZNF (0.05168 MB)", |b| {
        b.iter(|| {
            let _ = nom_pdb::Parser::parse(PDB_7ZNF);
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
