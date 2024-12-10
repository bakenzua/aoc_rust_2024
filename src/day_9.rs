#![allow(unused_variables)]

use std::fs::read_to_string;

const EXAMPLE_FILEPATH: &str = "./data/example_9.txt";
const INPUT_FILEPATH: &str = "./data/input_9.txt";

// main entry point for day 9
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
            panic!("Invalid part specified for day 9")
        }
    }
}

fn parse_disk(file_path: &str) -> Vec<Option<i64>> {
    let file_text = read_to_string(file_path).expect("Could not read file: {file_path");
    let file_iter = file_text
        .trim()
        .chars()
        .map(|c| match c.to_digit(10) {
            Some(n) => n as i64,
            None => panic!("Could not convert {c} to digit!"),
        })
        .enumerate();

    let mut disk: Vec<Option<i64>> = Vec::new();

    let mut file_n = 0;

    // map files and empty space to disk
    for f in file_iter {
        for reps in 0..(f.1) {
            if f.0 % 2 == 0 {
                disk.push(Some(file_n));
            } else {
                disk.push(None);
            }
        }
        if f.0 % 2 == 0 {
            file_n += 1
        }
    }
    disk
}

fn part_1(file_path: &str) -> i64 {
    let mut disk = parse_disk(file_path);

    let mut start = 0;
    let mut last = disk.len() - 1;

    println!("{}, {}", start, last);

    while start < last {
        while disk[start].is_some() {
            start += 1;
        }
        while disk[last].is_none() {
            last -= 1;
        }
        if start < last {
            disk.swap(start, last)
        }
        start += 1;
        last -= 1;
    }
    // println!("{:?}", disk);
    checksum(&disk)
}

fn checksum(data: &Vec<Option<i64>>) -> i64 {
    data.iter()
        .enumerate()
        .map(|(i, v)| match v {
            Some(n) => (i as i64) * n,
            None => 0,
        })
        .sum::<i64>()
        .clone()
}

fn part_2(file_path: &str) -> i64 {
    let disk = parse_disk(file_path);
    0
}

///////////////////////////////////////////
//   Some but not all tests
///////////////////////////////////////////

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_part_1() {
        assert_eq!(1928, part_1(EXAMPLE_FILEPATH));
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
