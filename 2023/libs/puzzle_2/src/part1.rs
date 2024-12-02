use crate::utils::{Cubes, Game};

pub fn validate_game(game: &Game, max: &Cubes) -> bool {
    game.rounds
        .iter()
        .all(|round| round.red <= max.red && round.green <= max.green && round.blue <= max.blue)
}

pub fn calculate_result(games: &[Game], max: &Cubes) -> u32 {
    games
        .iter()
        .filter(|game| validate_game(game, max))
        .map(|game| game.id)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_game() {
        let inputs = vec![
            Game {
                id: 1,
                rounds: vec![
                    Cubes {
                        red: 4,
                        green: 0,
                        blue: 3,
                    },
                    Cubes {
                        red: 1,
                        green: 2,
                        blue: 6,
                    },
                    Cubes {
                        red: 0,
                        green: 2,
                        blue: 0,
                    },
                ],
            },
            Game {
                id: 2,
                rounds: vec![
                    Cubes {
                        red: 0,
                        green: 2,
                        blue: 1,
                    },
                    Cubes {
                        red: 1,
                        green: 3,
                        blue: 4,
                    },
                    Cubes {
                        red: 0,
                        green: 1,
                        blue: 1,
                    },
                ],
            },
            Game {
                id: 3,
                rounds: vec![
                    Cubes {
                        red: 20,
                        green: 8,
                        blue: 6,
                    },
                    Cubes {
                        red: 4,
                        green: 13,
                        blue: 5,
                    },
                    Cubes {
                        red: 1,
                        green: 5,
                        blue: 0,
                    },
                ],
            },
            Game {
                id: 4,
                rounds: vec![
                    Cubes {
                        red: 3,
                        green: 1,
                        blue: 6,
                    },
                    Cubes {
                        red: 6,
                        green: 3,
                        blue: 0,
                    },
                    Cubes {
                        red: 14,
                        green: 3,
                        blue: 15,
                    },
                ],
            },
            Game {
                id: 5,
                rounds: vec![
                    Cubes {
                        red: 6,
                        green: 3,
                        blue: 1,
                    },
                    Cubes {
                        red: 1,
                        green: 2,
                        blue: 2,
                    },
                ],
            },
        ];

        let expected = vec![true, true, false, false, true];

        for (input, expected) in inputs.iter().zip(expected) {
            let actual = validate_game(
                input,
                &Cubes {
                    red: 20,
                    green: 8,
                    blue: 6,
                },
            );

            assert_eq!(actual, expected);
        }
    }

    #[test]
    fn test_calculate_result() {
        let inputs = vec![
            Game {
                id: 1,
                rounds: vec![
                    Cubes {
                        red: 4,
                        green: 0,
                        blue: 3,
                    },
                    Cubes {
                        red: 1,
                        green: 2,
                        blue: 6,
                    },
                    Cubes {
                        red: 0,
                        green: 2,
                        blue: 0,
                    },
                ],
            },
            Game {
                id: 2,
                rounds: vec![
                    Cubes {
                        red: 0,
                        green: 2,
                        blue: 1,
                    },
                    Cubes {
                        red: 1,
                        green: 3,
                        blue: 4,
                    },
                    Cubes {
                        red: 0,
                        green: 1,
                        blue: 1,
                    },
                ],
            },
            Game {
                id: 3,
                rounds: vec![
                    Cubes {
                        red: 20,
                        green: 8,
                        blue: 6,
                    },
                    Cubes {
                        red: 4,
                        green: 13,
                        blue: 5,
                    },
                    Cubes {
                        red: 1,
                        green: 5,
                        blue: 0,
                    },
                ],
            },
            Game {
                id: 4,
                rounds: vec![
                    Cubes {
                        red: 3,
                        green: 1,
                        blue: 6,
                    },
                    Cubes {
                        red: 6,
                        green: 3,
                        blue: 0,
                    },
                    Cubes {
                        red: 14,
                        green: 3,
                        blue: 15,
                    },
                ],
            },
            Game {
                id: 5,
                rounds: vec![
                    Cubes {
                        red: 6,
                        green: 3,
                        blue: 1,
                    },
                    Cubes {
                        red: 1,
                        green: 2,
                        blue: 2,
                    },
                ],
            },
        ];

        let actual = calculate_result(
            &inputs,
            &Cubes {
                red: 12,
                green: 13,
                blue: 14,
            },
        );

        assert_eq!(actual, 8);
    }
}
