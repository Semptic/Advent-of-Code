pub use nom::bytes::complete::tag;
use nom::{
    self,
    bytes::complete::take_until,
    multi::many0,
    sequence::{delimited, separated_pair},
    IResult,
};
use anyhow::{Result, bail};

#[derive(Debug, PartialEq)]
pub struct Mult {
    pub x: i32,
    pub y: i32,
}

fn parse_input(input: &str) -> IResult<&str, &str> {
    tag("mul")(input)
}

fn parse_pair(input: &str) -> IResult<&str, (i32, i32)> {
    use nom::character::complete::i32;
    separated_pair(i32, tag(","), i32)(input)
}

fn parse_mult(input: &str) -> IResult<&str, Mult> {
    let (remaining, (x, y)) = delimited(tag("mul("), parse_pair, tag(")"))(input)?;

    Ok((remaining, Mult { x, y }))
}

fn till_mult(s: &str) -> IResult<&str, &str> {
    take_until("mul(")(s)
}

fn parse_corrupt(input: &str) -> IResult<&str, Mult> {
    let result = till_mult(input);
    let (remaining, _) = result?;

    let result = parse_mult(remaining);
    match result {
        Ok(result) => Ok(result),
        Err(error) => match error {
            nom::Err::Incomplete(_) => Err(error),
            nom::Err::Error(error) => parse_corrupt(error.input),
            nom::Err::Failure(_) => Err(error),
        },
    }
}

fn parse_all(input: &str) -> IResult<&str, Vec<Mult>> {
    many0(parse_corrupt)(input)
}

pub fn extract_mult(input: String) -> Result<Vec<Mult>> {
  let result = parse_all(input.as_str());

  match result {
    Ok((_, mults)) => Ok(mults),
    Err(_) => bail!("parsing failed"),
  }

}

pub fn extract_mult_conditional(input: String) -> Result<Vec<Mult>> {
  let result = parse_all(input.as_str());

  match result {
    Ok((_, mults)) => Ok(mults),
    Err(_) => bail!("parsing failed"),
  }

}


#[cfg(test)]
mod test {
    use super::*;
    use test_log::test;

    #[test]
    fn test_parsing_mult() {
        let input = "mul(2,4)";
        let parsed = parse_mult(input);
        assert!(parsed.is_ok());
        let (remaining, mult) = parsed.unwrap();
        assert_eq!(mult, Mult { x: 2, y: 4 });
        assert_eq!(remaining, "");
    }

    #[test]
    fn test_parsing_simple_corrupted_1() {
        let input = "xmul(2,4)";
        let parsed = parse_corrupt(input);
        assert!(parsed.is_ok());
        let (remaining, mult) = parsed.unwrap();
        assert_eq!(mult, Mult { x: 2, y: 4 });
        assert_eq!(remaining, "");
    }
    #[test]
    fn test_parsing_simple_corrupted_2() {
        let input = "!@^do_not_mul(5,5)";
        let parsed = parse_corrupt(input);
        assert!(parsed.is_ok());
        let (remaining, mult) = parsed.unwrap();
        assert_eq!(mult, Mult { x: 5, y: 5 });
        assert_eq!(remaining, "");
    }
    #[test]
    fn test_parsing_simple_corrupted_3() {
        let input = "then(mul(11,8)";
        let parsed = parse_corrupt(input);
        assert!(parsed.is_ok());
        let (remaining, mult) = parsed.unwrap();
        assert_eq!(mult, Mult { x: 11, y: 8 });
        assert_eq!(remaining, "");
    }
    #[test]
    fn test_parsing_complex_corrupted_1() {
        let input = "+mul(32,64]then(mul(11,8)";
        let parsed = parse_corrupt(input);
        assert!(parsed.is_ok());
        let (remaining, mult) = parsed.unwrap();
        assert_eq!(mult, Mult { x: 11, y: 8 });
        assert_eq!(remaining, "");
    }
    #[test]
    fn test_parsing_with_remaining() {
        let input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        let parsed = parse_corrupt(input);
        assert!(parsed.is_ok());
        let (remaining, mult) = parsed.unwrap();
        assert_eq!(mult, Mult { x: 2, y: 4 });
        assert_eq!(
            remaining,
            "%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))"
        );
    }
    #[test]
    fn test_parsing_all() {
        let input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        let parsed = parse_all(input);
        assert!(parsed.is_ok());
        let (remaining, mults) = parsed.unwrap();
        assert_eq!(
            mults,
            vec![
                Mult { x: 2, y: 4 },
                Mult { x: 5, y: 5 },
                Mult { x: 11, y: 8 },
                Mult { x: 8, y: 5 }
            ]
        );
        assert_eq!(remaining, ")");
    }
}
