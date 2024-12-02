use std::{
    fs::File,
    io::{self, BufRead},
};

use anyhow::Result;
use log::{debug, info};

pub fn run(file: File) -> Result<usize> {
    let file = io::BufReader::new(file);

    let result = file.lines().fold(0, |acc, line| {
        let line = line.unwrap();

        let words = line.split_whitespace();

        let report: Vec<_> = words.map(|w| w.parse::<usize>().unwrap()).collect();

        if is_report_save_cheat(report).unwrap() {
            acc + 1
        } else {
            acc
        }
    });
    Ok(result)
}

#[derive(Debug, PartialEq)]
enum ReportSafety {
    Save,
    Unsafe(usize),
}

fn is_report_save(report: &[usize]) -> Result<ReportSafety> {
    let len = report.len();
    let n = len - 1;

    let mut positive: Option<bool> = None;
    for index in 0..n {
        let diff = isize::try_from(report[index])? - isize::try_from(report[index + 1])?;

        if diff == 0 {
            return Ok(ReportSafety::Unsafe(index));
        }

        if let Some(positive) = positive {
            if diff > 0 && !positive {
                return Ok(ReportSafety::Unsafe(index));
            } else if diff < 0 && positive {
                return Ok(ReportSafety::Unsafe(index));
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
            return Ok(ReportSafety::Unsafe(index));
        }
    }

    Ok(ReportSafety::Save)
}

fn is_report_save_cheat(report: Vec<usize>) -> Result<bool> {
    let is_save = is_report_save(&report)?;

    match is_save {
        ReportSafety::Save => Ok(true),
        ReportSafety::Unsafe(index) => {
            let len = report.len();
            let mut cheat_reports: Vec<usize> = Vec::new();

            if index > 0 {
                let mut cheat_report = report.clone();
                cheat_report.remove(index - 1);
                let is_save = is_report_save(&cheat_report)?;

                if matches!(is_save, ReportSafety::Save) {
                    return Ok(true);
                }
            }

            {
                let mut cheat_report = report.clone();
                cheat_report.remove(index);
                let is_save = is_report_save(&cheat_report)?;

                if matches!(is_save, ReportSafety::Save) {
                    return Ok(true);
                }
            }

            if index + 1 < len {
                let mut cheat_report = report.clone();
                cheat_report.remove(index + 1);
                let is_save = is_report_save(&cheat_report)?;

                if matches!(is_save, ReportSafety::Save) {
                    return Ok(true);
                }
            }

            Ok(false)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_log::test;

    #[test]
    fn test_without_cheating() {
        assert_eq!(
            is_report_save(&mut vec![7, 6, 4, 2, 1]).unwrap(),
            ReportSafety::Save
        );
        assert_eq!(
            is_report_save(&mut vec![1, 2, 7, 8, 9]).unwrap(),
            ReportSafety::Unsafe(1)
        );
        assert_eq!(
            is_report_save(&mut vec![9, 7, 6, 2, 1]).unwrap(),
            ReportSafety::Unsafe(2)
        );
        assert_eq!(
            is_report_save(&mut vec![9, 7, 2, 1]).unwrap(),
            ReportSafety::Unsafe(1)
        );
        assert_eq!(
            is_report_save(&mut vec![1, 3, 2, 4, 5]).unwrap(),
            ReportSafety::Unsafe(1)
        );
        assert_eq!(
            is_report_save(&mut vec![1, 2, 4, 5]).unwrap(),
            ReportSafety::Save
        );
        assert_eq!(
            is_report_save(&mut vec![8, 6, 4, 4, 1]).unwrap(),
            ReportSafety::Unsafe(2)
        );
        assert_eq!(
            is_report_save(&mut vec![8, 6, 4, 1]).unwrap(),
            ReportSafety::Save
        );
        assert_eq!(
            is_report_save(&mut vec![1, 3, 6, 7, 9]).unwrap(),
            ReportSafety::Save
        );
    }
    #[test]
    fn test() {
        assert_eq!(is_report_save_cheat(vec![7, 6, 4, 2, 1]).unwrap(), true);
        assert_eq!(is_report_save_cheat(vec![1, 2, 7, 8, 9]).unwrap(), false);
        assert_eq!(is_report_save_cheat(vec![9, 7, 6, 2, 1]).unwrap(), false);
        assert_eq!(is_report_save_cheat(vec![1, 3, 2, 4, 5]).unwrap(), true);
        assert_eq!(is_report_save_cheat(vec![1, 3, 13, 4, 5]).unwrap(), true);
        assert_eq!(is_report_save_cheat(vec![5, 4, 13, 3, 1]).unwrap(), true);
        assert_eq!(is_report_save_cheat(vec![8, 6, 4, 4, 1]).unwrap(), true);
        assert_eq!(is_report_save_cheat(vec![1, 3, 6, 7, 9]).unwrap(), true);

        assert_eq!(
            is_report_save_cheat(vec![48, 46, 47, 49, 51, 54, 56]).unwrap(),
            true
        );
        assert_eq!(is_report_save_cheat(vec![1, 1, 2, 3, 4, 5]).unwrap(), true);
        assert_eq!(is_report_save_cheat(vec![1, 2, 3, 4, 5, 5]).unwrap(), true);
        assert_eq!(is_report_save_cheat(vec![5, 1, 2, 3, 4, 5]).unwrap(), true);
        assert_eq!(is_report_save_cheat(vec![1, 4, 3, 2, 1]).unwrap(), true);
        assert_eq!(is_report_save_cheat(vec![1, 6, 7, 8, 9]).unwrap(), true);
        assert_eq!(is_report_save_cheat(vec![1, 2, 3, 4, 3]).unwrap(), true);
        assert_eq!(is_report_save_cheat(vec![9, 8, 7, 6, 7]).unwrap(), true);
    }
}
