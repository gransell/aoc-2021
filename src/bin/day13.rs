use itertools::Itertools;
use regex::Regex;
use std::{
  collections::HashSet,
  io::{self, Read},
};

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
struct Dot {
  pub y: i32,
  pub x: i32,
}

#[derive(Debug)]
enum Fold {
  AlongX(i32),
  AlongY(i32),
}

fn parse_dots(input: &str) -> HashSet<Dot> {
  let re = Regex::new(r"(\d+),(\d+)").unwrap();
  input
    .split('\n')
    .map(|line| {
      let capture = re.captures(line).unwrap();
      let x = capture[1].parse::<i32>().unwrap();
      let y = capture[2].parse::<i32>().unwrap();
      Dot { x, y }
    })
    .collect()
}

fn parse_folds(input: &str) -> Vec<Fold> {
  let re = Regex::new(r"fold along ([x|y])=(\d+)").unwrap();
  input
    .split('\n')
    .filter(|line| !line.is_empty())
    .fold(Vec::new(), |mut acc, line| {
      let capture = re.captures(line).unwrap();
      let axis = &capture[1];
      let coord: i32 = capture[2].parse().unwrap();
      let fold = match axis {
        "x" => Fold::AlongX(coord),
        "y" => Fold::AlongY(coord),
        _ => panic!("Invalid axis"),
      };
      acc.push(fold);
      acc
    })
}

fn find_unaffected_dots(dots: &mut HashSet<Dot>, fold: &Fold) -> HashSet<Dot> {
  dots
    .iter()
    .filter(|dot| match fold {
      Fold::AlongX(coord) => dot.x < *coord,
      Fold::AlongY(coord) => dot.y < *coord,
    })
    .cloned()
    .collect()
}

fn find_affected_dots(dots: &mut HashSet<Dot>, fold: &Fold) -> HashSet<Dot> {
  dots
    .iter()
    .filter(|dot| match fold {
      Fold::AlongX(coord) => dot.x > *coord,
      Fold::AlongY(coord) => dot.y > *coord,
    })
    .cloned()
    .collect()
}

fn dot_after_fold(original: &Dot, fold: &Fold) -> Dot {
  match fold {
    Fold::AlongY(coord) => Dot {
      x: original.x,
      y: coord - (original.y - coord),
    },
    Fold::AlongX(coord) => Dot {
      x: coord - (original.x - coord),
      y: original.y,
    },
  }
}

fn fold(dots: &mut HashSet<Dot>, fold: &Fold) -> HashSet<Dot> {
  let new = find_unaffected_dots(dots, fold);
  let affected = find_affected_dots(dots, fold);
  affected.iter().fold(new, |mut acc, dot| {
    acc.insert(dot_after_fold(dot, fold));
    acc
  })
}

#[allow(dead_code)]
fn part_1(input: &str) {
  let dots_end_idx = input.find("\n\n").unwrap();
  let mut parsed = parse_dots(&input[..dots_end_idx]);
  let folds = parse_folds(&input[dots_end_idx + 2..]);
  let remaining = fold(&mut parsed, folds.first().unwrap());
  println!("Remaining {:?}", remaining.len());
}

fn part_2(input: &str) {
  let dots_end_idx = input.find("\n\n").unwrap();
  let parsed = parse_dots(&input[..dots_end_idx]);
  let folds = parse_folds(&input[dots_end_idx + 2..]);
  let remaining = folds.iter().fold(parsed, |mut acc, f| fold(&mut acc, f));

  let max_x = remaining.iter().map(|dot| dot.x).max().unwrap() + 1;
  let sorted = remaining.iter().sorted().collect_vec();
  let mut current_line_idx = 0;
  let mut current_line = (0..max_x).map(|_| ".").collect::<String>();
  sorted.iter().for_each(|dot| {
    if dot.y > current_line_idx {
      println!("{}", current_line);
      current_line_idx += 1;
      current_line = (0..max_x).map(|_| ".").collect::<String>();
    }
    let idx: usize = dot.x.try_into().unwrap();
    current_line.replace_range(idx..idx + 1, "X");
  });
  println!("{}", current_line);
}

#[test]
fn test_part_2() {
  let input = "6,10
0,14
9,10
0,3
10,4
4,11
6,0
6,12
4,1
0,13
10,12
3,4
3,0
8,4
1,10
2,14
8,10
9,0

fold along y=7
fold along x=5";
  part_2(input);
}

fn main() {
  let mut input = String::new();
  let _result = io::stdin().lock().read_to_string(&mut input);
  part_2(&input);
}
