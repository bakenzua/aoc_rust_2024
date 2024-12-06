#![allow(unused_variables)]

use crate::aoc_utils::{CompassDirection, GridCoordinate};
use std::{collections::HashMap, fs::read_to_string};

const EXAMPLE_FILEPATH: &str = "./data/example_4.txt";
const INPUT_FILEPATH: &str = "./data/input_4.txt";

// main entry point for day 4
pub fn run(part: i16) {
    // part 1
    match part {
        1 => {
            let example_result: i32 = part_1(EXAMPLE_FILEPATH);
            println!("Example result: {example_result}");

            let question_result: i32 = part_1(INPUT_FILEPATH);
            println!("Question result: {question_result}");
        }
        2 => {
            let example_result: i32 = part_2(EXAMPLE_FILEPATH);
            println!("Example result: {example_result}");

            let question_result: i32 = part_2(INPUT_FILEPATH);
            println!("Question result: {question_result}");
        }
        _ => {
            panic!("Invalid part specified for day 4")
        }
    }
}

fn part_1(file_path: &str) -> i32 {
    // read and parse input to HashMap grid of chars
    let filetxt = read_to_string(file_path).unwrap();
    let grid = parse_to_grid(filetxt);

    let mut score: i32 = 0;

    // iterate through grid points and if X
    // look in all compass directions for XMAS
    for pos in grid.keys() {
        if grid.get(pos).unwrap() == &'X' {
            for direction in [
                CompassDirection::North,
                CompassDirection::NorthEast,
                CompassDirection::East,
                CompassDirection::SouthEast,
                CompassDirection::South,
                CompassDirection::SouthWest,
                CompassDirection::West,
                CompassDirection::NorthWest,
            ] {
                let mut current_location = pos.clone();
                let mut candidate = String::new();
                for _ in 0..4 {
                    candidate.push(*grid.get(&current_location).unwrap_or(&'%'));
                    current_location.move_direction(&direction, 1);
                }
                // println!("candidate: {candidate}");
                if candidate == "XMAS" {
                    score = score + 1;
                }
            }
        }
    }
    score
}

fn part_2(file_path: &str) -> i32 {
    let filetxt = read_to_string(file_path).unwrap();
    let grid = parse_to_grid(filetxt);

    let mut score: i32 = 0;
    let targets: Vec<&str> = vec!["SAM", "MAS"];

    for pos in grid.keys() {
        if grid.get(pos).unwrap() == &'A' {
            let mut current_location = pos.clone();
            let mut diag1 = String::new();
            let mut diag2 = String::new();

            current_location.move_direction(&CompassDirection::NorthWest, 1);
            diag1.push(*grid.get(&current_location).unwrap_or(&'%'));
            diag1.push('A');
            current_location.move_direction(&CompassDirection::SouthEast, 2);
            diag1.push(*grid.get(&current_location).unwrap_or(&'%'));

            current_location.move_direction(&CompassDirection::West, 2);
            diag2.push(*grid.get(&current_location).unwrap_or(&'%'));
            diag2.push('A');
            current_location.move_direction(&CompassDirection::NorthEast, 2);
            diag2.push(*grid.get(&current_location).unwrap_or(&'%'));
            if targets.contains(&&diag1.as_str()) && targets.contains(&&diag2.as_str()) {
                score = score + 1;
            }
        }
    }
    score
}

fn parse_to_grid(input: String) -> HashMap<GridCoordinate, char> {
    let mut grid: HashMap<GridCoordinate, char> = HashMap::new();
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            grid.insert(
                GridCoordinate {
                    x: x as i32,
                    y: y as i32,
                },
                c,
            );
        }
    }
    grid
}

///////////////////////////////////////////
//   Some but not all tests
///////////////////////////////////////////

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_part_1() {
        assert_eq!(18, part_1(EXAMPLE_FILEPATH));
    }

    #[test]
    fn test_part_2() {
        assert_eq!(9, part_2(EXAMPLE_FILEPATH));
    }

    #[test]
    fn test_parse_to_grid() {
        let grid = parse_to_grid(read_to_string(EXAMPLE_FILEPATH).unwrap());
        assert_eq!(grid.len(), 100)
    }
}
