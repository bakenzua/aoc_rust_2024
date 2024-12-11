#![allow(unused_variables)]

use std::{collections::HashMap, fs::read_to_string};

use crate::aoc_utils;

const EXAMPLE_FILEPATH: &str = "./data/example_11.txt";
const INPUT_FILEPATH: &str = "./data/input_11.txt";

// main entry point for day 11
pub fn run(part: i16) {
    // part 1
    match part {
        1 => {
            let example_result: i64 = part_1(EXAMPLE_FILEPATH);
            println!("Example result: {example_result}");

            let question_result: i64 = part_1(INPUT_FILEPATH);
            println!("Question result: {question_result}");
        }
        2 => {
            let example_result: i64 = part_2(EXAMPLE_FILEPATH);
            println!("Example result: {example_result}");

            let question_result: i64 = part_2(INPUT_FILEPATH);
            println!("Question result: {question_result}");
        }
        _ => {
            panic!("Invalid part specified for day 11")
        }
    }
}

fn parse_input(file_path: &str) -> HashMap<i64, i64> {
    let file_str = read_to_string(file_path).expect("Could not read file: {file_path");
    let integers: Vec<i64> = file_str.trim()
    .split_whitespace()
    .map(
        |st| st.parse::<i64>().expect("Could not parse: {st}")
    )
    .collect();
    let mut stones = HashMap::new();
    for integer in integers {
        insert_stone(&integer, 1, &mut stones);
    }
    stones
}

fn insert_stone(stone: &i64, ns: i64, stones: &mut HashMap<i64, i64>) {
    match stones.get(&stone) {
        Some(n) => stones.insert(*stone, n + ns),
        None => stones.insert(*stone, ns)
    };
}

fn blink(stones: HashMap<i64, i64>) -> HashMap<i64, i64> {
    let mut new_stones: HashMap<i64, i64> = HashMap::new();
    for stone in stones {
        if stone.0 == 0 {
            insert_stone(&1, stone.1, &mut new_stones);
            continue;
        }
        if aoc_utils::even_digits(&stone.0) {
            let split_stone = aoc_utils::split_digits(&stone.0).unwrap();
            // println!("Split stone: {:?}", split_stone);
            insert_stone(&split_stone.0, stone.1, &mut new_stones);
            insert_stone(&split_stone.1, stone.1, &mut new_stones);
        } else {
            insert_stone(&(stone.0 * 2024), stone.1, &mut new_stones);
        }
    }
    new_stones
}

fn part_1(file_path: &str) -> i64 {
    let mut stones = parse_input(file_path);
    for _ in 0..25 { stones = blink(stones) }
    stones.values().sum::<i64>() 
}

fn part_2(file_path: &str) -> i64 {
    let mut stones = parse_input(file_path);
    for _ in 0..75 { stones = blink(stones) }
    stones.values().sum::<i64>() 
}

///////////////////////////////////////////
//   Some but not all tests
///////////////////////////////////////////

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_blink() {
        
        let mut stones = parse_input(EXAMPLE_FILEPATH);
        for _ in 0..6 {stones = blink(stones)}
        
        assert_eq!(
            stones.values().sum::<i64>(),
            22
        );

        let mut stones = parse_input(EXAMPLE_FILEPATH);
        for _ in 0..25 {stones = blink(stones)}
        
        assert_eq!(
            stones.values().sum::<i64>(),
            55312
        );
    }

    #[test]
    fn test_part_1() {
        assert_eq!(55312, part_1(EXAMPLE_FILEPATH));
    }

    // #[test]
    // fn test_part_2() {
    //     // assert!(true);
    //     assert_eq!(0, part_2(EXAMPLE_FILEPATH));
    // }

    #[test]
    fn test_insert_stone() {
        let mut stones: HashMap<i64, i64> = HashMap::new();
        insert_stone(&1, 1, &mut stones);
        assert_eq!(
            stones.get(&1).unwrap(),
            &1
        );
        insert_stone(&1, 1, &mut stones);
        assert_eq!(
            stones.get(&1).unwrap(),
            &2
        );

        let mut stones: HashMap<i64, i64> = HashMap::new();
        insert_stone(&4048, 2, &mut stones);
        assert_eq!(
            stones.get(&4048).unwrap(),
            &2
        );
    }

    #[test]
    fn test_parse_input() {
        let stones = parse_input(EXAMPLE_FILEPATH);
        assert_eq!(
            Some(&1),
            stones.get(&125)
        );
        assert_eq!(
            Some(&1),
            stones.get(&17)
        );
    }
}
