use anyhow::{bail, Context, Result};
use regex::Regex;

#[derive(Debug, PartialEq)]
pub struct Cubes {
    pub red: u32,
    pub green: u32,
    pub blue: u32,
}

#[derive(Debug, PartialEq)]
pub struct Game {
    pub id: u32,
    pub rounds: Vec<Cubes>,
}

pub fn parse_line(line: &str) -> Result<Game> {
    let parts: Vec<&str> = line.split(':').collect();

    if parts.len() != 2 {
        bail!("Invalid line: {}", line);
    }

    let game_id: u32 = parts[0]
        .trim()
        .strip_prefix("Game ")
        .context("Failed to parse game id")?
        .parse()
        .context("Failed to parse game id")?;

    let parts: Vec<&str> = parts[1].split(';').collect();

    let blue_re = Regex::new(r"([0-9]+) blue").unwrap();
    let red_re = Regex::new(r"([0-9]+) red").unwrap();
    let green_re = Regex::new(r"([0-9]+) green").unwrap();

    let rounds: Vec<Cubes> = parts
        .iter()
        .map(|round| {
            let blue = blue_re
                .captures(round)
                .and_then(|c| c.get(1))
                .map(|m| m.as_str())
                .and_then(|s| s.parse::<u32>().ok())
                .unwrap_or(0);
            let green = green_re
                .captures(round)
                .and_then(|c| c.get(1))
                .map(|m| m.as_str())
                .and_then(|s| s.parse::<u32>().ok())
                .unwrap_or(0);
            let red = red_re
                .captures(round)
                .and_then(|c| c.get(1))
                .map(|m| m.as_str())
                .and_then(|s| s.parse::<u32>().ok())
                .unwrap_or(0);

            Cubes { red, green, blue }
        })
        .collect();

    Ok(Game {
        id: game_id,
        rounds,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_line() {
        let inputs = vec![
            "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green",
            "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue",
            "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red",
            "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red",
            "Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green",
        ];

        let expected = vec![
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

        for (input, expected) in inputs.iter().zip(expected.into_iter()) {
            let actual = super::parse_line(input).unwrap();
            assert_eq!(actual, expected);
        }
    }
}
