use crate::aoc_utils;

fn part_1(file_path: &str) -> i32 {

    // get input data parsed into two vectors of ids
    let (mut l, mut r) = aoc_utils::get_two_numbers_from_lines(file_path);

    // // sort the parsed vectors
    l.sort();
    r.sort();

    let result: i32 = std::iter::zip(l, r)
        .map(|(l, r)| (l-r).abs())
        .sum();
    
    result
}


fn part_2(file_path: &str) -> i32  {
    let (l, r): (Vec<i32>, Vec<i32>) = aoc_utils::get_two_numbers_from_lines(file_path);

    let result = l.iter()
        .map(|number| {
            number * r.iter()
                .filter(|ii| *ii == number)
                .count() as i32
            }
        )
        .sum();
    result
}

pub fn run(part: i16){

    // Example day 1
    let example1_file_path = "./data/example_1.txt";
    // Question day 1
    let input1_file_path = "./data/input_1.txt";

    if part==1 {
        let example_result = part_1(example1_file_path);
        println!("Example result: {example_result}");

        let question_result = part_1(input1_file_path);
        println!("Question result: {question_result}");
    } else {
        let example_result = part_2(example1_file_path);
        println!("Example result: {example_result}");

        let question_result = part_2(input1_file_path);
        println!("Question result: {question_result}");
    };

}