use crate::utils::Card;

pub fn get_card_points(card: &Card) -> u32 {
    let winner = card
        .numbers
        .iter()
        .filter(|num| card.winning_numbers.contains(num))
        .count() as u32;

    if winner == 0 {
        0
    } else if winner == 1 {
        1
    } else {
        2u32.pow(winner - 1)
    }
    // 4  2  5
    // 1  1  1
    // 2  2  2
    // 4     4
    // 8     8
    //       16
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
    fn test_get_card_points() {
        let cards = cards();

        let expected = vec![8, 2, 2, 1, 0, 0];

        let actual: Vec<u32> = cards.iter().map(get_card_points).collect();

        for (actual, expected) in actual.iter().zip(expected.iter()) {
            assert_eq!(actual, expected);
        }
    }
}
