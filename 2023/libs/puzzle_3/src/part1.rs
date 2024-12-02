use anyhow::{Context, Result};

fn is_symbol(c: char) -> bool {
    !c.is_ascii_digit() && c != '.'
}

pub fn extract_part_numbers(engine: &Vec<Vec<char>>) -> Result<Vec<u32>> {
    let mut part_numbers = Vec::new();

    for x in 0..engine.len() {
        let mut num = String::new();
        let mut is_part = false;

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
                            continue;
                        }
                        // center
                        if is_symbol(engine[x - 1][y]) {
                            is_part = true;
                            continue;
                        }
                        //right
                        if y + 1 < engine[x].len() && is_symbol(engine[x - 1][y + 1]) {
                            is_part = true;
                            continue;
                        }
                    }

                    // same line
                    // left
                    if y > 0 && is_symbol(engine[x][y - 1]) {
                        is_part = true;
                        continue;
                    }
                    // center
                    // if is_symbol(engine[x][y]) {
                    //     is_part = true;
                    // }
                    // right
                    if y + 1 < engine[x].len() && is_symbol(engine[x][y + 1]) {
                        is_part = true;
                        continue;
                    }

                    // below
                    if x + 1 < engine.len() {
                        // left
                        if y > 0 && is_symbol(engine[x + 1][y - 1]) {
                            is_part = true;
                            continue;
                        }
                        // center
                        if is_symbol(engine[x + 1][y]) {
                            is_part = true;
                            continue;
                        }
                        // right
                        if y + 1 < engine[x].len() && is_symbol(engine[x + 1][y + 1]) {
                            is_part = true;
                            continue;
                        }
                    }
                }
            } else {
                if is_part && !num.is_empty() {
                    let parsed: u32 = num
                        .parse()
                        .with_context(|| format!("Not a number: {num} ({c}: {x}/{y})"))?;
                    part_numbers.push(parsed);
                }

                num = String::new();
                is_part = false;
            }
        }
        if is_part && !num.is_empty() {
            let parsed: u32 = num
                .parse()
                .with_context(|| format!("Not a number: {num} (_: {x}/$)"))?;
            part_numbers.push(parsed);
        }
    }

    Ok(part_numbers)
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

        assert_eq!(actual, vec![467, 35, 633, 617, 592, 755, 664, 598, 1, 1]);
    }
}
