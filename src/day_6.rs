#![allow(unused_variables)]

use crate::aoc_utils::GridCoordinate;
use std::collections::HashSet;
use std::fmt;
use std::fs::read_to_string;

const EXAMPLE_FILEPATH: &str = "./data/example_6.txt";
const INPUT_FILEPATH: &str = "./data/input_6.txt";

// main entry point for day 6
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
            panic!("Invalid part specified for day 6")
        }
    }
}

fn part_1(file_path: &str) -> i32 {

    let mut slm = parse_file(file_path);
    let mut current_direction: SuitLabDirection = SuitLabDirection::Down;

    while current_direction != SuitLabDirection::OffMap {
        current_direction = slm.move_step(current_direction);
    }
    slm.history.len() as i32
}

fn part_2(file_path: &str) -> i32 {
    let mut slm = parse_file(file_path);
    let mut loop_n: i32 = 0;

    // do and stash original path
    let mut current_direction: SuitLabDirection = SuitLabDirection::Down;
    while current_direction != SuitLabDirection::OffMap {
        current_direction = slm.move_step(current_direction);
    }
    let original_path = slm.history.drain();
    
    // start with clean map
    let slm = parse_file(file_path);
    for pos in original_path {
        let mut new_slm = slm.clone();
        _ = new_slm.obstacles.insert(GridCoordinate {x: pos.x, y: pos.y});
        if new_slm.is_loop_map() {
            loop_n += 1;
        }
    }
    loop_n
}

#[derive(Clone)]
struct SuitLabMap {
    obstacles: HashSet<GridCoordinate>,
    history: HashSet<GridCoordinate>,
    xmax: i32,
    ymax: i32,
    guardpos: GridCoordinate,
    steps_since_fresh: i32
}

#[derive(PartialEq, Clone, Copy)]
enum SuitLabDirection {
    Up,
    Right,
    Left,
    Down, 
    OffMap
}

impl SuitLabMap {

    fn off_map(&self, coord: GridCoordinate) -> bool {
        (coord.x < 0) || (coord.x > self.xmax) || (coord.y < 0) || (coord.y > self.ymax) 
    }

    pub fn max_loop_size(&self) -> i32 {
        (self.xmax + self.ymax) * 2
    }

    pub fn is_loop_map(&self) -> bool {
        let mut map_clone = self.clone();
        let mut current_direction = SuitLabDirection::Down;
        let max_loop_size = map_clone.max_loop_size();

        while current_direction != SuitLabDirection::OffMap {
            current_direction = map_clone.move_step(current_direction);
            if map_clone.steps_since_fresh >= max_loop_size {
                return true
            }
        }
        
        false
    }
    pub fn move_step(&mut self, direction: SuitLabDirection) -> SuitLabDirection {
        
        let mut dest = GridCoordinate{x:0, y:0};
        if self.history.contains(&self.guardpos) {
            self.steps_since_fresh += 1;
        } else {
            self.history.insert(self.guardpos.clone());
            self.steps_since_fresh = 0;
        }
        

        match direction {
            SuitLabDirection::Up => {
                dest.y = self.guardpos.y + 1;
                dest.x = self.guardpos.x;

                if self.obstacles.contains(&dest) {
                    return SuitLabDirection::Right
                } else if self.off_map(dest) {
                    return SuitLabDirection::OffMap;
                } else {
                    self.guardpos = dest;
                    return direction
                }
            },
            SuitLabDirection::Left => {
                dest.y = self.guardpos.y;
                dest.x = self.guardpos.x + 1;

                if self.obstacles.contains(&dest) {
                    return SuitLabDirection::Up
                } else if self.off_map(dest) {
                    return SuitLabDirection::OffMap;
                } else {
                    self.guardpos = dest;
                    return direction
                }
            },
            SuitLabDirection::Right => {
                dest.y = self.guardpos.y;
                dest.x = self.guardpos.x - 1;

                if self.obstacles.contains(&dest) {
                    return SuitLabDirection::Down
                } else if self.off_map(dest) {
                    return SuitLabDirection::OffMap;
                } else {
                    self.guardpos = dest;
                    return direction
                }
            },
            SuitLabDirection::Down => {
                dest.y = self.guardpos.y - 1;
                dest.x = self.guardpos.x;

                if self.obstacles.contains(&dest) {
                    return SuitLabDirection::Left
                } else if self.off_map(dest) {
                    return SuitLabDirection::OffMap;
                } else {
                    self.guardpos = dest;
                    return direction
                }
            },
            _ => return SuitLabDirection::OffMap
        }

    }
    
}

impl fmt::Display for SuitLabMap {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut output_str = String::new();
        for _ in 0..=self.xmax {output_str.push('=')};
        output_str.push('\n');

        for y in 0..=self.ymax {
            for x in 0..=self.xmax {
                let coord = GridCoordinate{x:x, y:y};
                if self.obstacles.contains(&coord) {
                    output_str.push('#');
                } else if self.guardpos == coord {
                    output_str.push('^');
                } else {
                    output_str.push('.');
                }
            }
            output_str.push('\n');
        }

        for _ in 0..=self.xmax {output_str.push('=')};
        output_str.push('\n');

        write!(f, "{}", output_str)
    }
}

fn parse_file(file_path: &str) -> SuitLabMap {
    let filetxt = read_to_string(file_path).expect("File should be read to string");

    let mut obstacles: HashSet<GridCoordinate> = HashSet::new();
    let mut guardpos = GridCoordinate{x:0, y:0};

    for (y, line) in filetxt.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            match c {
                '#' => {
                    obstacles.insert(GridCoordinate {
                        x: x as i32,
                        y: y as i32,
                    });
                }
                '^' => {
                    guardpos = GridCoordinate {
                        x: x as i32,
                        y: y as i32,
                    }
                }
                _ => continue,
            }
        }
    }

    let ymax: i32 = filetxt.lines().count() as i32 - 1;
    let firstline = filetxt.lines().next().expect("Could not get line from string");
    let xmax: i32 = firstline.chars().count() as i32 - 1;

    SuitLabMap {
        obstacles: obstacles,
        history: HashSet::new(),
        xmax: xmax,
        ymax: ymax,
        guardpos: guardpos,
        steps_since_fresh: 0
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
        assert_eq!(41, part_1(EXAMPLE_FILEPATH));
    }

    #[test]
    fn test_part_2() {
        // assert!(true);
        assert_eq!(6, part_2(EXAMPLE_FILEPATH));
    }

    #[test]
    fn test_parse_file() {
        let slm = parse_file(EXAMPLE_FILEPATH);

        assert_eq!(slm.obstacles.len(), 8);

        assert_eq!(slm.xmax, 9, "xmax is wrong");
        assert_eq!(slm.ymax, 9, "ymax x is wrong");

        assert_eq!(slm.guardpos.x, 4, "guardpos x is wrong");
        assert_eq!(slm.guardpos.y, 6, "guardpos y is wrong");
    }

    #[test]
    fn test_is_loop_map() {
        let mut slm = parse_file(EXAMPLE_FILEPATH);
        slm.obstacles.insert(GridCoordinate{x:3,y:6});
        assert!(slm.is_loop_map());
    }

    #[test]
    fn test_is_not_loop_map() {
        let mut slm = parse_file(EXAMPLE_FILEPATH);
        slm.obstacles.insert(GridCoordinate{x:2,y:6});
        assert!(!slm.is_loop_map());
    }

    #[test]
    fn test_max_loop_size() {
        let slm = parse_file(EXAMPLE_FILEPATH);
        assert_eq!(
            slm.max_loop_size(),
            9*4
        )
    }
}
