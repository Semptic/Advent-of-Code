use crate::utils::*;
use anyhow::{Context, Result};

pub fn extract_seeds(input: &str) -> Result<Vec<Seed>> {
    let raw_seeds = input
        .trim()
        .split(':')
        .last()
        .context("Failed to extract seeds")?;

    raw_seeds
        .split(' ')
        .map(|num| num.trim())
        .filter(|num| !num.is_empty())
        .map(|num| {
            num.parse()
                .map(Seed)
                .with_context(|| format!("Failed to parse {num}"))
        })
        .collect()
}
#[derive(Debug, PartialEq, Clone)]
pub struct PlantDetails {
    pub seed: Seed,
    pub soil: Soil,
    pub fertilizer: Fertilizer,
    pub water: Water,
    pub light: Light,
    pub temperature: Temperature,
    pub humidity: Humidity,
    pub location: Location,
}

fn get_details(almanac: &Almanac, seed: Seed) -> PlantDetails {
    let soil = almanac.soil(seed);
    let fertilizer = almanac.fertilizer(soil);
    let water = almanac.water(fertilizer);
    let light = almanac.light(water);
    let temperature = almanac.temperature(light);
    let humidity = almanac.humidity(temperature);
    let location = almanac.location(humidity);

    PlantDetails {
        seed,
        soil,
        fertilizer,
        water,
        light,
        temperature,
        humidity,
        location,
    }
}

pub fn get_lowest_location<I: Iterator<Item = Seed>>(
    seeds: I,
    almanac: &Almanac,
) -> Result<PlantDetails> {
    seeds
        .map(|seed| get_details(almanac, seed))
        .min_by_key(|details| details.location)
        .context("Failed to find lowest location")
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

    #[test]
    fn test_extract_seeds() {
        let input = "   seeds: 79 14 55 1213   ";

        assert_eq!(
            extract_seeds(input).unwrap(),
            vec![Seed(79), Seed(14), Seed(55), Seed(1213)]
        );
    }

    #[test]
    fn test_get_lowest_location() {
        let input = input();
        let seeds = extract_seeds(input.lines().next().unwrap()).unwrap();
        let lines: Vec<_> = input.lines().skip(1).collect();
        let almanac = parse_input(&lines).unwrap();

        let actual = get_lowest_location(seeds.into_iter(), &almanac).unwrap();

        assert_eq!(
            actual,
            PlantDetails {
                seed: Seed(13),
                soil: Soil(13),
                fertilizer: Fertilizer(52),
                water: Water(41),
                light: Light(34),
                temperature: Temperature(34),
                humidity: Humidity(35),
                location: Location(35),
            }
        );
    }
}
