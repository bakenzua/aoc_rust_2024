use std::env;

mod aoc_utils;
mod day_1;
mod day_2;
mod day_3;

fn main() {
    let args:Vec<String> = env::args().collect();

    if args.len() != 3 {
        eprintln!("Usage: cargo run <day> <part>");
        std::process::exit(1);
    }

    let day: i32 = args[1].parse().expect("Please provide a day number.");
    let part: i32 = args[2].parse().expect("Please provide a part number.");

    aoc_utils::print_xmas_tree();
    println!("\n\n//===========================================//");
    println!("//       Advent of code: Day {}, part {}       //", day, part);
    println!("//===========================================//\n");


    // let start = Instant::now();
    match (day, part) {
        (1, 1) => day_1::run(1),
        (1, 2) => day_1::run(2),
        (2, 1) => day_2::run(1),
        (2, 2) => day_2::run(2),
        (3, 1) => day_3::run(1),
        (3, 2) => day_3::run(2),
        _ => {
            println!("Solution for day {} part {} not found.", day, part);
            std::process::exit(1);
        }
    }
    println!("\n\n")
}
