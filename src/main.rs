use std::env;

mod aoc_utils;
mod day_1;
mod day_2;
mod day_3;
mod day_4;
mod day_5;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        eprintln!("Usage: cargo run <day> <part>");
        std::process::exit(1);
    }

    let day: i16 = args[1].parse().expect("Please provide a day number.");
    let part: i16 = args[2].parse().expect("Please provide a part number.");

    aoc_utils::print_xmas_tree();
    println!("\n\n//===========================================//");
    println!(
        "//       Advent of code: Day {}, part {}       //",
        day, part
    );
    println!("//===========================================//\n");

    // let start = Instant::now();
    match day {
        1 => day_1::run(part),
        2 => day_2::run(part),
        3 => day_3::run(part),
        4 => day_4::run(part),
        5 => day_5::run(part),
        _ => {
            println!("Solution for day {} part {} not found.", day, part);
            std::process::exit(1);
        }
    }
    println!("\n\n")
}
