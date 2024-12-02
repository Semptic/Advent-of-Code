use crate::utils::{Cubes, Game};

pub fn get_min_cubes(game: &Game) -> Cubes {
    let mut min = Cubes {
        red: 0,
        green: 0,
        blue: 0,
    };

    for round in &game.rounds {
        if round.red > min.red {
            min.red = round.red;
        }
        if round.green > min.green {
            min.green = round.green;
        }
        if round.blue > min.blue {
            min.blue = round.blue;
        }
    }

    min
}

pub fn calculate_power(cube: &Cubes) -> u32 {
    cube.red * cube.green * cube.blue
}

pub fn sum_power(games: &[Game]) -> u32 {
    games
        .iter()
        .map(get_min_cubes)
        .map(|cube| calculate_power(&cube))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
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
        let expected_cubes = vec![
            Cubes {
                red: 4,
                green: 2,
                blue: 6,
            },
            Cubes {
                red: 1,
                green: 3,
                blue: 4,
            },
            Cubes {
                red: 20,
                green: 13,
                blue: 6,
            },
            Cubes {
                red: 14,
                green: 3,
                blue: 15,
            },
            Cubes {
                red: 6,
                green: 3,
                blue: 2,
            },
        ];

        let expected_power = vec![48, 12, 1560, 630, 36];

        for ((input, expected_cubes), expected_power) in inputs
            .iter()
            .zip(expected_cubes.iter())
            .zip(expected_power.iter())
        {
            let min = get_min_cubes(input);
            assert_eq!(min, *expected_cubes);
            let power = calculate_power(&min);
            assert_eq!(power, *expected_power);
        }

        let actual = sum_power(&inputs);
        assert_eq!(actual, 2286);
    }
}
