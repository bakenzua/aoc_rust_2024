#![allow(unused_variables)]

use std::{
    collections::{HashMap, HashSet},
    fmt,
    fs::read_to_string,
};

const EXAMPLE_FILEPATH: &str = "./data/example_8.txt";
const INPUT_FILEPATH: &str = "./data/input_8.txt";

// main entry point for day 8
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
            panic!("Invalid part specified for day 8")
        }
    }
}

fn part_1(file_path: &str) -> i64 {
    let mut antenna_map = parse_file(file_path);
    antenna_map.calc_anti_nodes_part1();
    // println!("anti_nodes_length: {}", antenna_map.anti_nodes.len());
    antenna_map.anti_nodes.len() as i64
}

fn part_2(file_path: &str) -> i64 {
    let mut antenna_map = parse_file(file_path);
    antenna_map.calc_anti_nodes_part2();
    // println!("anti_nodes_length: {}", antenna_map.anti_nodes.len());
    antenna_map.anti_nodes.len() as i64
}

struct AntennaMap {
    antennas: HashMap<(i64, i64), char>,
    antenna_types: HashSet<char>,
    anti_nodes: HashSet<(i64, i64)>,
    max_x: i64,
    max_y: i64,
}

impl AntennaMap {
    fn calc_anti_nodes_part1(&mut self) {
        for antenna_type in self.antenna_types.clone() {
            let antennas_of_type: &Vec<(i64, i64)> = &self
                .antennas
                .clone()
                .into_iter()
                .filter(|(loc, c)| *c == antenna_type)
                .map(|(loc, c)| loc)
                .collect();

            for (i, ant1) in antennas_of_type.iter().enumerate() {
                for ant2 in antennas_of_type[(i + 1)..].iter() {
                    let dx = ant1.0 - ant2.0;
                    let dy = ant1.1 - ant2.1;
                    //

                    let dx = ant1.0 - ant2.0;
                    let dy = ant1.1 - ant2.1;
                    self.add_anti_node((ant1.0 + dx, ant1.1 + dy));
                    self.add_anti_node((ant2.0 - dx, ant2.1 - dy));
                }
            }
        }
    }
    fn calc_anti_nodes_part2(&mut self) {
        for antenna_type in self.antenna_types.clone() {
            let antennas_of_type: &Vec<(i64, i64)> = &self
                .antennas
                .clone()
                .into_iter()
                .filter(|(loc, c)| *c == antenna_type)
                .map(|(loc, c)| loc)
                .collect();

            for (i, ant1) in antennas_of_type.iter().enumerate() {
                for ant2 in antennas_of_type[(i + 1)..].iter() {
                    let dx = ant1.0 - ant2.0;
                    let dy = ant1.1 - ant2.1;

                    //
                    let mut tmp_coord = ant1.clone();

                    while self.in_map_bounds(tmp_coord) {
                        self.add_anti_node(tmp_coord);
                        tmp_coord = (tmp_coord.0 - dx, tmp_coord.1 - dy);
                    }

                    let mut tmp_coord = ant2.clone();
                    while self.in_map_bounds(tmp_coord) {
                        self.add_anti_node(tmp_coord);
                        tmp_coord = (tmp_coord.0 + dx, tmp_coord.1 + dy);
                    }
                }
            }
        }
    }

    fn add_anti_node(&mut self, anti_node: (i64, i64)) {
        if self.in_map_bounds(anti_node) {
            self.anti_nodes.insert(anti_node);
        }
    }

    fn in_map_bounds(&self, coord: (i64, i64)) -> bool {
        (coord.0 <= self.max_x) && (coord.0 >= 0) && (coord.1 <= self.max_y) && (coord.1 >= 0)
    }
}

impl fmt::Display for AntennaMap {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut output_str = String::new();
        for _ in 0..=self.max_x {
            output_str.push('=')
        }
        output_str.push('\n');

        for y in 0..=self.max_y {
            for x in 0..=self.max_x {
                let coord = (x, y);
                let mut insert_char = '.';
                match self.antennas.get(&coord) {
                    Some(c) => insert_char = *c,
                    _ => (),
                }
                match self.anti_nodes.get(&coord) {
                    Some(c) => insert_char = '#',
                    _ => (),
                }
                output_str.push(insert_char);
            }

            output_str.push('\n');
        }

        for _ in 0..=self.max_x {
            output_str.push('=')
        }
        output_str.push('\n');

        write!(f, "{}", output_str)
    }
}

fn parse_file(file_path: &str) -> AntennaMap {
    let file_text = read_to_string(file_path).expect("Could not read file: {file_path");

    let mut antennas: HashMap<(i64, i64), char> = HashMap::new();
    let mut antenna_types = HashSet::new();

    let max_x: i64 = <usize as TryInto<i64>>::try_into(
        file_text
            .split_once('\n')
            .expect("Could not split file")
            .0
            .chars()
            .count(),
    )
    .unwrap()
        - 1;
    let max_y: i64 = <usize as TryInto<i64>>::try_into(file_text.lines().count()).unwrap() - 1;

    for (y, line) in file_text.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            antenna_types.insert(c);
            match c {
                '.' => continue,
                _ => {
                    antennas.insert((x as i64, y as i64), c);
                }
            }
        }
    }
    AntennaMap {
        antennas: antennas,
        antenna_types: antenna_types,
        anti_nodes: HashSet::new(),
        max_x: max_x,
        max_y: max_y,
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
        assert_eq!(14, part_1(EXAMPLE_FILEPATH));
    }

    #[test]
    fn test_part1_workings() {
        let mut antenna_map = parse_file(EXAMPLE_FILEPATH);
        antenna_map.calc_anti_nodes_part1();

        println!("{}", antenna_map);

        assert_eq!(14, antenna_map.anti_nodes.len());
    }
    #[test]
    fn test_part2_workings() {
        let mut antenna_map = parse_file(EXAMPLE_FILEPATH);
        antenna_map.calc_anti_nodes_part2();

        println!("{}", antenna_map);
        for node in &antenna_map.anti_nodes {
            // println!("{:?}", node);
        }
        assert_eq!(34, antenna_map.anti_nodes.len());
    }

    #[test]
    fn test_part_2() {
        // assert!(true);
        assert_eq!(34, part_2(EXAMPLE_FILEPATH));
    }

    #[test]
    fn test_parse_file() {
        let antenna_map = parse_file(EXAMPLE_FILEPATH);

        assert_eq!(
            antenna_map
                .antennas
                .values()
                .into_iter()
                .filter(|a| **a == 'A')
                .count(),
            3
        );
        assert_eq!(
            antenna_map
                .antennas
                .values()
                .into_iter()
                .filter(|a| **a == '0')
                .count(),
            4
        );

        // max_x
        assert_eq!(antenna_map.max_x, 11);
        // max_y
        assert_eq!(antenna_map.max_y, 11);
    }

    #[test]
    fn test_calc_anti_nodes_part1() {
        let mut antenna_map = parse_file(EXAMPLE_FILEPATH);
        antenna_map.calc_anti_nodes_part1();
        println!("{}", antenna_map);
        assert!(antenna_map.anti_nodes.contains(&(3, 1)))
    }

    #[test]
    fn test_in_map_bounds() {
        let antenna_map = parse_file(EXAMPLE_FILEPATH);

        assert!(!antenna_map.in_map_bounds((12, 12)));
    }
}
