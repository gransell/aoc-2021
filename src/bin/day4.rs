use std::io::{self, Read};

use itertools::Itertools;

#[derive(Debug, Clone)]
struct BoardLocation {
  pub number: i32,
  pub is_marked: bool,
}
#[derive(Debug, Clone)]
struct Board {
  pub lines: Vec<Vec<BoardLocation>>,
}

#[derive(Debug)]
struct Bingo {
  pub numbers: Vec<i32>,
  pub current_round: usize,
  pub boards: Vec<Board>,
}

impl Bingo {
  fn play_next_round(&mut self) -> bool {
    if self.current_round >= self.numbers.len() {
      println!("num boards left {}", self.boards.len());
      return false;
    }
    let number = self.numbers[self.current_round];
    for board in &mut self.boards {
      for line in &mut board.lines {
        for location in line {
          if location.number == number {
            location.is_marked = true;
          }
        }
      }
    }
    self.current_round += 1;
    self.current_round < self.numbers.len()
  }

  fn current_round_number(&self) -> i32 {
    self.numbers[self.current_round - 1]
  }

  fn check_and_remove_winning_board(&mut self) -> Option<(Board, i32)> {
    let board_idx = self
      .boards
      .iter()
      .find_position(|board| is_winning_board(board))
      .map(|(pos, board_ref)| (pos, board_ref.clone()));

    if let Some((pos, board)) = board_idx {
      self.boards.remove(pos);
      return Some((board, self.current_round_number()));
    }
    None
  }
}

fn is_winning_board(board: &Board) -> bool {
  let winning_line = board.lines.iter().find(|line| {
    for location in line.iter() {
      if !location.is_marked {
        return false;
      }
    }
    true
  });
  if winning_line.is_some() {
    return true;
  }

  let num_columns = board.lines.first().unwrap().len();
  let winning_column = (0..num_columns).find(|col_idx| {
    for line in board.lines.iter() {
      if let Some(column_loc) = line.get(*col_idx) {
        if !column_loc.is_marked {
          return false;
        }
      }
    }
    true
  });
  if winning_column.is_some() {
    return true;
  }

  false
}

fn parse_board(board_str: &str) -> Board {
  let line_iter = board_str.split('\n');
  let locations = line_iter.fold(Vec::new(), |mut acc, line| {
    if line.is_empty() {
      return acc;
    }
    let trimmed = line.trim();
    let line_locations: Vec<BoardLocation> = trimmed
      .split(' ')
      .filter(|number| !number.is_empty())
      .map(|number| BoardLocation {
        number: number.trim().parse::<i32>().unwrap(),
        is_marked: false,
      })
      .collect();
    acc.push(line_locations);
    acc
  });
  Board { lines: locations }
}

fn parse_game(input: &str) -> Bingo {
  let mut board_iterator = input.split("\n\n");
  let numbers: Vec<i32> = board_iterator
    .next()
    .map(|line| {
      line
        .split(',')
        .into_iter()
        .map(|n| n.parse::<i32>().unwrap())
        .collect()
    })
    .unwrap();

  let boards = board_iterator.fold(Vec::new(), |mut acc, board_string| {
    acc.push(parse_board(board_string));
    acc
  });

  Bingo {
    numbers,
    current_round: 0,
    boards,
  }
}

fn calculate_score(board: &Board, round_number: i32) -> i32 {
  let board_score = board.lines.iter().fold(0, |acc, line| {
    acc
      + line.iter().fold(0, |acc, location| {
        if location.is_marked {
          acc
        } else {
          acc + location.number
        }
      })
  });
  board_score * round_number
}

fn part_1(input: &str) {
  let mut game = parse_game(input);
  while game.play_next_round() && game.boards.len() > 1 {
    while game.check_and_remove_winning_board().is_some() {}
  }

  let score = calculate_score(game.boards.first().unwrap(), game.current_round_number());
  println!("Winning board score: {}", score);
}

#[test]
fn test_part_1() {
  let input = "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

  22 13 17 11  0
   8  2 23  4 24
  21  9 14 16  7
   6 10  3 18  5
   1 12 20 15 19

   3 15  0  2 22
   9 18 13 17  5
  19  8  7 25 23
  20 11 10 24  4
  14 21 16 12  6

  14 21 17 24  4
  10 16 15  9 19
  18  8 23 26 20
  22 11 13  6  5
   2  0 12  3  7";

  part_1(input);
}

fn main() {
  let mut input = String::new();
  let _result = io::stdin().lock().read_to_string(&mut input);

  part_1(&input);
}
