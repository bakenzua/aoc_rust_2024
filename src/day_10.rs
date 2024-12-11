#![allow(unused_variables)]

use std::{
    collections::HashMap
};

use crate::aoc_utils;

const EXAMPLE_FILEPATH: &str = "./data/example_10.txt";
const INPUT_FILEPATH: &str = "./data/input_10.txt";

// main entry point for day 10
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
            panic!("Invalid part specified for day 10")
        }
    }
}

fn part_1(file_path: &str) -> i32 {
    let trail_map: HashMap<(i32, i32), i32> = aoc_utils::parse_aoc_map(file_path);

    trail_map.iter().fold(
        0, |mut acc, coord| {
            if *coord.1 == 0 {
                match nines_reachable_from(coord.0, &trail_map) {
                    Some(mut nines) => {
                        nines.sort_unstable();
                        nines.dedup();
                        acc += nines.len() as i32;
                    },
                    None => ()
                }
            };
            acc
        }
    )
}

fn part_2(file_path: &str) -> i32 {
    let trail_map = aoc_utils::parse_aoc_map(file_path);

    trail_map.iter().fold(
        0, |mut acc, coord| {
            if *coord.1 == 0 {
               match nines_reachable_from(coord.0, &trail_map) {
                    Some(nines) => {
                        acc += nines.len() as i32;
                    },
                    None => ()
                }
            };
            acc
        }
    )
}

fn nines_reachable_from(coord: &(i32, i32), trail_map: &HashMap<(i32, i32), i32>) -> Option<Vec<(i32, i32)>> {
    fn internal_recursive_call(coord: (i32, i32), trail_map: &HashMap<(i32, i32), i32>, current_elevation: &i32, mut reachable_nines: Vec<(i32, i32)>) -> Vec<(i32, i32)> {

        match trail_map.get(&coord) {
            Some(e) => {
                if *e == current_elevation + 1 {
                    let nines = nines_reachable_from(&coord, trail_map);
                    match nines {
                        Some(mut nines) => {
                            _ = reachable_nines.append(&mut nines);
                        },
                        None => ()

                    }
                }
            }
            None => (),
        }
        return reachable_nines
    }

    let mut tmp_coord = coord.clone();
    let mut reachable_nines: Vec<(i32, i32)> = Vec::new();
    let current_elevation = trail_map.get(coord).unwrap();

    if *current_elevation == 9 {
        _ = reachable_nines.push(*coord);
        return Some(reachable_nines);
    } else {
        // north
        tmp_coord = (tmp_coord.0, tmp_coord.1 + 1);
        reachable_nines = internal_recursive_call(tmp_coord, trail_map, current_elevation, reachable_nines);

        // south
        tmp_coord = coord.clone();
        tmp_coord = (tmp_coord.0, tmp_coord.1 - 1);
        reachable_nines = internal_recursive_call(tmp_coord, trail_map, current_elevation, reachable_nines);

        // east
        tmp_coord = coord.clone();
        tmp_coord = (tmp_coord.0 + 1, tmp_coord.1);
        reachable_nines = internal_recursive_call(tmp_coord, trail_map, current_elevation, reachable_nines);

        // west
        tmp_coord = coord.clone();
        tmp_coord = (tmp_coord.0 - 1, tmp_coord.1);
        reachable_nines = internal_recursive_call(tmp_coord, trail_map, current_elevation, reachable_nines);
    }
    if reachable_nines.is_empty() {
        return None
    } else  {
        return Some(reachable_nines);
    }
}


///////////////////////////////////////////
//   Some but not all tests
///////////////////////////////////////////

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_part_1() {
        assert_eq!(36, part_1(EXAMPLE_FILEPATH));
    }

    #[test]
    fn test_part_2() {
        // assert!(true);
        assert_eq!(81, part_2(EXAMPLE_FILEPATH));
    }
}
