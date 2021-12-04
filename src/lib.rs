pub mod parse {

  type ParseResult<'a, Output> = Result<(&'a str, Output), &'a str>;

  pub trait Parser<'a, Output> {
    fn parse(&self, input: &'a str) -> ParseResult<'a, Output>;
  }

  impl<'a, F, Output> Parser<'a, Output> for F
  where
    F: Fn(&'a str) -> ParseResult<Output>,
  {
    fn parse(&self, input: &'a str) -> ParseResult<'a, Output> {
      self(input)
    }
  }

  struct BoxedParser<'a, Output> {
    parser: Box<dyn Parser<'a, Output> + 'a>,
  }

  impl<'a, Output> BoxedParser<'a, Output> {
    fn new<P>(parser: P) -> Self
    where
      P: Parser<'a, Output> + 'a,
    {
      BoxedParser {
        parser: Box::new(parser),
      }
    }
  }

  impl<'a, Output> Parser<'a, Output> for BoxedParser<'a, Output> {
    fn parse(&self, input: &'a str) -> ParseResult<'a, Output> {
      self.parser.parse(input)
    }
  }

  pub fn match_literal<'a>(expected: &'static str) -> impl Parser<'a, ()> {
    move |input: &'a str| {
      if let Some(stripped) = input.strip_prefix(expected) {
        Ok((stripped, ()))
      } else {
        Err(input)
      }
    }
  }

  pub fn pair<'a, P1, P2, R1, R2>(parser1: P1, parser2: P2) -> impl Parser<'a, (R1, R2)>
  where
    P1: Parser<'a, R1>,
    P2: Parser<'a, R2>,
  {
    move |input| {
      parser1.parse(input).and_then(|(next_input, result1)| {
        parser2
          .parse(next_input)
          .map(|(last_input, result2)| (last_input, (result1, result2)))
      })
    }
  }

  pub fn map<'a, P, F, A, B>(parser: P, map_fn: F) -> impl Parser<'a, B>
  where
    P: Parser<'a, A>,
    F: Fn(A) -> B,
  {
    move |input| {
      parser
        .parse(input)
        .map(|(next_input, result)| (next_input, map_fn(result)))
    }
  }

  pub fn left<'a, P1, P2, R1, R2>(parser1: P1, parser2: P2) -> impl Parser<'a, R1>
  where
    P1: Parser<'a, R1>,
    P2: Parser<'a, R2>,
  {
    map(pair(parser1, parser2), |(left, _right)| left)
  }

  pub fn right<'a, P1, P2, R1, R2>(parser1: P1, parser2: P2) -> impl Parser<'a, R2>
  where
    P1: Parser<'a, R1>,
    P2: Parser<'a, R2>,
  {
    map(pair(parser1, parser2), |(_left, right)| right)
  }

  pub fn one_or_more<'a, P, A>(parser: P) -> impl Parser<'a, Vec<A>>
  where
    P: Parser<'a, A>,
  {
    move |mut input| {
      let mut result = Vec::new();

      if let Ok((next_input, first_item)) = parser.parse(input) {
        input = next_input;
        result.push(first_item);
      } else {
        return Err(input);
      }

      while let Ok((next_input, next_item)) = parser.parse(input) {
        input = next_input;
        result.push(next_item);
      }

      Ok((input, result))
    }
  }

  pub fn zero_or_more<'a, P, A>(parser: P) -> impl Parser<'a, Vec<A>>
  where
    P: Parser<'a, A>,
  {
    move |mut input| {
      let mut result = Vec::new();

      while let Ok((next_input, next_item)) = parser.parse(input) {
        input = next_input;
        result.push(next_item);
      }

      Ok((input, result))
    }
  }

  pub fn any_char(input: &str) -> ParseResult<char> {
    match input.chars().next() {
      Some(next) => Ok((&input[next.len_utf8()..], next)),
      _ => Err(input),
    }
  }

  pub fn pred<'a, P, A, F>(parser: P, predicate: F) -> impl Parser<'a, A>
  where
    P: Parser<'a, A>,
    F: Fn(&A) -> bool,
  {
    move |input| {
      if let Ok((next_input, value)) = parser.parse(input) {
        if predicate(&value) {
          return Ok((next_input, value));
        }
      }
      Err(input)
    }
  }

  #[test]
  fn test_literal_parser() {
    let parse_joe = match_literal("Hello Joe!");
    assert_eq!(Ok(("", ())), parse_joe.parse("Hello Joe!"));
    assert_eq!(
      Ok((" Hello Robert!", ())),
      parse_joe.parse("Hello Joe! Hello Robert!")
    );
    assert_eq!(Err("Hello Mike!"), parse_joe.parse("Hello Mike!"));
  }

  #[test]
  fn one_or_more_combinator() {
    let parser = one_or_more(match_literal("ha"));
    assert_eq!(Ok(("", vec![(), (), ()])), parser.parse("hahaha"));
    assert_eq!(Err("ahah"), parser.parse("ahah"));
    assert_eq!(Err(""), parser.parse(""));
  }

  #[test]
  fn zero_or_more_combinator() {
    let parser = zero_or_more(match_literal("ha"));
    assert_eq!(Ok(("", vec![(), (), ()])), parser.parse("hahaha"));
    assert_eq!(Ok(("ahah", vec![])), parser.parse("ahah"));
    assert_eq!(Ok(("", vec![])), parser.parse(""));
  }

  #[test]
  fn predicate_combinator() {
    let parser = pred(any_char, |c| *c == 'o');
    assert_eq!(Ok(("mg", 'o')), parser.parse("omg"));
    assert_eq!(Err("lol"), parser.parse("lol"));
  }
}
