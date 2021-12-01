use itertools::Itertools;
use std::io::{self, BufRead};

fn isDeeper(depth1: &i32, depth2: &i32) -> i32 {
  if depth1 < depth2 {
    1
  } else {
    0
  }
}

#[allow(dead_code)]
fn part1(lines: Vec<i32>) {
  let num_decreases: i32 = lines
    .iter()
    .tuple_windows()
    .map(|(i1, i2)| isDeeper(i1, i2))
    .sum();

  println!("Num decreases: {}", num_decreases);
}

fn part2(lines: Vec<i32>) {
  let num_decreases: i32 = lines
    .iter()
    .tuple_windows()
    .map(|(i1, i2, i3)| i1 + i2 + i3)
    .tuple_windows()
    .map(|(i1, i2)| isDeeper(&i1, &i2))
    .sum();

  println!("Num decreases: {}", num_decreases);
}

fn main() {
  let stdin = io::stdin();
  let lines: Vec<i32> = stdin
    .lock()
    .lines()
    .map(|line| line.map(|line| line.parse::<i32>().unwrap()).unwrap())
    .collect();
  part2(lines);
}
