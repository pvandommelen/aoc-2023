use clap::Parser;
use std::time::Instant;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Day
    #[arg(short, long)]
    day: Option<usize>,
}

fn read_input(day: usize) -> String {
    std::fs::read_to_string(format!("./input/day{:0>2}.txt", day)).unwrap()
}

fn main() {
    let args = Args::parse();

    let all_days = [
        aoc_2023::day::day01::solve,
        aoc_2023::day::day02::solve,
        aoc_2023::day::day03::solve,
        aoc_2023::day::day04::solve,
        aoc_2023::day::day05::solve,
        aoc_2023::day::day06::solve,
        aoc_2023::day::day07::solve,
        aoc_2023::day::day08::solve,
    ];

    let day_and_solver: Vec<_> = match args.day {
        None => all_days
            .into_iter()
            .enumerate()
            .map(|(i, solve)| (i + 1, solve))
            .collect(),
        Some(d) => vec![(d, all_days[d - 1])],
    };

    let start = Instant::now();
    day_and_solver.into_iter().for_each(|(day, solver)| {
        let input = read_input(day);

        let start = Instant::now();
        let (p1, p2) = solver(&input);
        let end = Instant::now();

        println!("day{}/part1: {}", day, p1);
        println!("day{}/part2: {}", day, p2);
        println!("day{}/solve_time: {:?}", day, end - start);
    });
    let end = Instant::now();
    println!("Total solve_time: {:?}", end - start);
}
