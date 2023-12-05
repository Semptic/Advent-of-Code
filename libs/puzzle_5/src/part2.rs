use crate::utils::*;
use anyhow::{Context, Result};
use log::debug;

#[derive(Debug, PartialEq)]
pub struct SeedRange {
    start: Id,
    length: Id,
}

// impl<S: Into<Id>, D: From<Id> + From<S>> Range<S, D> {
//     pub fn intersects<SS: Into<Id>>(&self, range: Range<SS, S>) -> bool
//     where
//         S: From<SS> + From<Id>,
//     {
//         (range.destination >= self.source && range.destination < self.source + self.length)
//             || (range.destination + range.length - 1 >= self.source
//                 && range.destination + range.length - 1 < self.source + self.length)
//     }
// }

pub struct SeedRangeIterator {
    current: Id,
    stop: Id,
}

impl Iterator for SeedRangeIterator {
    type Item = Seed;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current > self.stop {
            return None;
        }

        let result = self.current;
        self.current += 1;
        Some(result.into())
    }
}

impl IntoIterator for SeedRange {
    type Item = Seed;
    type IntoIter = SeedRangeIterator;

    fn into_iter(self) -> Self::IntoIter {
        debug!(
            "IntoIterator for SeedRange({}, {})",
            self.start, self.length
        );
        SeedRangeIterator {
            current: self.start,
            stop: self.start + self.length - 1,
        }
    }
}

pub fn extract_seed_ranges(input: &str) -> Result<Vec<SeedRange>> {
    let raw_seeds: Vec<_> = input
        .trim()
        .split(':')
        .last()
        .context("Failed to extract seeds")?
        .split(' ')
        .map(|num| num.trim())
        .filter(|num| !num.is_empty())
        .collect();

    let mut seed_ranges = Vec::new();

    for i in (1..raw_seeds.len()).step_by(2) {
        let start = raw_seeds[i - 1].parse()?;
        let length = raw_seeds[i].parse()?;

        seed_ranges.push(SeedRange { start, length })
    }

    Ok(seed_ranges)
}
#[cfg(test)]
mod test {
    use super::*;

    fn input() -> &'static str {
        "seeds: 79 14 55 13

         seed-to-soil map:
         50 98 2
         52 50 48
         
         soil-to-fertilizer map:
         0 15 37
         37 52 2
         39 0 15
         
         fertilizer-to-water map:
         49 53 8
         0 11 42
         42 0 7
         57 7 4
         
         water-to-light map:
         88 18 7
         18 25 70
         
         light-to-temperature map:
         45 77 23
         81 45 19
         68 64 13
         
         temperature-to-humidity map:
         0 69 1
         1 0 69
         
         humidity-to-location map:
         60 56 37
         56 93 4"
            .trim()
    }

    // #[test]
    // fn test_range_intersects() {
    //     assert!(Range::<Seed, Soil>::new(0, 10, 10).intersects(Range::<Seed, Seed>::new(0, 9, 10)));
    //     assert!(Range::<Seed, Soil>::new(0, 10, 1).intersects(Range::<Seed, Seed>::new(0, 0, 10)));
    //     assert_eq!(
    //         Range::<Seed, Soil>::new(0, 10, 1).intersects(Range::<Seed, Seed>::new(0, 9, 1)),
    //         false
    //     );
    // }

    #[test]
    fn test_seed_range_extraction() {
        let seed_line = input().lines().next().unwrap();

        let actual = extract_seed_ranges(seed_line).unwrap();

        assert_eq!(
            actual,
            vec![
                SeedRange {
                    start: 79,
                    length: 14
                },
                SeedRange {
                    start: 55,
                    length: 13
                }
            ]
        );
    }

    #[test]
    fn test_iterate_seed_range() {
        let seed_range = SeedRange {
            start: 79,
            length: 5,
        };
        let mut iter = seed_range.into_iter();

        assert_eq!(iter.next(), Some(Seed(79)));
        assert_eq!(iter.next(), Some(Seed(80)));
        assert_eq!(iter.next(), Some(Seed(81)));
        assert_eq!(iter.next(), Some(Seed(82)));
        assert_eq!(iter.next(), Some(Seed(83)));
        assert_eq!(iter.next(), None);
    }
}
