#![allow(unused_variables)]

use crate::aoc_utils;

const EXAMPLE_FILEPATH: &str = "./data/example_2.txt";
const INPUT_FILEPATH: &str = "./data/input_2.txt";

fn part_1(file_path: &str) -> i32 {
    // get input data parsed into a vector of saftey reports (numeric vectors)
    let safety_reports: Vec<Vec<i32>> = aoc_utils::get_numbers_in_lines(file_path);

    let result: i32 = safety_reports
        .iter()
        .map(|v| is_safe(v))
        .filter(|x| *x)
        .count()
        .try_into()
        .unwrap();

    result
}

fn part_2(file_path: &str) -> i32 {
    // get input data parsed into a vector of safety reports (numeric vectors)
    let safety_reports = aoc_utils::get_numbers_in_lines(file_path);

    // brute force across all perms of every report code
    // hashmap or stop when safe perm found
    let result: i32 = safety_reports
        .iter()
        .map(|v| is_safe(v) | get_sub_reports(v).iter().any(|v| is_safe(v)))
        .filter(|x| *x)
        .count()
        .try_into()
        .unwrap();

    result
}

pub fn run(part: i16) {
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
            panic!("Incorrect part specified for day 2 run()")
        }
    };
}

// check if report code is 'safe'
// there is probably a more idiomatic method....match?
fn is_safe(report: &Vec<i32>) -> bool {
    let asc_desc = report.windows(2).all(|x| x[0] > x[1]) || report.windows(2).all(|x| x[0] < x[1]);

    // should check asc_desc is TRUE before running this...
    let diffs_lte_three = report.windows(2).all(|x| (x[0] - x[1]).abs() <= 3);

    asc_desc && diffs_lte_three
}

// generate permutations of a report code with individual elements removed
fn get_sub_reports(report: &Vec<i32>) -> Vec<Vec<i32>> {
    let mut sub_reports: Vec<Vec<i32>> = Vec::new();

    for i in 0..report.len() {
        let mut sub_report = report.clone();
        sub_report.remove(i);
        sub_reports.push(sub_report);
    }
    // sub_reports
    sub_reports
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(EXAMPLE_FILEPATH), 2);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(EXAMPLE_FILEPATH), 4);
    }

    #[test]
    fn ascending_is_safe() {
        assert!(is_safe(&vec![1, 2, 3]))
    }

    #[test]
    fn descending_is_safe() {
        assert!(is_safe(&vec![3, 2, 1]))
    }

    #[test]
    fn asc_and_descending_is_not_safe() {
        assert!(!is_safe(&vec![1, 2, 3, 2, 1]))
    }

    #[test]
    fn diff_gt_three() {
        assert!(!is_safe(&vec![1, 2, 7]))
    }

    #[test]
    fn sub_reports_correct_length() {
        assert_eq!(get_sub_reports(&vec![1, 2, 3]).len(), 3)
    }

    #[test]
    fn sub_reports_correct_lengths_for_elements() {
        assert!(get_sub_reports(&vec![1, 2, 3, 4])
            .iter()
            .all(|v| v.len() == 3))
    }
}
