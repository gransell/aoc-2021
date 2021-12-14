use std::{
  collections::HashMap,
  io::{self, Read},
};

use itertools::Itertools;
use regex::Regex;

#[derive(Debug)]
struct Input {
  input: String,
  mappings: HashMap<String, String>,
}

fn parse(input: &str) -> Input {
  let mut lines = input.split('\n');
  let input = lines.next().unwrap().to_owned();
  lines.next(); // Get rid of empty line

  let re = Regex::new(r"(\w\w) -> (\w)").unwrap();
  let mappings = lines
    .filter(|line| !line.is_empty())
    .fold(HashMap::new(), |mut acc, line| {
      let captures = re.captures(line).unwrap();
      let from = captures[1].to_owned();
      let to = captures[2].to_owned();
      acc.insert(from, to);
      acc
    });

  Input { input, mappings }
}

fn map(input: &Input, num_steps: i32) -> String {
  (0..num_steps).fold(input.input.clone(), |acc, step| {
    let mut temp = acc
      .chars()
      .tuple_windows()
      .fold(String::new(), |acc, (current, next)| {
        let to_map: String = [current, next].iter().collect();
        if let Some(to_insert) = input.mappings.get(&to_map) {
          let to_append = current.to_string() + to_insert;
          return acc + &to_append;
        }
        acc
      });
    temp += &(acc.chars().last().unwrap().to_string());
    println!("Completed step {}\n{}", step, temp);
    temp
  })
}

fn into_character_map(word: &str) -> HashMap<char, i32> {
  word.chars().fold(HashMap::new(), |mut acc, c| {
    *acc.entry(c).or_insert(0) += 1;
    acc
  })
}

#[allow(dead_code)]
fn part_1(input: &str) {
  let parsed = parse(input);
  let mapped = map(&parsed, 10);
  println!("Completed mapping");
  let char_map = into_character_map(&mapped);
  let mut most_common_char = ('X', 0);
  let mut least_common_char = ('X', i32::MAX);
  char_map.iter().for_each(|(ch, count)| {
    if count > &most_common_char.1 {
      most_common_char = (*ch, *count);
    } else if count < &least_common_char.1 {
      least_common_char = (*ch, *count);
    }
  });

  println!("Result: {}", most_common_char.1 - least_common_char.1);
}

struct Input2 {
  last_char: char,
  input: HashMap<Vec<char>, i64>,
  mappings: HashMap<Vec<char>, char>,
}

fn parse_2(input: &str) -> Input2 {
  let mut lines = input.split('\n');
  let input_line = lines.next().unwrap().to_owned();
  let input =
    input_line
      .chars()
      .tuple_windows()
      .fold(HashMap::new(), |mut acc, (current, next)| {
        *acc.entry(vec![current, next]).or_insert(0) += 1;
        acc
      });
  let last_char = input_line.chars().last().unwrap();
  lines.next(); // Get rid of empty line

  let re = Regex::new(r"(\w\w) -> (\w)").unwrap();
  let mappings = lines
    .filter(|line| !line.is_empty())
    .fold(HashMap::new(), |mut acc, line| {
      let captures = re.captures(line).unwrap();
      let from = captures[1].chars().collect_vec();
      let to = captures[2].chars().next().unwrap();
      acc.insert(from, to);
      acc
    });

  Input2 {
    last_char,
    input,
    mappings,
  }
}

fn map_2(input: &Input2, num_steps: i32) -> HashMap<Vec<char>, i64> {
  (0..num_steps).fold(input.input.clone(), |acc, _step| {
    acc.iter().fold(HashMap::new(), |mut acc, (key, count)| {
      let mapping = input.mappings[key];
      *acc.entry(vec![key[0], mapping]).or_insert(0) += count;
      *acc.entry(vec![mapping, key[1]]).or_insert(0) += count;
      acc
    })
  })
}

fn count_chars_2(mapped: &HashMap<Vec<char>, i64>) -> HashMap<char, i64> {
  mapped.iter().fold(HashMap::new(), |mut acc, (k, v)| {
    *acc.entry(k[0]).or_insert(0) += v;
    acc
  })
}

#[allow(dead_code)]
fn part_2(input: &str) {
  let parsed = parse_2(input);
  let mapped = map_2(&parsed, 40);
  let mut char_count = count_chars_2(&mapped);
  *char_count.entry(parsed.last_char).or_insert(0) += 1;

  let mut most_common_char = ('.', 0);
  let mut least_common_char = ('.', i64::MAX);
  char_count.iter().for_each(|(ch, count)| {
    if count > &most_common_char.1 {
      most_common_char = (*ch, *count);
    } else if count < &least_common_char.1 {
      least_common_char = (*ch, *count);
    }
  });

  println!("Result: {}", most_common_char.1 - least_common_char.1);
}

#[test]
fn test_part_1() {
  let input = "NNCB

CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C";
  part_2(input);
}

fn main() {
  let mut input = String::new();
  let _result = io::stdin().lock().read_to_string(&mut input);
  part_2(&input);
}
