pub mod parse {
  pub fn match_literal(expected: &'static str) -> impl Fn(&str) -> Result<(&str, ()), &str> {
    move |input| match input.get(0..expected.len()) {
      Some(next) if next == expected => Ok((&input[expected.len()..], ())),
      _ => Err(input),
    }
  }

  #[test]
  fn literal_parser() {
    let parse_joe = match_literal("Hello Joe!");
    assert_eq!(Ok(("", ())), parse_joe("Hello Joe!"));
    assert_eq!(
      Ok((" Hello Robert!", ())),
      parse_joe("Hello Joe! Hello Robert!")
    );
    assert_eq!(Err("Hello Mike!"), parse_joe("Hello Mike!"));
  }

  fn identifier(input: &str) -> Result<(&str, String), &str> {
    let mut matched = String::new();
    let mut chars = input.chars();

    match chars.next() {
      Some(next) if next.is_alphabetic() => matched.push(next),
      _ => return Err(input),
    }

    while let Some(next) = chars.next() {
      if next.is_alphanumeric() || next == '-' {
        matched.push(next);
      } else {
        break;
      }
    }

    let next_index = matched.len();
    Ok((&input[next_index..], matched))
  }
}
