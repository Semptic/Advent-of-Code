use std::{fs::File, io::{self, BufRead}};

use anyhow::Result;

pub fn run(file: File) -> Result<i32> {
    let mut left_list = Vec::new(); 
    let mut right_list = Vec::new();

    let file = io::BufReader::new(file);

    for line in file.lines() {
        let line = line?;
        let mut word_iter = line.split_whitespace();
        let left = word_iter.next().unwrap();
        let right = word_iter.next().unwrap();

        let left = left.parse::<i32>()?;
        let right = right.parse::<i32>()?;

        left_list.push(left);
        right_list.push(right);
    }

    let d = distance(&mut left_list, &mut right_list)?;

    Ok(d)
}

fn distance(left: &mut [i32], right: &mut [i32]) -> Result<i32> {
    left.sort();
    right.sort();

    let d = left.iter().zip(right.iter()).fold(0, |acc, (l, r)| {
        let d = l - r;

        acc + d.abs()
    });

    Ok(d)
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
