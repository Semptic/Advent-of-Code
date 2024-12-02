use std::{fs::File, io::{self, BufRead}};

use anyhow::Result;
use log::{debug, info};

pub fn run(file: File) -> Result<usize> {
    let file = io::BufReader::new(file);

    let result = file.lines().fold(0, |acc, line| {
        let line = line.unwrap();

        let words = line.split_whitespace();

        let report: Vec<_> = words.map(|w| w.parse::<usize>().unwrap()).collect();

        if is_report_save(&report).unwrap() {
            acc + 1
        } else {
            acc
        }

    });
    Ok(result)
}

 fn is_report_save(report: &[usize]) -> Result<bool> {
    let len = report.len();
    let n = len - 1;

    let mut positive: Option<bool> = None;
    for index in 0..n {
            let diff = isize::try_from(report[index])? - isize::try_from(report[index + 1])?;

            if diff == 0 {
                return Ok(false);
            }

            if let Some(positive) = positive {
                if diff > 0 && !positive {
                   return Ok(false);
                } else if diff < 0 && positive {
                   return Ok(false);
                }
            } else { 
                if diff > 0 {
                   positive = Some(true);
                } else {
                   positive = Some(false);
                }
            }
                
            let diff = diff.abs(); 

            if diff < 1 || diff > 3 {
                return Ok(false)
            }
    }

    Ok(true)
}



#[cfg(test)]
mod tests {
    use test_log::test;
    use super::*;

    #[test]
    fn test() {
        assert_eq!(is_report_save(&mut vec![7, 6, 4, 2, 1]).unwrap(), true);
        assert_eq!(is_report_save(&mut vec![1, 2, 7, 8, 9]).unwrap(), false);
        assert_eq!(is_report_save(&mut vec![9, 7, 6, 2, 1]).unwrap(), false);
        assert_eq!(is_report_save(&mut vec![1, 3, 2, 4, 5]).unwrap(), false);
        assert_eq!(is_report_save(&mut vec![8, 6, 4, 4, 1]).unwrap(), false);
        assert_eq!(is_report_save(&mut vec![1, 3, 6, 7, 9]).unwrap(), true);
    }
}
