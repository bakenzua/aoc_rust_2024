#![allow(unused_variables)]

use std::{collections::HashMap, fs::read_to_string};

use itertools::Itertools;
use regex::Regex;
// use itertools::

const EXAMPLE_FILEPATH: &str = "./data/example_14.txt";
const EXAMPLE_MAP_SIZE: (i64, i64) = (11, 7);
const INPUT_FILEPATH: &str = "./data/input_14.txt";
const INPUT_MAP_SIZE: (i64, i64) = (101, 103);

// main entry point for day 14
pub fn run(part: i16) {
    // part 1
    match part {
        1 => {
            let example_result: i64 = part_1(EXAMPLE_FILEPATH, &EXAMPLE_MAP_SIZE);
            println!("Example result: {example_result}");

            let question_result: i64 = part_1(INPUT_FILEPATH, &INPUT_MAP_SIZE);
            println!("Question result: {question_result}");
        }
        2 => {
            let example_result: i64 = part_2(EXAMPLE_FILEPATH, &EXAMPLE_MAP_SIZE);
            println!("Example result: {example_result}");

            let question_result: i64 = part_2(INPUT_FILEPATH, &INPUT_MAP_SIZE);
            println!("Question result: {question_result}");
        }
        _ => {
            panic!("Invalid part specified for day 14")
        }
    }
}

fn part_1(file_path: &str, map_size: &(i64, i64)) -> i64 {
    let robots = parse_input(read_to_string(file_path).expect("Could not read file!"));

    let step = 100_i64;

    let robot_positions_after_step: Vec<(i64, i64)> = robots
        .iter()
        .map(|robot| {
            let mut rb = (
                (robot.0 .0 + (step * robot.1 .0)) % map_size.0,
                (robot.0 .1 + (step * robot.1 .1)) % map_size.1,
            );
            if rb.0 < 0 {
                rb.0 = rb.0 + map_size.0
            }
            if rb.1 < 0 {
                rb.1 = rb.1 + map_size.1
            }
            rb
        })
        .collect();

    let quad_counts = robot_positions_after_step
        .iter()
        .filter_map(|position| get_quadrant(position, map_size))
        .counts();
    println!("{:?}", quad_counts);
    quad_counts.values().product::<usize>().try_into().unwrap() // 219512160
}

fn part_2(file_path: &str, map_size: &(i64, i64)) -> i64 {
    let robots = parse_input(read_to_string(file_path).expect("Could not read file!"));
    let mut entropies = HashMap::new();

    // calculate Shannon entropy for every step up to 10000
    for step in 1..10000 {
        let robot_positions_after_step: Vec<(i64, i64)> = robots
            .iter()
            .map(|robot| {
                let mut rb = (
                    (robot.0 .0 + (step * robot.1 .0)) % map_size.0,
                    (robot.0 .1 + (step * robot.1 .1)) % map_size.1,
                );
                if rb.0 < 0 {
                    rb.0 = rb.0 + map_size.0
                }
                if rb.1 < 0 {
                    rb.1 = rb.1 + map_size.1
                }
                rb
            })
            .collect();

        let bin_size = 5_f64;

        let entropy: f64 = robot_positions_after_step.iter()
        .map(
            |robot| 
            // get_quadrant(robot, map_size)
            (
                (robot.0 as f64 / bin_size).floor() as i64,
                (robot.1 as f64 / bin_size).floor() as i64
            )
        )
        .counts()
        .values()
        .map(
            |count| 
            if *count > 0 {
                let p: f64 = (*count as f64 )/ (robots.len() as f64);
                -1_f64 * (p * p.log2())
            } else {
                0_f64
            }
        )
        .sum();

        entropies.insert(step, entropy);
    }

    // Find the key with the smallest value, is with smallest entropy
    entropies.iter()
        .min_by(|a, b| a.1.partial_cmp(b.1).unwrap())
        .map(|(key, _value)| *key).unwrap()

}

fn parse_input(file_txt: String) -> Vec<((i64, i64), (i64, i64))> {
    // "p=0,4 v=3,-3"
    let mut robots: Vec<((i64, i64), (i64, i64))> = Vec::new();
    let robot_re = Regex::new(r"(-?\d+),(-?\d+)\sv=(-?\d+),(-?\d+)").unwrap();

    for ln in file_txt.lines() {
        match robot_re.captures(ln) {
            Some(digits) => robots.push((
                (
                    digits[1].parse::<i64>().unwrap(),
                    digits[2].parse::<i64>().unwrap(),
                ),
                (
                    digits[3].parse::<i64>().unwrap(),
                    digits[4].parse::<i64>().unwrap(),
                ),
            )),
            None => panic!("Could not find params: {}", ln),
        }
    }

    // println!("{:?}", &robots);
    robots
}

fn get_quadrant(pos: &(i64, i64), bounds: &(i64, i64)) -> Option<Quadrant> {
    let centres = ((bounds.0 - 1) / 2, (bounds.1 - 1) / 2);
    if (pos.0 < centres.0) && (pos.1 < centres.1) {
        return Some(Quadrant::One);
    } else if (pos.0 > centres.0) && (pos.1 < centres.1) {
        return Some(Quadrant::Two);
    } else if (pos.0 < centres.0) && (pos.1 > centres.1) {
        return Some(Quadrant::Three);
    } else if (pos.0 > centres.0) && (pos.1 > centres.1) {
        return Some(Quadrant::Four);
    } else {
        return None;
    }
}

#[derive(Hash, Eq, PartialEq, Debug)]
enum Quadrant {
    One,
    Two,
    Three,
    Four,
}
///////////////////////////////////////////
//   Some but not all tests
///////////////////////////////////////////

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_part_1() {
        assert_eq!(12, part_1(EXAMPLE_FILEPATH, &EXAMPLE_MAP_SIZE));
    }

    #[test]
    fn test_part_2() {
        assert_eq!(10, part_2(EXAMPLE_FILEPATH, &EXAMPLE_MAP_SIZE));
    }

    #[test]
    fn test_parse_input() {
        let test_string = "p=0,4 v=3,-3".to_string();
        assert_eq!(parse_input(test_string), vec![((0, 4), (3, -3))])
    }

    #[test]
    fn test_get_quadrant() {
        assert_eq!(get_quadrant(&(0_i64, 0_i64), &INPUT_MAP_SIZE), Some(Quadrant::One));
        assert_eq!(get_quadrant(&(52_i64, 0_i64), &INPUT_MAP_SIZE), Some(Quadrant::Two));
        assert_eq!(get_quadrant(&(50_i64, 0_i64), &INPUT_MAP_SIZE), None);
        assert_eq!(get_quadrant(&(50_i64, 52_i64), &INPUT_MAP_SIZE), None)
    }
}
