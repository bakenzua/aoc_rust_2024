#![allow(unused_variables)]

use std::fs::read_to_string;


const EXAMPLE_FILEPATH: &str = "./data/example_14.txt";
const INPUT_FILEPATH: &str = "./data/input_14.txt";

// main entry point for day 14
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
            panic!("Invalid part specified for day 14")
        }
    }
}

fn part_1(file_path: &str) -> i64 {

    let robots = parse_input(
        read_to_string(file_path).expect("Could not read file!")
    );

    let result = 0;
    result as i64
}

fn part_2(file_path: &str) -> i64 {

    let robots = parse_input(
        read_to_string(file_path).expect("Could not read file!")
    );

    let result = 0;
    result as i64
}


fn parse_input(file_txt: String) -> Vec<((i32, i32), (i32, i32))>{

    let file_lines = file_txt.lines();
    let robots: Vec<((i32, i32), (i32, i32))> = Vec::new();

    robots
}

///////////////////////////////////////////
//   Some but not all tests
///////////////////////////////////////////

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_part_1() {
        assert_eq!(0, part_1(EXAMPLE_FILEPATH));
    }

    #[test]
    fn test_part_2() {
        assert_eq!(0, part_2(EXAMPLE_FILEPATH));
    }

    #[test]
    fn test_parse_input() {
        
        assert_eq!(
            4,
            4
        )
    }



}
