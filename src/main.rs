use clap::Parser;
use std::time::Instant;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Day
    #[arg(short, long)]
    day: u8,
}

fn main() {
    let args = Args::parse();

    let solver = match args.day {
        1 => aoc_2023::day::day01::solve,
        2 => aoc_2023::day::day02::solve,
        3 => aoc_2023::day::day03::solve,
        4 => aoc_2023::day::day04::solve,
        _ => unimplemented!(),
    };

    let input = std::fs::read_to_string(format!("./input/day{:0>2}.txt", args.day)).unwrap();

    let start = Instant::now();

    let (p1, p2) = solver(&input);

    let end = Instant::now();

    println!("day{}/part1: {}", args.day, p1);
    println!("day{}/part2: {}", args.day, p2);
    println!(
        "Execution time: {:.2} ms",
        ((end - start).as_micros() as f64) / 1000f64
    );
}
