#![allow(unused_variables)]

use std::{collections::HashMap, fs::read_to_string};

use itertools::Itertools;
use regex::Regex;

use crate::aoc_utils;

const EXAMPLE_1_FILEPATH: &str = "./data/example_16_1.txt";
const EXAMPLE_2_FILEPATH: &str = "./data/example_16_2.txt";
const INPUT_FILEPATH: &str = "./data/input_16.txt";

// main entry point for day 16
pub fn run(part: i16) {
    // part 1
    match part {
        1 => {
            let example_1_result: i64 = part_1(EXAMPLE_1_FILEPATH);
            println!("Example 1 result: {example_1_result}");
            let example_2_result: i64 = part_1(EXAMPLE_2_FILEPATH);
            println!("Example 2 result: {example_2_result}");

            let question_result: i64 = part_1(INPUT_FILEPATH);
            println!("Question result: {question_result}");
        }
        2 => {
            let example_1_result: i64 = part_2(EXAMPLE_1_FILEPATH);
            println!("Example 1 result: {example_1_result}");
            let example_2_result: i64 = part_2(EXAMPLE_2_FILEPATH);
            println!("Example 2 result: {example_2_result}");

            let question_result: i64 = part_2(INPUT_FILEPATH);
            println!("Question result: {question_result}");
        }
        _ => {
            panic!("Invalid part specified for day 16")
        }
    }
}

fn part_1(file_path: &str) -> i64 {
    // let maze: HashMap<(i32, i32), char> = aoc_utils::parse_aoc_char_map(file_path);
    
    0_i64
}

fn part_2(file_path: &str) -> i64 {
    // let maze: HashMap<(i32, i32), char> = aoc_utils::parse_aoc_char_map(file_path);
    0_i64
}

///////////////////////////////////////////
//   Some but not all tests
///////////////////////////////////////////

#[cfg(test)]
mod tests {


    use super::*;

    #[test]
    fn test_part_1() {
        assert_eq!(7036, part_1(EXAMPLE_1_STR));
        assert_eq!(11048, part_1(EXAMPLE_2_STR));
    }

    #[test]
    fn test_part_2() {
        assert_eq!(10, part_2(EXAMPLE_FILEPATH));
    }

}
