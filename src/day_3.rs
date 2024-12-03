#![allow(unused_variables)]

use std::fs::read_to_string;
use regex::Regex;
// use crate::aoc_utils;

const EXAMPLE_1_FILEPATH: &str = "./data/example_3.1.txt";
const EXAMPLE_2_FILEPATH: &str = "./data/example_3.2.txt";
const INPUT_1_FILEPATH: &str = "./data/input_3.txt";

// regex function to compile tuples of the operands of each mul() call
fn regex_muls(text: &str) -> Vec<(i32, i32)>{
    let re: Regex = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

    let mut commands = vec![];
    for (_, [op1, op2]) in re.captures_iter(text).map(|c| c.extract()) {
        commands.push((op1.parse::<i32>().unwrap(), op2.parse::<i32>().unwrap()));
    }
    commands
}

// regex function to remove text between do() and don't()
// unused
// fn remove_dont_code(text: &str) -> std::borrow::Cow<'_, str> {
//     let re: Regex = Regex::new(r"don't\(\)\S+do\(\)").unwrap();

//     let res = re.replace_all(text, NoExpand(""));
//     // print!("{res}");
//     res
// }

// part 1 solver
fn part_1(file_path: &str) -> i32 {

    // get input data 
    let corrupt_code: String = read_to_string(file_path).expect("Error reading file: {file_path}");

    part_1_calc(&corrupt_code)
}

// part 1 calculation using regex_muls() to compile operands
fn part_1_calc(corrupt_code: &str) -> i32 {
    // let ops = regex_muls(corrupt_code);
    let result = regex_muls(corrupt_code)
        .iter()
        .map(|ops| ops.0 * ops.1)
        .sum();
    result

}

// enum to hold state of parser
#[derive(PartialEq)]
enum DoSwitch {
    Do,
    DoNot
}

// part 2 solver
fn part_2(file_path: &str) -> i32  {
    let corrupt_code: String = read_to_string(file_path).expect("Error reading file: {file_path}");
    //part_2_calc(&corrupt_code)
    part_2_parse(&corrupt_code)
}

// part 2 direct parser
fn part_2_parse(corrupt_code: &str) -> i32 {

    // parse string with peekable iterator and enum DoSwitch
    let mut result: i32 = 0;
    let mut do_switch: DoSwitch = DoSwitch::Do;
    let mut corrupt_code_iter: std::iter::Peekable<std::str::Chars<'_>> = corrupt_code.chars().peekable();

    while let Some(chr) = corrupt_code_iter.next() {
        match chr {
            'm' => {
                // if next chars are ul(
                if corrupt_code_iter.next_if_eq(&'u').is_some()
                && corrupt_code_iter.next_if_eq(&'l').is_some()
                && corrupt_code_iter.next_if_eq(&'(').is_some() {
                    // capture digits until ','
                    let mut number_1 = String::new();
                    while let Some(d) = corrupt_code_iter.next_if(|&chr| chr.is_digit(10)) {
                        number_1.push(d);
                    }
                    // if next character is ',' then capture subsequent digits until ')'
                    if corrupt_code_iter.next_if_eq(&',').is_some() {
                        let mut number_2 = String::new();
                        while let Some(d) = corrupt_code_iter.next_if(|&chr| chr.is_digit(10)) {
                            number_2.push(d);
                        }
                    // if next char is ')' then parse the number strings to i32
                    if corrupt_code_iter.next_if_eq(&')').is_some() 
                    && do_switch == DoSwitch::Do {
                        if let (Ok(n1), Ok(n2)) = (number_1.parse::<i32>(), number_2.parse::<i32>()) {
                            // if we are in a do section, multiply the operands and add to the total
                            result += n1 * n2
                        }
                    }
                    }
                }
            }
            'd' => {
                // if next chars are o()
                if corrupt_code_iter.next_if_eq(&'o').is_some()
                && corrupt_code_iter.next_if_eq(&'(').is_some()
                && corrupt_code_iter.next_if_eq(&')').is_some() {
                    do_switch = DoSwitch::Do;
                } else 
                // if next chars are "n't()" - 'd' and 'o' have already been 
                // consumed if this is don't()
                if corrupt_code_iter.next_if_eq(&'n').is_some() 
                && corrupt_code_iter.next_if_eq(&'\'').is_some() 
                && corrupt_code_iter.next_if_eq(&'t').is_some() 
                && corrupt_code_iter.next_if_eq(&'(').is_some() 
                && corrupt_code_iter.next_if_eq(&')').is_some() {
                    do_switch = DoSwitch::DoNot;
                }
            },
            _ => continue,
        }
    }
    result
}


// original attempt using regexp remove_dont_code() 
// to remove 'code' between a don't() and a do()
// works on example not on input!!
// fn part_2_calc(corrupt_code: &str) -> i32 {
//     let do_code = remove_dont_code(&corrupt_code);
//     let result = regex_muls(&do_code)
//         .iter()
//         .map(|ops| ops.0 * ops.1)
//         .sum();
//     result

// }

// main entry point for day 3
pub fn run(part: i16){

    // part 1
    match part {
        1 => {
            let example_result: i32 = part_1(EXAMPLE_1_FILEPATH);
            println!("Example result: {example_result}");
    
            let question_result: i32 = part_1(INPUT_1_FILEPATH);
            println!("Question result: {question_result}");
        },
        2 => {
            let example_result: i32 = part_2(EXAMPLE_2_FILEPATH);
            println!("Example result: {example_result}");
    
            let question_result: i32 = part_2(INPUT_1_FILEPATH);
            println!("Question result: {question_result}");
        },
        _ => {
            panic!("Invalid part specified for day 3")
        }
    }
}



///////////////////////////////////////////
//   Some but not all tests
///////////////////////////////////////////

#[cfg(test)]
mod tests {

    use core::panic;

    use super::*;

    fn get_example_str(part: i32) -> String {

        match part {
            1 => {
                return read_to_string(EXAMPLE_1_FILEPATH)
                .expect("Error reading file: {file_path}");
            },
            2 => {
                return read_to_string(EXAMPLE_2_FILEPATH)
                    .expect("Error reading file: {file_path}");
            },
            _ => {
                panic!("Incorrect part supplied to example_str()")
            }
        }
    }

    #[test]
    fn test_regex_muls_returns_correct_number_of_matches() {
        assert_eq!(
            regex_muls(&get_example_str(1)).len(),
            4 
        )
    }

    #[test]
    fn test_part_1_calc() {
        assert_eq!(
            part_1_calc(&get_example_str(1)),
            161
        )
    }

    #[test]
    fn test_remove_dont_code() {
        assert_eq!(
            "xmul(2,4)&mul[3,7]!^?mul(8,5))",
            remove_dont_code(&get_example_str(2))

        );
    }

    // #[test]
    // fn test_part_2_calc() {
    //     // original attempt using regexp to chop out the 'code' between a don't() and a do()
    //     // works on example not on input!!
    //     assert_eq!(
    //         48,
    //         part_2_calc(remove_dont_code(&get_example_str(2)))
    //     );
    // }

    #[test]
    fn test_part_2_parse() {
        assert_eq!(
            77055967,
            part_2(INPUT_1_FILEPATH)
        )
    }

}