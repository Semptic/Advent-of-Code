use anyhow::{Context, Result};
use log::debug;
use regex::Regex;

fn str_to_num(input: &str) -> Result<u32> {
    match input {
        "one" => Ok(1),
        "eno" => Ok(1),
        "two" => Ok(2),
        "owt" => Ok(2),
        "three" => Ok(3),
        "eerht" => Ok(3),
        "four" => Ok(4),
        "ruof" => Ok(4),
        "five" => Ok(5),
        "evif" => Ok(5),
        "six" => Ok(6),
        "xis" => Ok(6),
        "seven" => Ok(7),
        "neves" => Ok(7),
        "eight" => Ok(8),
        "thgie" => Ok(8),
        "nine" => Ok(9),
        "enin" => Ok(9),
        other => other
            .parse()
            .with_context(|| format!("{input} is not a number")),
    }
}

fn extract_first_calibration_value(input: &str) -> Result<u32> {
    let re = Regex::new("([0-9]|one|two|three|four|five|six|seven|eight|nine)")?;

    let matched = input.match_indices(&re).collect::<Vec<_>>();

    let first_match = matched.first().context("Failed to find first match")?.1;

    debug!("input: {input}, matched: {:?}", matched);

    let first = str_to_num(first_match)?;

    Ok(first * 10)

}

fn extract_last_calibration_value(input: &str) -> Result<u32> {
    let re = Regex::new("([0-9]|eno|owt|eerht|ruof|evif|xis|neves|thgie|enin)")?;

    let input_rev: String = input.chars().rev().collect();
    let matched = input_rev.match_indices(&re).collect::<Vec<_>>();

    let last_match = matched.first().context("Failed to find first match")?.1;

    debug!("input: {input_rev}, matched: {:?}", matched);

    let last = str_to_num(last_match)?;

    Ok(last)
}

fn extract_calibration_value(input: &str) -> Result<u32> {
    let first = extract_first_calibration_value(input)?;
    let last = extract_last_calibration_value(input)?;

    Ok(first + last)
}

pub fn sum_calibration_values(input: &str) -> Result<u32> {
    let mut sum = 0;

    for line in input.lines() {
        let value = extract_calibration_value(line)?;
        sum += value;
    }

    Ok(sum)
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_extract_calibration_value() {
        let test_data = vec![
            ("two1nine", 29),
            ("eightwothree", 83),
            ("abcone2threexyz", 13),
            ("xtwone3four", 24),
            ("4nineeightseven2", 42),
            ("zoneight234", 14),
            ("7pqrstsixteen", 76),
            ("eighthree", 83), 
            ("twoneight", 28)
        ];

        for (input, expected) in test_data {
            let actual = super::extract_calibration_value(input).unwrap();
            assert_eq!(actual, expected);
        }
    }

    #[test]
    fn test_sum_calibration_values() {
        let input = "two1nine
           eightwothree
           abcone2threexyz
           xtwone3four
           4nineeightseven2
           zoneight234
           7pqrstsixteen";

        let actual = super::sum_calibration_values(input).unwrap();
        assert_eq!(actual, 281);
    }
}
