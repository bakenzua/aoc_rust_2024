#![allow(unused_variables)]

use std::{collections::HashMap, fs::read_to_string};

const EXAMPLE_FILEPATH: &str = "./data/example_5.txt";
const INPUT_FILEPATH: &str = "./data/input_5.txt";

// main entry point for day 3
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
    let mut score: i32 = 0;

    score
}

fn part_2(file_path: &str) -> i32 {
    let filetxt = read_to_string(file_path).unwrap();

    let mut score: i32 = 0;

    score
}

enum ParserState {
    Rules,
    Pages,
}

fn parse_file(file_path: &str) -> (Vec<(i32, i32)>, Vec<Vec<i32>>) {
    let filetxt = read_to_string(file_path).expect("Error reading file: {file_path}");

    let mut state: ParserState = ParserState::Rules;
    let mut rules: Vec<(i32, i32)> = Vec::new();
    let mut manual_updates: Vec<Vec<i32>> = Vec::new();

    for line in filetxt.lines() {
        if line == "" {
            state = ParserState::Pages;
        } else {
            match state {
                ParserState::Rules => {
                    let mut rule = line.split('|');
                    let lhs = rule
                        .next()
                        .expect("LHS of rule not found: {line}")
                        .parse::<i32>()
                        .expect("LHS of rule would not parse {line}");
                    let rhs = rule
                        .next()
                        .expect("RHS of rule not found: {line}")
                        .parse::<i32>()
                        .expect("RHS of rule would not parse {line}");
                    rules.push((lhs, rhs));
                }
                ParserState::Pages => {
                    let pages: Vec<i32> = line
                        .split(',')
                        .map(|x| x.parse::<i32>().expect("Could not parse page number: {x}"))
                        .collect();
                    manual_updates.push(pages)
                }
            }
        }
    }
    (rules, manual_updates)
}

///////////////////////////////////////////
//   Some but not all tests
///////////////////////////////////////////

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_part_1() {
        assert_eq!(143, part_1(EXAMPLE_FILEPATH));
    }

    #[test]
    fn test_part_2() {
        // assert_eq!(9, part_2(EXAMPLE_FILEPATH));
    }

    #[test]
    fn test_parser() {
        let (rules, manual_updates) = parse_file(EXAMPLE_FILEPATH);

        assert_eq!(rules[0], (47, 53));

        assert_eq!(manual_updates[0], vec![75, 47, 61, 53, 29]);
    }
}
