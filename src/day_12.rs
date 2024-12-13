#![allow(unused_variables)]

use std::collections::{HashSet, HashMap};

use crate::aoc_utils;

const EXAMPLE1_FILEPATH: &str = "./data/example_12.1.txt";
const EXAMPLE2_FILEPATH: &str = "./data/example_12.2.txt";
const EXAMPLE3_FILEPATH: &str = "./data/example_12.3.txt";
const INPUT_FILEPATH: &str = "./data/input_12.txt";

// main entry point for day 12
pub fn run(part: i16) {
    // part 1
    match part {
        1 => {
            let example_result: i64 = part_1(EXAMPLE3_FILEPATH);
            println!("Example result: {example_result}");

            let question_result: i64 = part_1(INPUT_FILEPATH);
            println!("Question result: {question_result}");
        }
        2 => {
            let example_result: i64 = part_2(EXAMPLE3_FILEPATH);
            println!("Example result: {example_result}");

            let question_result: i64 = part_2(INPUT_FILEPATH);
            println!("Question result: {question_result}");
        }
        _ => {
            panic!("Invalid part specified for day 12")
        }
    }
}

fn part_1(file_path: &str) -> i64 {
    let world_map: HashMap<(i32, i32), char> = aoc_utils::parse_aoc_char_map(file_path);
    let mut veg_maps: Vec<VegMap> = Vec::new();
    let mut visited_coords: HashSet<(i32, i32)> = HashSet::new();

    'coord_loop: for coord in world_map.keys() {
        if visited_coords.contains(&coord) {
            continue 'coord_loop;
        }
        visited_coords.insert(*coord);

        let veg_type: char = *world_map.get(coord).unwrap();

        let veg_map = VegMap{
            coords: HashMap::new(),
            veg: veg_type
        };

        let free_edges: i32 = 0;
        
    }
    0
}

fn check_neighbours(
    coord: &(i32, i32), 
    world_map: &mut HashMap<(i32, i32), char>,
    current_veg_map: &mut VegMap
) -> (i32, Vec<(i32, i32)>){
    let free_edges: i32 = 0;

    // North
    let mut tmp_coord = (coord.0, coord.1 + 1);
    match world_map.get(&tmp_coord) {
        Some(c) => {
            if c == &current_veg_map.veg {
                // is in same VegMap
            } else {
                // is not in same VegMap
            }
        },
        None => free_edges += 1
        
    }
    return (0, (0,1))
}

fn part_2(file_path: &str) -> i64 {
    0
}

struct VegMap {
    coords: HashMap<(i32, i32), i32>,
    veg: char
}

///////////////////////////////////////////
//   Some but not all tests
///////////////////////////////////////////

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_part_1() {
        assert_eq!(140, part_1(EXAMPLE_FILEPATH));
        assert_eq!(772, part_1(EXAMPLE2_FILEPATH));
        assert_eq!(1930, part_1(EXAMPLE3_FILEPATH));
    }

    #[test]
    fn test_part_2() {
        // assert!(true);
        assert_eq!(0, part_2(EXAMPLE_FILEPATH));
    }

    #[test]
    fn test_checksum() {
        // let file_text = "0099811188827773336446555566..............";
        // let file_iter = file_text
        //     .chars()
        //     .map(|c| c.to_digit(10).unwrap() as i64)
        //     .enumerate();

        // let mut disk: Vec<Option<i64>> = Vec::new();

        // let mut file_n = 0;

        // // map files and empty space to disk
        // for f in file_iter {
        //     for reps in 0..(f.1) {
        //         if f.0 % 2 == 0 {
        //             disk.push(Some(f.1));

        //         } else {
        //             disk.push(None);
        //         }
        //     }
        //     if f.0 % 2 == 0 {file_n += 1}
        // }
        // assert_eq!(
        //     file_text.chars().enumerate().map(|(i, v)| {
        //         match v.to_digit(10) {
        //             Some(n) => (i as u32) * n,
        //             None => 0

        //         }
        //     })
        //     .sum::<u32>(),
        //     1928
        // );
        // assert_eq!(
        //     chksum(disk),
        //     1928
        // );
    }
}
