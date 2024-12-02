use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use anyhow::{Context, Result};

pub fn run(file: File) -> Result<isize> {
    let mut left_list = Vec::new();
    let mut right_list = Vec::new();

    let file = BufReader::new(file);

    for line in file.lines() {
        let line = line?;
        let mut words = line.split_whitespace();

        let left = words.next().context("No left")?;
        let left = left.parse::<isize>()?;

        let right = words.next().context("No right")?;
        let right = right.parse::<isize>()?;

        left_list.push(left);
        right_list.push(right);
    }

    distance(&mut left_list, &mut right_list)
}

fn distance(left: &mut [isize], right: &mut [isize]) -> Result<isize> {
    left.sort();
    right.sort();

    let result = left.iter().zip(right.iter()).fold(0, |acc, (l, r)| {
        let d = l - r;

        acc + d.abs()
    });

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let mut left = vec![3, 4, 2, 1, 3, 3];
        let mut right = vec![4, 3, 5, 3, 9, 3];

        let d = distance(&mut left, &mut right).unwrap();

        assert_eq!(d, 11);
    }
}
