use crate::part1;
use crate::utils::*;
use anyhow::{bail, Result};

pub fn count_all_wins(cards: &[Card]) -> Vec<u32> {
    cards.iter().map(part1::count_wins).collect()
}

fn get_new_cards(points: &[u32], current_pos: usize) -> Result<u32> {
    if current_pos > points.len() {
        return Ok(0);
    }
    if current_pos == 0 {
        bail!("Invalid current_pos: {current_pos}");
    }

    let next_cards = points[current_pos - 1] as usize;

    let mut sum = 1;
    for i in 1..(next_cards + 1) {
        sum += get_new_cards(points, current_pos + i)?;
    }

    Ok(sum)
}

pub fn get_all_new_cards(points: &[u32]) -> Result<u32> {
    let mut sum = 0;
    for i in 0..points.len() {
        let result = get_new_cards(points, i + 1)?;
        sum += result
    }

    Ok(sum)
}

#[cfg(test)]
mod test {
    use std::collections::HashSet;

    use super::*;

    fn cards() -> Vec<Card> {
        vec![
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
        ]
    }

    #[test]
    fn test_get_new_cards() {
        let points = vec![1, 1, 0];

        assert_eq!(get_new_cards(&points, 1).unwrap(), 3);
        assert_eq!(get_new_cards(&points, 2).unwrap(), 2);
        assert_eq!(get_new_cards(&points, 3).unwrap(), 1);
    }

    #[test]
    fn test_get_all_new_cards() {
        let points = vec![1, 1, 0];

        let actual = get_all_new_cards(&points).unwrap();

        let expected = 6;

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_with_test_data() {
        let cards = cards();
        let points = count_all_wins(&cards);

        // Original:
        // 1: 1 -> [2] -> [3] -> [4] -> [5]
        // 2: 2 -> [3] -> [4]
        // 3: 3 -> [3] -> [4] -> [5]
        // 4: 4 -> [5]
        // 5: 5
        // 6: 6

        // After recursion:
        // 1: 1 -> [2 -> [3 -> [4 -> [5]] -> [5]] -> [4 -> [5]]] -> [3 -> [4 -> [5]] -> [5]] -> [4 -> [5]] -> [5]   => 15
        // 2: 2 -> [3 -> [4 -> [5]] -> [5]] -> [4 -> [5]]                                                           => 7
        // 3: 3 -> [4 -> [5]] -> [5]                                                                                => 4
        // 4: 4 -> [5]                                                                                              => 2
        // 5: 5                                                                                                     => 1
        // 6: 6                                                                                                     => 1
        // ==========================================================================================================> 30

        assert_eq!(get_new_cards(&points, 1).unwrap(), 15);
        assert_eq!(get_new_cards(&points, 2).unwrap(), 7);
        assert_eq!(get_new_cards(&points, 3).unwrap(), 4);
        assert_eq!(get_new_cards(&points, 4).unwrap(), 2);
        assert_eq!(get_new_cards(&points, 5).unwrap(), 1);
        assert_eq!(get_new_cards(&points, 6).unwrap(), 1);

        assert_eq!(get_all_new_cards(&points).unwrap(), 30);
    }
}
