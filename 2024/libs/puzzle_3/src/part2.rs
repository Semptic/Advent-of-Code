use std::{
    fs::File,
    io::{self, BufRead},
};

use anyhow::Result;

use crate::parser::extract_mult_conditional;

pub fn run(file: File) -> Result<usize> {
    let file = io::BufReader::new(file);

    Ok(0)
}

fn compute(input: String) -> Result<isize> {
  let mult = extract_mult_conditional(input)?;

  let result = mult.iter().fold(0, |acc, mul| {
    acc + mul.x * mul.y
  });

  Ok(isize::try_from(result).unwrap())
}


#[cfg(test)]
mod tests {
    use super::*;
    use test_log::test;

    #[test]
    fn test() {
      let input = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
      let result = 48;

      assert_eq!(compute(input.to_string()).unwrap(), result);
    }
}
