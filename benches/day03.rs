use aoc_2023::day::day03::*;
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use std::fs;
use std::path::Path;

pub fn criterion_benchmark(c: &mut Criterion) {
    let problem_name = Path::new(file!()).file_stem().unwrap();
    let input_filepath = format!("./input/{}.txt", problem_name.to_str().unwrap());
    let input = fs::read_to_string(input_filepath).expect("Unable to read input file");

    let prepared_input = prepare(&input);

    let mut group = c.benchmark_group(problem_name.to_str().unwrap());
    group.bench_function("parse", |b| b.iter(|| prepare(black_box(&input))));
    group.bench_function("part1", |b| {
        b.iter(|| solve_part1(black_box(&prepared_input)))
    });
    group.bench_function("part2", |b| {
        b.iter(|| solve_part2(black_box(&prepared_input)))
    });

    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
