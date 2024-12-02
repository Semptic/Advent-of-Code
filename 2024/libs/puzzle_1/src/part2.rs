use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use anyhow::{Context, Result};

pub fn run(file: File) -> Result<usize> {
    let mut left_list = Vec::new();
    let mut right_list = Vec::new();

    let file = BufReader::new(file);

    for line in file.lines() {
        let line = line?;
        let mut words = line.split_whitespace();

        let left = words.next().context("No left")?;
        let left = left.parse::<usize>()?;

        let right = words.next().context("No right")?;
        let right = right.parse::<usize>()?;

        left_list.push(left);
        right_list.push(right);
    }

    similarity(left_list, right_list)
}

fn similarity(left: Vec<usize>, right: Vec<usize>) -> Result<usize> {
    let mut left = left;
    let mut right = right;

    left.sort();
    right.sort();

    let s = left.iter().fold(0, |acc, l| {
        let count = right.iter().filter(|r| *r == l).count();

        acc + l * count
    });

    Ok(s)
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_log::test;

    #[test]
    fn test() {
        let left = vec![3, 4, 2, 1, 3, 3];
        let right = vec![4, 3, 5, 3, 9, 3];

        let s = similarity(left, right).unwrap();

        assert_eq!(s, 31);
    }
}
