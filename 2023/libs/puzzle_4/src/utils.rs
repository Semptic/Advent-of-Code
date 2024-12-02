use std::collections::HashSet;

use anyhow::{bail, Context, Result};

#[derive(Debug, PartialEq)]
pub struct Card {
    pub id: u32,
    pub numbers: Vec<u32>,
    pub winning_numbers: HashSet<u32>,
}

pub fn parse_line(line: &str) -> Result<Card> {
    let parts: Vec<&str> = line.split(':').collect();

    if parts.len() != 2 {
        bail!("Invalid line: {}", line);
    }

    let id: u32 = parts[0]
        .trim()
        .strip_prefix("Card")
        .context("Failed to parse card id")?
        .trim()
        .parse()
        .context("Failed to parse card id")?;

    let parts: Vec<&str> = parts[1].split('|').collect();

    if parts.len() != 2 {
        bail!("Invalid line: {}", line);
    }

    let winning_numbers: Result<HashSet<u32>> = parts[0]
        .trim()
        .split(' ')
        .map(|num| num.trim())
        .filter(|num| !num.is_empty())
        .map(|num| {
            num.parse()
                .with_context(|| format!("Failed to parse {num}"))
        })
        .collect();
    let winning_numbers = winning_numbers.context("Failed to parse winning numbers")?;
    let numbers: Result<Vec<u32>> = parts[1]
        .trim()
        .split(' ')
        .map(|num| num.trim())
        .filter(|num| !num.is_empty())
        .map(|num| {
            num.parse()
                .with_context(|| format!("Failed to parse {num}"))
        })
        .collect();
    let numbers = numbers.context("Failed to parse numbers")?;

    Ok(Card {
        id,
        numbers,
        winning_numbers,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_line() {
        let inputs = vec![
            "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53",
            "Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19",
            "Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1",
            "Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83",
            "Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36",
            "Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11",
        ];

        let expected = vec![
            Card {
                id: 1,
                winning_numbers: HashSet::from_iter(vec![41, 48, 83, 86, 17].into_iter()),
                numbers: vec![83, 86, 6, 31, 17, 9, 48, 53],
            },
            Card {
                id: 2,
                winning_numbers: HashSet::from_iter(vec![13, 32, 20, 16, 61].into_iter()),
                numbers: vec![61, 30, 68, 82, 17, 32, 24, 19],
            },
            Card {
                id: 3,
                winning_numbers: HashSet::from_iter(vec![1, 21, 53, 59, 44].into_iter()),
                numbers: vec![69, 82, 63, 72, 16, 21, 14, 1],
            },
            Card {
                id: 4,
                winning_numbers: HashSet::from_iter(vec![41, 92, 73, 84, 69].into_iter()),
                numbers: vec![59, 84, 76, 51, 58, 5, 54, 83],
            },
            Card {
                id: 5,
                winning_numbers: HashSet::from_iter(vec![87, 83, 26, 28, 32].into_iter()),
                numbers: vec![88, 30, 70, 12, 93, 22, 82, 36],
            },
            Card {
                id: 6,
                winning_numbers: HashSet::from_iter(vec![31, 18, 13, 56, 72].into_iter()),
                numbers: vec![74, 77, 10, 23, 35, 67, 36, 11],
            },
        ];

        for (input, expected) in inputs.iter().zip(expected.into_iter()) {
            let actual = super::parse_line(input).unwrap();
            assert_eq!(actual, expected);
        }
    }
}
