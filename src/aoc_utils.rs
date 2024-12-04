use std::fs::read_to_string;

pub fn get_two_numbers_from_lines(file_path: &str) -> (Vec<i32>, Vec<i32>) {

    let mut left_list: Vec<i32> = Vec::new();
    let mut right_list: Vec<i32> = Vec::new();


    for line in read_to_string(file_path).unwrap().lines(){
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

  for line in read_to_string(file_path).unwrap().lines(){
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
    print!("{tree_ascii}\n\n")
}


pub enum Direction {
  North,
  NorthEast,
  East,
  SouthEast,
  South,
  SouthWest,
  West,
  NorthWest
}

#[derive(PartialEq, Eq, Hash, Debug, Clone, Copy)]
pub struct GridCoordinate {
    pub x: i32,
    pub y: i32,
}

impl GridCoordinate {
  pub fn get_element_to(&self, direction: &Direction) -> GridCoordinate {
      match direction {
          Direction::North     => {GridCoordinate{x: self.x,     y: self.y + 1}},
          Direction::NorthEast => {GridCoordinate{x: self.x + 1, y: self.y + 1}},
          Direction::East      => {GridCoordinate{x: self.x + 1, y: self.y}}, 
          Direction::SouthEast => {GridCoordinate{x: self.x + 1, y: self.y - 1}},
          Direction::South     => {GridCoordinate{x: self.x,     y: self.y - 1}},
          Direction::SouthWest => {GridCoordinate{x: self.x - 1, y: self.y - 1}},
          Direction::West      => {GridCoordinate{x: self.x - 1, y: self.y}},
          Direction::NorthWest => {GridCoordinate{x: self.x - 1, y: self.y + 1}}
      }
  }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_get_element_to_north_and_south() {
        let ge = GridCoordinate{
            x: 1,
            y: 1
        };
        assert_eq!(
          ge.get_element_to(&Direction::North).x,
          1
        );
        assert_eq!(
          ge.get_element_to(&Direction::North).y,
          2
        );
        assert_eq!(
          ge.get_element_to(&Direction::South).y,
          0
        );
    }

}