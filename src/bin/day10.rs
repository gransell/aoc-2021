use std::io::{self, Read};

use itertools::Itertools;

#[derive(Debug)]
enum Error {
  Incomplete(Vec<Chunk>),
  Corrupted(char),
}

fn is_closing_char_for(opening_char: char, closing_char: char) -> bool {
  matches!(
    (opening_char, closing_char),
    ('(', ')') | ('[', ']') | ('{', '}') | ('<', '>')
  )
}

fn is_opening_char(ch: char) -> bool {
  matches!(ch, '(' | '[' | '{' | '<')
}

fn closing_char_for(opening_char: char) -> char {
  match opening_char {
    '(' => ')',
    '[' => ']',
    '{' => '}',
    '<' => '>',
    _ => panic!("Invalid opening char '{}'", opening_char),
  }
}

fn points_for_illegal(closing_char: char) -> i32 {
  match closing_char {
    ')' => 3,
    ']' => 57,
    '}' => 1197,
    '>' => 25137,
    _ => 0,
  }
}

fn auto_complete_score(closing_char: char) -> i64 {
  match closing_char {
    ')' => 1,
    ']' => 2,
    '}' => 3,
    '>' => 4,
    _ => 0,
  }
}

#[derive(Debug, Clone)]
struct Chunk {
  opening_char: char,
  closed: bool,
  children: Vec<Chunk>,
}

impl Chunk {
  fn new(opening_char: char) -> Chunk {
    Chunk {
      opening_char,
      closed: false,
      children: Vec::new(),
    }
  }

  fn parse<'a>(&mut self, input: &'a str) -> Result<&'a str, Error> {
    let mut input = input;
    while let Some(ch) = input.chars().next() {
      if is_opening_char(ch) {
        let mut child = Chunk::new(ch);
        input = child.parse(&input[1..]).map_err(|err| match err {
          Error::Incomplete(mut chunks) => {
            chunks.push(self.clone());
            Error::Incomplete(chunks)
          }
          _ => err,
        })?;
        child.close();
        self.children.push(child);
      } else if is_closing_char_for(self.opening_char, ch) || ch == '\n' {
        return Ok(&input[1..]);
      } else {
        return Err(Error::Corrupted(ch));
      }
    }
    Err(Error::Incomplete(vec![self.clone()]))
  }

  fn close(&mut self) {
    self.closed = true;
  }
}

fn parse(input: &str) -> Vec<Result<Vec<Chunk>, Error>> {
  input.split('\n').fold(Vec::new(), |mut acc, line| {
    let mut chunks = Vec::<Chunk>::new();
    let mut line_remainder = line;
    println!("'{}'", line_remainder);
    while let Some(opening_char) = line_remainder.chars().next() {
      let mut root = Chunk::new(opening_char);
      match root.parse(&line_remainder[1..]) {
        Ok(remainder) => {
          chunks.push(root);
          line_remainder = remainder;
        }
        Err(error) => {
          acc.push(Err(error));
          return acc;
        }
      }
    }
    acc.push(Ok(chunks));
    acc
  })
}

#[allow(dead_code)]
fn part_1(input: &str) {
  let parsed = parse(input);
  let points: i32 = parsed
    .iter()
    .map(|result| match result {
      Ok(_) => 0,
      Err(Error::Incomplete(_)) => 0,
      Err(Error::Corrupted(ch)) => points_for_illegal(*ch),
    })
    .sum();
  println!("Points: {}", points);
}

fn part_2(input: &str) {
  let parsed = parse(input);
  let auto_completed = parsed
    .iter()
    .filter_map(|result| match result {
      Ok(_) | Err(Error::Corrupted(_)) => None,
      Err(Error::Incomplete(chunks)) => Some(chunks),
    })
    .fold(Vec::new(), |mut acc, incomplete| {
      let to_complete = incomplete
        .iter()
        .fold(String::new(), |mut complete_acc, chunk| {
          complete_acc.push(closing_char_for(chunk.opening_char));
          complete_acc
        });
      acc.push(to_complete);
      acc
    });
  let points = auto_completed
    .iter()
    .map(|auto_completed| {
      auto_completed
        .chars()
        .fold(0, |acc, ch| (acc * 5) + auto_complete_score(ch))
    })
    .sorted_unstable()
    .collect_vec();

  let middle = points[points.len() / 2];

  println!("Middle {}", middle);
}

#[test]
fn test_part_1() {
  let input = "[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]";
  part_1(input);
}

#[test]
fn test_part_2() {
  let input = "[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]";
  part_2(input);
  // )}>]})
  // )}>]})
}

fn main() {
  let mut input = String::new();
  let _result = io::stdin().lock().read_to_string(&mut input);
  part_2(&input);
}
