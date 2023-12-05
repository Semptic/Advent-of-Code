use crate::utils::*;
use anyhow::{Context, Result};

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

fn get_details(almanac: &Almanac, seed: Seed) -> Result<PlantDetails> {
    let soil = almanac.soil(seed);
    let fertilizer = almanac.fertilizer(soil);
    let water = almanac.water(fertilizer);
    let light = almanac.light(water);
    let temperature = almanac.temperature(light);
    let humidity = almanac.humidity(temperature);
    let location = almanac.location(humidity);

    Ok(PlantDetails {
        seed,
        soil,
        fertilizer,
        water,
        light,
        temperature,
        humidity,
        location,
    })
}

pub fn get_lowest_location(almanac: &Almanac) -> Result<PlantDetails> {
    let details: Result<Vec<_>> = almanac
        .seeds
        .iter()
        .map(|seed| get_details(almanac, *seed))
        .collect();
    let details = details.context("Failed to extract details")?;

    details
        .into_iter()
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
    fn test_get_lowest_location() {
        let input = input();
        let almanac = parse_input(input).unwrap();

        let actual = get_lowest_location(&almanac).unwrap();

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
