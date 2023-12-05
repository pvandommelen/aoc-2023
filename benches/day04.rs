use aoc_2023::day::day04::*;
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use std::fs;
use std::path::Path;

pub fn criterion_benchmark(c: &mut Criterion) {
    let problem_name = Path::new(file!()).file_stem().unwrap();
    let input_filepath = format!("./input/{}.txt", problem_name.to_str().unwrap());
    let input = fs::read_to_string(input_filepath).expect("Unable to read input file");

    let mut group = c.benchmark_group(problem_name.to_str().unwrap());
    group.bench_function("solve", |b| b.iter(|| solve(black_box(&input))));

    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
