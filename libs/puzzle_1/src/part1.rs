use anyhow::{Context, Result};
use regex::Regex;

fn extract_calibration_value(input: &str) -> Result<u32> {
    let re = Regex::new("[0-9]")?;

    let matched = input.match_indices(&re).collect::<Vec<_>>();

    let first: u32 = matched
        .first()
        .context("Failed to find first match")?
        .1
        .parse()
        .context("First match is not a number")?;
    let last: u32 = matched
        .last()
        .context("Failed to find last match")?
        .1
        .parse()
        .context("First match is not a number")?;

    Ok(first * 10 + last)
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
            ("1abc2", 12),
            ("pqr3stu8vwx", 38),
            ("a1b2c3d4e5f", 15),
            ("treb7uchet", 77),
        ];

        for (input, expected) in test_data {
            let actual = super::extract_calibration_value(input).unwrap();
            assert_eq!(actual, expected);
        }
    }

    #[test]
    fn test_sum_calibration_values() {
        let input = "1abc2
        pqr3stu8vwx
        a1b2c3d4e5f
        treb7uchet";

        let actual = super::sum_calibration_values(input).unwrap();
        assert_eq!(actual, 142);
    }
}
