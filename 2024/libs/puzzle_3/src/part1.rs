use std::{
    fs::File,
    io::{self, BufRead, Read},
};

use anyhow::Result;

use crate::parser::extract_mult;

pub fn run(file: File) -> Result<isize> {
    let mut file = io::BufReader::new(file);
    
    let mut buffer = String::new();

    file.read_to_string(&mut buffer)?;

    compute(buffer)
}

fn compute(input: String) -> Result<isize> {
  let mult = extract_mult(input)?;

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
      let input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
      let result = 161;

      assert_eq!(compute(input.to_string()).unwrap(), result);
    }
}
