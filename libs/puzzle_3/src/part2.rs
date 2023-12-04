use anyhow::{Context, Result};
use std::collections::{HashMap, HashSet};

fn is_symbol(c: char) -> bool {
    !c.is_ascii_digit() && c != '.'
}

fn is_gear(c: char) -> bool {
    c == '*'
}

pub fn extract_part_numbers(engine: &Vec<Vec<char>>) -> Result<u32> {
    let mut gear_map: HashMap<(usize, usize), HashSet<u32>> = HashMap::new();

    for x in 0..engine.len() {
        let mut num = String::new();
        let mut is_part = false;
        let mut gears = HashSet::new();

        for y in 0..engine[x].len() {
            let c: char = engine[x][y];

            if c.is_ascii_digit() {
                num.push(c);

                if !is_part {
                    // above
                    if x > 0 {
                        // left
                        if y > 0 && is_symbol(engine[x - 1][y - 1]) {
                            is_part = true;
                            if is_gear(engine[x - 1][y - 1]) {
                                gears.insert((x - 1, y - 1));
                            }
                        }
                        // center
                        if is_symbol(engine[x - 1][y]) {
                            is_part = true;
                            if is_gear(engine[x - 1][y]) {
                                gears.insert((x - 1, y));
                            }
                        }
                        //right
                        if y + 1 < engine[x].len() && is_symbol(engine[x - 1][y + 1]) {
                            is_part = true;
                            if is_gear(engine[x - 1][y + 1]) {
                                gears.insert((x - 1, y + 1));
                            }
                        }
                    }

                    // same line
                    // left
                    if y > 0 && is_symbol(engine[x][y - 1]) {
                        is_part = true;
                        if is_gear(engine[x][y - 1]) {
                            gears.insert((x, y - 1));
                        }
                    }
                    // center
                    // if is_symbol(engine[x][y]) {
                    //     is_part = true;
                    // }
                    // right
                    if y + 1 < engine[x].len() && is_symbol(engine[x][y + 1]) {
                        is_part = true;
                        if is_gear(engine[x][y + 1]) {
                            gears.insert((x, y + 1));
                        }
                    }

                    // below
                    if x + 1 < engine.len() {
                        // left
                        if y > 0 && is_symbol(engine[x + 1][y - 1]) {
                            is_part = true;
                            if is_gear(engine[x + 1][y - 1]) {
                                gears.insert((x + 1, y - 1));
                            }
                        }
                        // center
                        if is_symbol(engine[x + 1][y]) {
                            is_part = true;
                            if is_gear(engine[x + 1][y]) {
                                gears.insert((x + 1, y));
                            }
                        }
                        // right
                        if y + 1 < engine[x].len() && is_symbol(engine[x + 1][y + 1]) {
                            is_part = true;
                            if is_gear(engine[x + 1][y + 1]) {
                                gears.insert((x + 1, y + 1));
                            }
                        }
                    }
                }
            }

            if !c.is_ascii_digit() || y + 1 == engine[x].len() {
                if is_part && !num.is_empty() {
                    let parsed: u32 = num
                        .parse()
                        .with_context(|| format!("Not a number: {num} ({c}: {x}/{y})"))?;

                    for loc in gears.iter() {
                        if gear_map.contains_key(loc) {
                            gear_map
                                .get_mut(loc)
                                .with_context(|| format!("No set for {loc:?}"))?
                                .insert(parsed);
                        } else {
                            let mut set = HashSet::new();
                            set.insert(parsed);
                            gear_map.insert(*loc, set);
                        }
                    }
                }

                num = String::new();
                is_part = false;
                gears.clear();
            }
        }
        // if is_part && !num.is_empty() {
        //     let parsed: u32 = num
        //         .parse()
        //         .with_context(|| format!("Not a number: {num} (_: {x}/$)"))?;
        //     part_numbers.push(parsed);
        // }
    }

    let result = gear_map
        .values()
        .filter(|parts| parts.len() == 2)
        .map(|parts| parts.iter().copied().reduce(|a, b| a * b).unwrap())
        .sum();

    Ok(result)
}

#[cfg(test)]
mod test {
    use crate::utils::load_input;

    use super::*;

    fn test_data() -> Vec<Vec<char>> {
        let input = "467..114..
        ...*......
        ..35..633.
        ......#...
        617*......
        .....+.58.
        ..592.....
        ......755.
        ...$.*....
        .664.598..
        ..........
        *..1.1.1.1
        1........*";

        load_input(input)
    }

    #[test]
    fn test_extract_part_numbers() {
        let engine = test_data();

        let actual = extract_part_numbers(&engine).unwrap();

        assert_eq!(actual, 467835);
    }
}
