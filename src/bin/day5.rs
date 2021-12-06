use itertools::zip;
use num_iter::range_step_inclusive;
use std::{
  cmp::{max, min},
  collections::HashMap,
  io::{self, Read},
};

use regex::Regex;

#[derive(Debug, PartialEq, Eq, Hash)]
struct Point {
  x: i32,
  y: i32,
}

#[derive(Debug)]
struct Line {
  start: Point,
  end: Point,
}

impl Line {
  fn from_string(input: &str) -> Line {
    let re = Regex::new(r"(\d+),(\d+) -> (\d+),(\d+)").unwrap();
    let captures = re
      .captures(input)
      .unwrap_or_else(|| panic!("Invalid input: {}", input));
    let start_x = captures[1].parse::<i32>().unwrap();
    let start_y = captures[2].parse::<i32>().unwrap();
    let end_x = captures[3].parse::<i32>().unwrap();
    let end_y = captures[4].parse::<i32>().unwrap();

    Line {
      start: Point {
        x: start_x,
        y: start_y,
      },
      end: Point { x: end_x, y: end_y },
    }
  }

  fn all_points(&self) -> Vec<Point> {
    if self.start.x == self.end.x {
      let min = min(self.start.y, self.end.y);
      let max = max(self.start.y, self.end.y);
      (min..=max).map(|y| Point { x: self.start.x, y }).collect()
    } else if self.start.y == self.end.y {
      let min = min(self.start.x, self.end.x);
      let max = max(self.start.x, self.end.x);
      (min..=max).map(|x| Point { x, y: self.start.y }).collect()
    } else {
      let x_range = range_step_inclusive(
        self.start.x,
        self.end.x,
        if self.start.x < self.end.x { 1 } else { -1 },
      );
      let y_range = range_step_inclusive(
        self.start.y,
        self.end.y,
        if self.start.y < self.end.y { 1 } else { -1 },
      );
      zip(x_range, y_range).map(|(x, y)| Point { x, y }).collect()
    }
  }
}

fn part_1(input: &str) {
  let lines: Vec<_> = input
    .split('\n')
    .filter(|line| !line.is_empty())
    .map(|line| Line::from_string(line))
    .collect();
  let covered_lines: HashMap<Point, i32> = lines.iter().fold(HashMap::new(), |mut acc, line| {
    let all_points = line.all_points();
    for p in all_points {
      match acc.get(&p) {
        Some(val) => {
          let to_insert = val + 1;
          acc.insert(p, to_insert);
        }
        None => {
          acc.insert(p, 1);
        }
      }
    }
    acc
  });
  let num_collisions = covered_lines
    .iter()
    .fold(0, |acc, point| acc + if point.1 > &1 { 1 } else { 0 });
  println!("Collisions: {:?}", num_collisions);
}

#[test]
fn test_part_1() {
  let input = "0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2";

  part_1(input);
}

fn main() {
  let mut input = String::new();
  let _result = io::stdin().lock().read_to_string(&mut input);

  part_1(&input);
}
