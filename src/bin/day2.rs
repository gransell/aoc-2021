use std::{
  fmt::Debug,
  io::{self, BufRead},
};

enum Direction {
  Up,
  Down,
  Forward,
  Back,
}

struct Command {
  pub direction: Direction,
  pub distance: i32,
}

impl Command {
  fn from_line(line: &str) -> Command {
    let remainder: &str;
    let direction: Direction;
    if line.starts_with("up") {
      remainder = &line[3..];
      direction = Direction::Up;
    } else if line.starts_with("down") {
      remainder = &line[5..];
      direction = Direction::Down;
    } else if line.starts_with("forward") {
      remainder = &line[8..];
      direction = Direction::Forward
    } else if line.starts_with("back") {
      remainder = &line[5..];
      direction = Direction::Back;
    } else {
      panic!("Unknown command {}", line);
    }
    let distance = remainder.parse::<i32>().unwrap();
    Command {
      direction,
      distance,
    }
  }
}

#[derive(Debug)]
struct Position {
  pub x: i32,
  pub y: i32,
  pub aim: i32,
}

impl Position {
  fn zero() -> Position {
    Position { x: 0, y: 0, aim: 0 }
  }
}

#[allow(dead_code)]
fn part1(commands: &[Command]) {
  let final_location =
    commands
      .iter()
      .fold(Position::zero(), |loc, command| match command.direction {
        Direction::Up => Position {
          y: loc.y - command.distance,
          ..loc
        },
        Direction::Down => Position {
          y: loc.y + command.distance,
          ..loc
        },
        Direction::Forward => Position {
          x: loc.x + command.distance,
          ..loc
        },
        Direction::Back => Position {
          x: loc.x - command.distance,
          ..loc
        },
      });

  println!("Result: {}", final_location.x * final_location.y);
}

fn part2(commands: &[Command]) {
  let final_location =
    commands
      .iter()
      .fold(Position::zero(), |loc, command| match command.direction {
        Direction::Up => Position {
          aim: loc.aim - command.distance,
          ..loc
        },
        Direction::Down => Position {
          aim: loc.aim + command.distance,
          ..loc
        },
        Direction::Forward => Position {
          x: loc.x + command.distance,
          y: loc.y + command.distance * loc.aim,
          ..loc
        },
        Direction::Back => Position {
          x: loc.x - command.distance,
          ..loc
        },
      });

  println!("Result: {}", final_location.x * final_location.y);
}

fn main() {
  let commands: Vec<Command> = io::stdin()
    .lock()
    .lines()
    .map(|line| Command::from_line(&line.unwrap()))
    .collect();

  part2(&commands);
}
