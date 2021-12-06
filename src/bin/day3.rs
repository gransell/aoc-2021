use std::io::{self, BufRead};

#[allow(dead_code)]
fn part1(lines: &[String]) {
  let length = lines.first().unwrap().len();
  let gamma_string: String = (0..length)
    .map(|idx| {
      let most_common = lines
        .iter()
        .fold((0, 0), |acc, line| match line.chars().nth(idx) {
          Some('0') => (acc.0 + 1, acc.1),
          Some('1') => (acc.0, acc.1 + 1),
          _ => panic!("Invalid character"),
        });
      if most_common.0 > most_common.1 {
        "0".to_owned()
      } else {
        "1".to_owned()
      }
    })
    .collect();
  let gamma = i32::from_str_radix(&gamma_string, 2).unwrap();
  let epsilon = !gamma;

  println!("Power: {}", gamma * epsilon);
}

fn generate<F>(lines: &[String], f: F) -> i32
where
  F: Fn(i32, i32) -> bool,
{
  let length = lines.first().unwrap().len();
  let string = (0..length).fold(lines.to_owned(), |acc, idx| {
    if acc.len() == 1 {
      return acc;
    }
    let num_bits = acc
      .iter()
      .fold((0, 0), |acc, line| match line.chars().nth(idx) {
        Some('0') => (acc.0 + 1, acc.1),
        Some('1') => (acc.0, acc.1 + 1),
        _ => panic!("Invalid character"),
      });
    let f_res = f(num_bits.0, num_bits.1);
    let to_keep = if f_res { '0' } else { '1' };
    acc
      .iter()
      .filter(|line| line.chars().nth(idx) == Some(to_keep))
      .map(|line| line.to_owned())
      .collect()
  });
  i32::from_str_radix(string.first().unwrap(), 2).unwrap()
}

fn part2(lines: &[String]) {
  let oxygen = generate(lines, |a, b| a > b);
  let co2 = generate(lines, |a, b| a <= b);
  println!("Oxygen: {}", oxygen);
  println!("CO2: {}", co2);
  println!("Life support rating: {}", oxygen * co2);
}

#[test]
fn part_2() {
  let lines = vec![
    "00100".to_owned(),
    "11110".to_owned(),
    "10110".to_owned(),
    "10111".to_owned(),
    "10101".to_owned(),
    "01111".to_owned(),
    "00111".to_owned(),
    "11100".to_owned(),
    "10000".to_owned(),
    "11001".to_owned(),
    "00010".to_owned(),
    "01010".to_owned(),
  ];
  part2(&lines);
}

fn main() {
  let lines: Vec<String> = io::stdin()
    .lock()
    .lines()
    .map(|line| line.unwrap())
    .collect();

  part2(&lines);
}
