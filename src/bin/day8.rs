use regex::Regex;
use std::io::{self, Read};

// enum Digit {
//   One,
//   Two,
//   Three,
//   Four,
//   Five,
//   Six,
//   Seven,
//   Eight,
//   Nine,
//   Unknown,
// }

// struct Entry {
//   digit: Digit,
//   signal: String,
// }

// impl Entry {
//   fn from_string(input: &str) -> Entry {
//     Entry {
//       digit: Digit::Unknown,
//       signal: input.to_owned(),
//     }
//   }
// }

struct SignalInput {
  // patterns: Vec<Entry>,
// output: Vec<Entry>,
}

impl SignalInput {
  fn from_string(input: &str) -> SignalInput {
    let re = Regex::new(r"([a|b|c|d|e|g|f]+) ([a|b|c|d|e|g|f]+) ([a|b|c|d|e|g|f]+) ([a|b|c|d|e|g|f]+) ([a|b|c|d|e|g|f]+) ([a|b|c|d|e|g|f]+) ([a|b|c|d|e|g|f]+) ([a|b|c|d|e|g|f]+) ([a|b|c|d|e|g|f]+) ([a|b|c|d|e|g|f]+) \| ([a|b|c|d|e|g|f]+) ([a|b|c|d|e|g|f]+) ([a|b|c|d|e|g|f]+) ([a|b|c|d|e|g|f]+)").unwrap();
    // let mut parse_outputs = false;
    let re.captures()
    re.captures(input).iter().for_each(|capture| {
      let patterns: Vec<_> = (1..=10).map(|idx| capture[idx].to_owned()).collect();
      let output: Vec<_> = (11..capture.len())
        .map(|idx| capture[idx].to_owned())
        .collect();
      println!("Patterns: {:?}", patterns);
      println!("Output: {:?}", output);
    });

    SignalInput {
      // patterns: vec![],
      // output: vec![],
    }
  }
}

fn parse_input(input: &str) -> Vec<SignalInput> {
  input
    .split('\n')
    .map(|line| SignalInput::from_string(line))
    .collect()
}

#[test]
fn test_day8() {
  let _input = parse_input(
    "acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf",
  );
}

fn main() {
  let mut input = String::new();
  let _result = io::stdin().lock().read_to_string(&mut input);
  let _fishes = parse_input(&input);
}
