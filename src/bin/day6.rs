use std::{
  collections::HashMap,
  io::{self, Read, Result},
};

use itertools::Itertools;

fn make_initial_state(fishes: &[u8]) -> HashMap<u8, u64> {
  fishes.iter().fold(HashMap::new(), |mut acc, days| {
    match acc.get(days) {
      Some(fish_count) => {
        let new_fish_count = fish_count + 1;
        acc.insert(*days, new_fish_count)
      }
      None => acc.insert(*days, 1),
    };
    acc
  })
}

fn part_1(fishes: &[u8], ticks: u32) {
  let initial_state = make_initial_state(fishes);
  let final_state = (1..=ticks).fold(initial_state, |acc, tick| {
    let mut next = HashMap::from([
      (0, 0),
      (1, 0),
      (2, 0),
      (3, 0),
      (4, 0),
      (5, 0),
      (6, 0),
      (7, 0),
      (8, 0),
    ]);
    acc.keys().sorted().for_each(|k| match k {
      0 => {
        let num_fishes = acc[k];
        next.insert(6, num_fishes);
        next.insert(8, num_fishes);
      }
      7 => {
        next.insert(6, next[&6] + acc[&7]);
      }
      1..=8 => {
        next.insert(k - 1, acc[k]);
      }
      _ => panic!("Invalid amount of days {}", k),
    });
    println!("Next for tick {}: {:?}", tick, next);
    next
  });
  println!("Final state {:?}", final_state);
  let num_fishes: u64 = final_state.values().sum();
  println!("Num fishes: {}", num_fishes);
}

fn parse_input(input: &str) -> Vec<u8> {
  input
    .split(',')
    .map(|days| days.parse::<u8>().unwrap())
    .collect()
}

#[test]
fn test_part_1() {
  let input = parse_input("1,4,2,4,5,3,5,2,2,5,2,1,2,4,5,2,3,5,4,3,3,1,2,3,2,1,4,4,2,1,1,4,1,4,4,4,1,4,2,4,3,3,3,3,1,1,5,4,2,5,2,4,2,2,3,1,2,5,2,4,1,5,3,5,1,4,5,3,1,4,5,2,4,5,3,1,2,5,1,2,2,1,5,5,1,1,1,4,2,5,4,3,3,1,3,4,1,1,2,2,2,5,4,4,3,2,1,1,1,1,2,5,1,3,2,1,4,4,2,1,4,5,2,5,5,3,3,1,3,2,2,3,4,1,3,1,5,4,2,5,2,4,1,5,1,4,5,1,2,4,4,1,4,1,4,4,2,2,5,4,1,3,1,3,3,1,5,1,5,5,5,1,3,1,2,1,4,5,4,4,1,3,3,1,4,1,2,1,3,2,1,5,5,3,3,1,3,5,1,5,3,5,3,1,1,1,1,4,4,3,5,5,1,1,2,2,5,5,3,2,5,2,3,4,4,1,1,2,2,4,3,5,5,1,1,5,4,3,1,3,1,2,4,4,4,4,1,4,3,4,1,3,5,5,5,1,3,5,4,3,1,3,5,4,4,3,4,2,1,1,3,1,1,2,4,1,4,1,1,1,5,5,1,3,4,1,1,5,4,4,2,2,1,3,4,4,2,2,2,3");
  part_1(&input, 256);
}

fn main() -> Result<()> {
  let mut input = String::new();
  let _result = io::stdin().lock().read_to_string(&mut input)?;
  let fishes = parse_input(&input);
  part_1(&fishes, 80);

  Ok(())
}
