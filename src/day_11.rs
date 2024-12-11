#![allow(unused_variables)]

use std::fs::read_to_string;

const EXAMPLE_FILEPATH: &str = "./data/example_11.txt";
const INPUT_FILEPATH: &str = "./data/input_11.txt";

// main entry point for day 11
pub fn run(part: i16) {
    // part 1
    match part {
        1 => {
            let example_result: u64 = part_1(EXAMPLE_FILEPATH);
            println!("Example result: {example_result}");

            let question_result: u64 = part_1(INPUT_FILEPATH);
            println!("Question result: {question_result}");
        }
        2 => {
            let example_result: u64 = part_2(EXAMPLE_FILEPATH);
            println!("Example result: {example_result}");

            let question_result: u64 = part_2(INPUT_FILEPATH);
            println!("Question result: {question_result}");
        }
        _ => {
            panic!("Invalid part specified for day 11")
        }
    }
}

fn parse_disk(file_path: &str) -> Vec<String> {
    let file_str = read_to_string(file_path).expect("Could not read file: {file_path");
    file_str.trim()
    .split_whitespace()
    .map(
        |st| String::from(st)
    )
    .collect()
}

fn blink(stones: Vec<String>) -> Vec<String> {
    let mut new_stones: Vec<String> = Vec::new();
    for stone in stones {
        let stone_size = stone.chars().count() as u64;
        if stone == "0" {
            new_stones.push(String::from("1"));
        } else if stone_size % 2 == 0 {
            // let engrave_len = stone.chars().count() /2;
            _ = new_stones.push(String::from(&stone[..(stone.chars().count() /2)]));
            let string2 = String::from(stone[(stone.chars().count() /2)..stone.chars().count()]
                // .as_str()
                .trim_start_matches('0'));
            if string2.is_empty() {
                new_stones.push(String::from("0"))
            } else {
                new_stones.push(string2)
            }
                
        } else {
            _ = new_stones.push((stone.parse::<u64>().expect("COuld not parse: {stone}") * 2024).to_string());
        }
    }
    new_stones
}

fn part_1(file_path: &str) -> u64 {
    let mut stones = parse_disk(file_path);
    for _ in 0..25 { stones = blink(stones) }
    println!("{:?}", stones.len());
    stones.len() as u64
}

fn part_2(file_path: &str) -> u64 {
    let mut stones = parse_disk(file_path);
    // Ha Ha
    for _ in 0..75 { stones = blink(stones) }
    println!("{:?}", stones.len());
    stones.len() as u64
}

///////////////////////////////////////////
//   Some but not all tests
///////////////////////////////////////////

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_part_1() {
        assert_eq!(55312, part_1(EXAMPLE_FILEPATH));
    }

    #[test]
    fn test_part_2() {
        // assert!(true);
        assert_eq!(0, part_2(EXAMPLE_FILEPATH));
    }

    #[test]
    fn test_parse_disk() {
        let stones = parse_disk(EXAMPLE_FILEPATH);
        assert_eq!(
            vec!["125", "17"],
            stones
        );
    }
}
