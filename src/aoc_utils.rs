use std::{collections::HashMap, fs::read_to_string};

pub fn get_two_numbers_from_lines(file_path: &str) -> (Vec<i32>, Vec<i32>) {
    let mut left_list: Vec<i32> = Vec::new();
    let mut right_list: Vec<i32> = Vec::new();

    for line in read_to_string(file_path).unwrap().lines() {
        let mut parts = line.split_whitespace();
        let a: i32 = parts.next().unwrap().parse().unwrap();
        let b: i32 = parts.next().unwrap().parse().unwrap();
        left_list.push(a);
        right_list.push(b);
    }
    (left_list, right_list)
}

pub fn get_numbers_in_lines(file_path: &str) -> Vec<Vec<i32>> {
    let mut result: Vec<Vec<i32>> = Vec::new();

    for line in read_to_string(file_path).unwrap().lines() {
        let parts = line
            .split_whitespace()
            .map(|x| x.parse::<i32>().unwrap())
            .collect();

        result.push(parts);
    }

    result
}

pub fn print_xmas_tree() {
    let tree_ascii: &str = r#"         |
        -+-
         A
        /=\               /\  /\    ___  _ __  _ __ __    __
      i/ O \i            /  \/  \  / _ \| '__|| '__|\ \  / /
      /=====\           / /\  /\ \|  __/| |   | |    \ \/ /
      /  i  \           \ \ \/ / / \___/|_|   |_|     \  /
    i/ O * O \i                                       / /
    /=========\        __  __                        /_/    _
    /  *   *  \        \ \/ /        /\  /\    __ _  ____  | |
  i/ O   i   O \i       \  /   __   /  \/  \  / _` |/ ___\ |_|
  /=============\       /  \  |__| / /\  /\ \| (_| |\___ \  _
  /  O   i   O  \      /_/\_\      \ \ \/ / / \__,_|\____/ |_|
i/ *   O   O   * \i
/=================\
       |___|
    "#;
    print!("\n\n{tree_ascii}\n")
}

pub fn parse_aoc_map(file_path: &str) -> HashMap<(i32, i32), i32> {
    let file_txt = read_to_string(file_path).expect("Could not read file: {file_path");
    let file_lines = file_txt.lines();

    let mut aoc_map: HashMap<(i32, i32), i32> = HashMap::new();
    let mut y: i32 = 0;

    for line in file_lines {
        for coord in line.chars().enumerate() {
            let x: i32 = coord.0 as i32;
            aoc_map.insert((x, y), coord.1.to_digit(10).unwrap() as i32);
        }
        y += 1;
    }
    aoc_map
}

pub enum CompassDirection {
    North,
    NorthEast,
    East,
    SouthEast,
    South,
    SouthWest,
    West,
    NorthWest,
}

#[derive(PartialEq, Eq, Hash, Debug, Clone, Copy)]
pub struct GridCoordinate {
    pub x: i32,
    pub y: i32,
}

impl GridCoordinate {
    pub fn move_direction(&mut self, direction: &CompassDirection, distance: i32) {
        //-> GridCoordinate {
        match direction {
            CompassDirection::North => {
                self.y += distance;
            }
            CompassDirection::NorthEast => {
                self.x += distance;
                self.y += distance;
            }
            CompassDirection::East => {
                self.x += distance;
            }
            CompassDirection::SouthEast => {
                self.x += distance;
                self.y -= distance;
            }
            CompassDirection::South => {
                self.y -= distance;
            }
            CompassDirection::SouthWest => {
                self.x -= distance;

                self.y -= distance;
            }
            CompassDirection::West => {
                self.x -= distance;
            }
            CompassDirection::NorthWest => {
                self.x -= distance;
                self.y += distance;
            }
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_move_direction_north() {
        let mut ge = GridCoordinate { x: 1, y: 1 };

        ge.move_direction(&CompassDirection::North, 1);

        assert_eq!(ge.y, 2);

        ge.move_direction(&CompassDirection::North, 2);

        assert_eq!(ge.y, 4);
    }
    #[test]
    fn test_move_direction_west() {
        let mut ge = GridCoordinate { x: 1, y: 1 };
        ge.move_direction(&CompassDirection::East, 1);

        assert_eq!(ge.x, 2);
    }
}
