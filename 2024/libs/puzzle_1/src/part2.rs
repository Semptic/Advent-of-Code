use std::{fs::File, io::{self, BufRead}};

use anyhow::Result;
use log::debug;

pub fn run(file: File) -> Result<usize> {
    let mut left_list = Vec::new(); 
    let mut right_list = Vec::new();

    let file = io::BufReader::new(file);

    for line in file.lines() {
        let line = line?;
        let mut word_iter = line.split_whitespace();
        let left = word_iter.next().unwrap();
        let right = word_iter.next().unwrap();

        let left = left.parse::<usize>()?;
        let right = right.parse::<usize>()?;

        left_list.push(left);
        right_list.push(right);
    }

    let d = similarity(left_list, right_list)?;

    Ok(d)
}

fn similarity(left: Vec<usize>, right: Vec<usize>) -> Result<usize> {
    let mut left = left;
    let mut right = right;
    
    left.sort();
    right.sort();

    let s = left.iter().fold(0, |acc, l| {
        let count = right.iter().filter(|r| *r == l).count();
        debug!("{l} -> {count}");

        acc + l * count
    });

    Ok(s)
}

#[cfg(test)]
mod tests {
    use test_log::test;
    use super::*;

    #[test]
    fn test() {
        let left = vec![3, 4, 2, 1, 3, 3];
        let right = vec![4, 3, 5, 3, 9, 3];

        let s = similarity(left, right).unwrap();

        assert_eq!(s, 31);
    }
}
