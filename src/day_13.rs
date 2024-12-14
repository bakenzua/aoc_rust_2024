#![allow(unused_variables)]

use std::fs::read_to_string;

use regex::Regex;


const EXAMPLE_FILEPATH: &str = "./data/example_13.txt";
const INPUT_FILEPATH: &str = "./data/input_13.txt";

// main entry point for day 13
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
            panic!("Invalid part specified for day 13")
        }
    }
}

fn part_1(file_path: &str) -> i64 {
    let machines: Vec<ClawMachine> = parse_file(file_path);

    let result = machines.into_iter()
    .fold(
        0, 
        |acc, m|
        match m.solve_claw_prize() {
            Some(p) => acc + p,
            None => acc + 0
        }
    );
    result as i64
}

fn part_2(file_path: &str) -> i64 {
    let increment: i64 = 10000000000000;
    let machines: Vec<ClawMachine> = parse_file(file_path);

    let result = machines.into_iter()
    .map(
        |cm| 
        ClawMachine {
            button_a: cm.button_a,
            button_b: cm.button_b,
            prize: (cm.prize.0 + increment, cm.prize.1 + increment),
        }
    )
    .fold(
        0, 
        |acc, m|
        match m.solve_claw_prize() {
            Some(p) => acc + p,
            None => acc + 0
        }
    );
    result
}

fn parse_button_line(line_str: &str) -> (i64, i64) {
    let button_re = Regex::new(r"(\d+),\sY\+(\d+)").unwrap();
    match button_re.captures(line_str) {
        Some(digits) => {
            return (digits[1].parse::<i64>().unwrap(), digits[2].parse::<i64>().unwrap())
        },
        None => panic!("Could not find params: {}", line_str)
    }
}
fn parse_prize_line(line_str: &str) -> (i64, i64) {
    let prize_re = Regex::new(r"(\d+),\sY=(\d+)").unwrap();
    match prize_re.captures(line_str) {
        Some(digits) => {
            return (digits[1].parse::<i64>().unwrap(), digits[2].parse::<i64>().unwrap())
        },
        None => panic!("Could not find params: {}", line_str)
    }
}

fn parse_file(file_path: &str) -> Vec<ClawMachine> {

    let file_txt = read_to_string(file_path).expect("Could not read file!");
    let file_lines = file_txt.lines();

    let mut machines: Vec<ClawMachine> = Vec::new();
    let mut temp_machine = ClawMachine{
        button_a: (0,0),
        button_b: (0,0),
        prize: (0,0)
    };

    let mut claw_desc = ClawDesc::A;
    // let mut still_loop = true;
    

    for ln in file_lines {
        // println!("{:?}", ln);
        match claw_desc {
            // Button A: X+94, Y+34
            // Button B: X+22, Y+67
            // Prize: X=8400, Y=5400
            ClawDesc::A => {
                temp_machine.button_a = parse_button_line(ln);
                claw_desc = ClawDesc::B;
            },
            ClawDesc::B => {
                temp_machine.button_b = parse_button_line(ln);
                claw_desc = ClawDesc::Prize;
            },
            ClawDesc::Prize => {                        
                temp_machine.prize =  parse_prize_line(ln);
                machines.push(temp_machine.clone());
                claw_desc = ClawDesc::Space;
            },
            ClawDesc::Space => {
                temp_machine = ClawMachine{
                    button_a: (0,0),
                    button_b: (0,0),
                    prize: (0,0)
                };
                claw_desc = ClawDesc::A;
            }
        }
    }
    return machines
}

#[derive(Clone, Debug, Copy)]
struct ClawMachine {
    button_a: (i64, i64),
    button_b: (i64, i64),
    prize: (i64, i64)
}

impl ClawMachine {
    pub fn solve_claw_prize(&self) -> Option<i64> {
    
        // Cramers rule for 2x2
        let d = (self.button_a.0 * self.button_b.1) - (self.button_a.1 * self.button_b.0);
        let dx = (self.prize.0 * self.button_b.1) - (self.prize.1 * self.button_b.0);
        let dy = (self.button_a.0 * self.prize.1) - (self.button_a.1 * self.prize.0);
        
        // if non integer solution return None
        if (dx % d == 0) && (dy % d == 0) {
            return Some((3*dx/d) + (dy/d))
        } else { 
            return None 
        }
    }


    // pub fn solve_claw_prize_ab(&self) -> Option<(i64, i64)> {
    
    //     // Cramers rule for 2x2
    //     let d = (self.button_a.0 * self.button_b.1) - (self.button_a.1 * self.button_b.0);
    //     let dx = (self.prize.0 * self.button_b.1) - (self.prize.1 * self.button_b.0);
    //     let dy = (self.button_a.0 * self.prize.1) - (self.button_a.1 * self.prize.0);

    //     // if non integer solution return None
    //     if (dx % d == 0) && (dy % d == 0) {
    //         return Some((dx/d, dy/d))
    //     } else { 
    //         return None 
    //     }
    // }
}

enum ClawDesc {
    A,
    B,
    Prize,
    Space
}
///////////////////////////////////////////
//   Some but not all tests
///////////////////////////////////////////

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_part_1() {
        assert_eq!(480, part_1(EXAMPLE_FILEPATH));
    }

    // #[test]
    // fn test_part_2() {
    //     assert_eq!(0, part_2(EXAMPLE_FILEPATH));
    // }

    #[test]
    fn test_parse_file() {
        let machines: Vec<ClawMachine> = parse_file(EXAMPLE_FILEPATH);
        println!("blash!!!");
        assert_eq!(
            machines.len(),
            4
        )
    }

    #[test]
    fn test_parse_button() {
        let but_str = "Button A: X+94, Y+34";
        let res = parse_button_line(&but_str);

        assert_eq!(
            res,
            (94, 34)
        )
    }

    #[test]
    fn test_parse_prize() {
        let but_str = "Prize: X=8400, Y=5400";
        let res = parse_prize_line(&but_str);

        assert_eq!(
            res,
            (8400, 5400)
        )
    }

    #[test]
    fn test_solve_claw_prize() {
        let m = ClawMachine {
            button_a: (94, 34),
            button_b: (22, 67),
            prize: (8400, 5400)
        };
        assert_eq!(
            Some(280),
            m.solve_claw_prize()
        );

        let m = ClawMachine {
            button_a: (26, 66),
            button_b: (67, 21),
            prize: (12748, 12176)
        };
        assert_eq!(
            None,
            m.solve_claw_prize()
        );

        let m = ClawMachine {
            button_a: (26, 66),
            button_b: (67, 21),
            prize: (12748+10000000000000, 12176+10000000000000)
        };
        assert!(
            m.solve_claw_prize_ab().is_some()
        );
    }

}
