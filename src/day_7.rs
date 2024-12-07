#![allow(unused_variables)]

use std::fs::read_to_string;

const EXAMPLE_FILEPATH: &str = "./data/example_7.txt";
const INPUT_FILEPATH: &str = "./data/input_7.txt";

// main entry point for day 7
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
            panic!("Invalid part specified for day 7")
        }
    }
}

fn part_1(file_path: &str) -> i64 {
    let calibrations = parse_file(file_path);

    let result = calibrations
        .into_iter()
        .filter(|x| eq_has_solutions(x.0, 0, &x.1))
        .map(|x| x.0)
        .sum();

    result
}

fn eq_has_solutions(answer: i64, current: i64, operands: &[i64]) -> bool {
    if operands.is_empty() {
        return answer == current;
    }
    if answer < current {
        return false;
    }
    let next_operand = *operands.first().unwrap();

    eq_has_solutions(answer, current + next_operand, &operands[1..])
        || eq_has_solutions(answer, current * next_operand, &operands[1..])
}

fn part_2(file_path: &str) -> i64 {
    0
}

fn parse_file(file_path: &str) -> Vec<(i64, Vec<i64>)> {
    let file_str = read_to_string(file_path).expect("Could not read file: {file_path}");
    let mut result = Vec::new();

    for line in file_str.lines() {
        let parts = line
            .split_once(":")
            .expect("Could not split line on ':': {line}");
        println!("{:?}", parts);
        let answer = parts
            .0
            .parse::<i64>()
            .expect("Could not parse {parts.0} as integer");
        let operands = parts
            .1
            .split_whitespace()
            .map(|op| {
                op.parse::<i64>()
                    .expect("Could not parse operand {op} to integer")
            })
            .collect();
        result.push((answer, operands));
    }

    result
}

///////////////////////////////////////////
//   Some but not all tests
///////////////////////////////////////////

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_part_1() {
        assert_eq!(3749, part_1(EXAMPLE_FILEPATH));
    }

    #[test]
    fn test_part_2() {
        // assert!(true);
        assert_eq!(6, part_2(EXAMPLE_FILEPATH));
    }

    #[test]
    fn test_parse_file() {
        let example = parse_file(EXAMPLE_FILEPATH);

        assert_eq!(example.len(), 9);

        assert_eq!(example[0].0, 190);

        assert_eq!(example[1].1.len(), 3)
    }
}
