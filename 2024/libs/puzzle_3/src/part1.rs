use std::{
    fs::File,
    io::{self, BufRead},
};

use anyhow::Result;

pub fn run(file: File) -> Result<usize> {
    let file = io::BufReader::new(file);

    Ok(0)
}


#[cfg(test)]
mod tests {
    use super::*;
    use test_log::test;

    #[test]
    fn test() {
        // assert_eq!(is_report_save(&mut vec![7, 6, 4, 2, 1]).unwrap(), true);
        // assert_eq!(is_report_save(&mut vec![1, 2, 7, 8, 9]).unwrap(), false);
        // assert_eq!(is_report_save(&mut vec![9, 7, 6, 2, 1]).unwrap(), false);
        // assert_eq!(is_report_save(&mut vec![1, 3, 2, 4, 5]).unwrap(), false);
        // assert_eq!(is_report_save(&mut vec![8, 6, 4, 4, 1]).unwrap(), false);
        // assert_eq!(is_report_save(&mut vec![1, 3, 6, 7, 9]).unwrap(), true);
    }
}
