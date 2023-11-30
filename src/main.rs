use clap::Parser;

mod day;
mod solution;

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
        1 => day::day01::solve,
        _ => unimplemented!(),
    };

    let (p1, p2) = solver(&std::fs::read_to_string(format!("./input/day{:0>2}.txt", args.day)).unwrap());

    println!("day{}/part1: {}", args.day, p1);
    println!("day{}/part2: {}", args.day, p2);
}
