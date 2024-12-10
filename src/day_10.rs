#![allow(unused_variables)]

use std::{
    collections::HashMap,
    fs::read_to_string,
};

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

fn parse_disk(file_path: &str) -> HashMap<(i32, i32), i32> {
    let file_txt = read_to_string(file_path).expect("Could not read file: {file_path");
    let file_lines = file_txt.lines();

    let mut trail_map: HashMap<(i32, i32), i32> = HashMap::new();
    let mut y: i32 = 0;

    for line in file_lines {
        for coord in line.chars().enumerate() {
            let x: i32 = coord.0 as i32;
            trail_map.insert((x, y), coord.1.to_digit(10).unwrap() as i32);
        }
        y += 1;
    }
    trail_map
}

fn part_1(file_path: &str) -> i32 {
    let trail_map = parse_disk(file_path);

    let mut trailhead_scores: i32 = 0;

    for coord in &trail_map {
        if *coord.1 == 0 {
            let mut reachable_nines = nines_reachable_from(coord.0, &trail_map);
            reachable_nines.sort_unstable();
            reachable_nines.dedup();
            trailhead_scores += reachable_nines.len() as i32;
        }
    }
    trailhead_scores
}

fn part_2(file_path: &str) -> i32 {
    let trail_map = parse_disk(file_path);

    let mut trailhead_scores: i32 = 0;

    for coord in &trail_map {
        if *coord.1 == 0 {
            let reachable_nines = nines_reachable_from(coord.0, &trail_map);
            trailhead_scores += reachable_nines.len() as i32;
        }
    }
    trailhead_scores
}

fn nines_reachable_from(coord: &(i32, i32), trail_map: &HashMap<(i32, i32), i32>) -> Vec<(i32, i32)> {
    fn internal_recursive_call(coord: (i32, i32), trail_map: &HashMap<(i32, i32), i32>, current_elevation: &i32, mut reachable_nines: Vec<(i32, i32)>) -> Vec<(i32, i32)> {

        match trail_map.get(&coord) {
            Some(e) => {
                if *e == current_elevation + 1 {
                    let mut nines = nines_reachable_from(&coord, trail_map);
                    _ = reachable_nines.append(&mut nines);
                }
            }
            None => (),
        }
        return reachable_nines
    }

    let mut tmp_coord = coord.clone();
    // let mut score = 0;
    let mut reachable_nines: Vec<(i32, i32)> = Vec::new();

    let current_elevation = trail_map.get(coord).unwrap();

    if *current_elevation == 9 {
        _ = reachable_nines.push(*coord);
        return reachable_nines;
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
    reachable_nines
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
