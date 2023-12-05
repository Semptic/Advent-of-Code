use anyhow::{bail, Context, Result};
use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub struct Almanac {
    pub seeds: Vec<u32>,
    pub seed_to_soil: HashMap<u32, u32>,
    pub soil_to_fertilizer: HashMap<u32, u32>,
    pub fertilizer_to_water: HashMap<u32, u32>,
    pub water_to_light: HashMap<u32, u32>,
    pub light_to_temperature: HashMap<u32, u32>,
    pub temperature_to_humidity: HashMap<u32, u32>,
    pub humidity_to_location: HashMap<u32, u32>,
}

impl Almanac {
    fn soil(self: &Self, seed: &u32) -> u32 {
        self.seed_to_soil.get(seed).unwrap_or(seed).clone()
    }
    fn fertilizer(self: &Self, soil: &u32) -> u32 {
        self.soil_to_fertilizer.get(soil).unwrap_or(soil).clone()
    }
    fn water(self: &Self, fertilizer: &u32) -> u32 {
        self.fertilizer_to_water
            .get(fertilizer)
            .unwrap_or(fertilizer)
            .clone()
    }
    fn light(self: &Self, water: &u32) -> u32 {
        self.water_to_light.get(water).unwrap_or(water).clone()
    }
    fn temperature(self: &Self, light: &u32) -> u32 {
        self.light_to_temperature
            .get(light)
            .unwrap_or(light)
            .clone()
    }
    fn humidity(self: &Self, temperature: &u32) -> u32 {
        self.temperature_to_humidity
            .get(temperature)
            .unwrap_or(temperature)
            .clone()
    }
    fn location(self: &Self, humidity: &u32) -> u32 {
        self.humidity_to_location
            .get(humidity)
            .unwrap_or(humidity)
            .clone()
    }
}

fn extract_seeds(input: &str) -> Result<Vec<u32>> {
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
                .with_context(|| format!("Failed to parse {num}"))
        })
        .collect()
}

fn extract_map(line: &str) -> Result<HashMap<u32, u32>> {
    let trimmed = line.trim();

    if trimmed.is_empty() {
        bail!("Empty line");
    }

    let mut map = HashMap::new();

    let parts: Result<Vec<u32>> = trimmed
        .split(' ')
        .map(|num| num.trim())
        .filter(|num| !num.is_empty())
        .map(|num| {
            num.parse()
                .with_context(|| format!("Failed to parse {num}"))
        })
        .collect();

    let parts = parts.with_context(|| format!("Failed to parse {line}"))?;

    println!("parts: {parts:?}");
    if parts.len() != 3 {
        bail!("Invalid line: {}", line);
    }

    let destination_start = parts[0];
    let source_start = parts[1];
    let lenght = parts[2];

    for i in 0..lenght {
        map.insert(source_start + i, destination_start + i);
    }

    Ok(map)
}

pub fn parse_input(input: &str) -> Result<Almanac> {
    let seed_line = input.lines().next().context("Empty input")?;
    let seeds = extract_seeds(seed_line)?;

    let mut almanac = Almanac {
        seeds,
        seed_to_soil: HashMap::new(),
        soil_to_fertilizer: HashMap::new(),
        fertilizer_to_water: HashMap::new(),
        water_to_light: HashMap::new(),
        light_to_temperature: HashMap::new(),
        temperature_to_humidity: HashMap::new(),
        humidity_to_location: HashMap::new(),
    };

    let mut in_block = false;
    let mut target_map: Option<&mut HashMap<u32, u32>> = None;
    for line in input.lines().map(|line| line.trim()).skip(1) {
        if in_block {
            if line.is_empty() {
                in_block = false;
                target_map = None;
            } else if let Some(map) = target_map {
                let extraced = extract_map(line)?;

                map.extend(extraced);
                target_map = Some(map);
            }
        } else if line.contains("seed-to-soil map") {
            println!("seed-to-soil");
            in_block = true;
            target_map = Some(&mut almanac.seed_to_soil);
        } else if line.contains("soil-to-fertilizer map") {
            println!("soil-to-fertilizer");
            in_block = true;
            target_map = Some(&mut almanac.soil_to_fertilizer);
        } else if line.contains("fertilizer-to-water map") {
            println!("fertilizer-to-water");
            in_block = true;
            target_map = Some(&mut almanac.fertilizer_to_water);
        } else if line.contains("water-to-light map") {
            println!("water-to-light");
            in_block = true;
            target_map = Some(&mut almanac.water_to_light);
        } else if line.contains("light-to-temperature map") {
            println!("light-to-temperature");
            in_block = true;
            target_map = Some(&mut almanac.light_to_temperature);
        } else if line.contains("temperature-to-humidity map") {
            println!("temperature-to-humidity");
            in_block = true;
            target_map = Some(&mut almanac.temperature_to_humidity);
        } else if line.contains("humidity-to-location map") {
            println!("humidity-to-location");
            in_block = true;
            target_map = Some(&mut almanac.humidity_to_location);
        }
    }

    Ok(almanac)
}

#[cfg(test)]
mod tests {
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

        let actual = extract_seeds(input).unwrap();

        let expected = vec![79, 14, 55, 1213];

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_extract_map() {
        assert_eq!(
            extract_map("50 98 2").unwrap(),
            vec![(98, 50), (99, 51)].into_iter().collect()
        );
        assert_eq!(
            extract_map("10 2 2").unwrap(),
            vec![(2, 10), (3, 11)].into_iter().collect()
        );
        assert_eq!(extract_map("121 199 80").unwrap().len(), 80);
    }

    #[test]
    fn test_parse_input() {
        let input = input();

        let actual = parse_input(input).unwrap();

        // Seed extraction
        assert_eq!(actual.seeds, vec![79, 14, 55, 13], "Seed extraction");

        // seed-to-soil
        assert_eq!(actual.seed_to_soil.len(), 50, "seed-to-soil");
        // 50 98 2
        assert_eq!(
            actual.seed_to_soil.get(&98),
            Some(&50),
            "seed-to-soil: 50 98 2 #1"
        );
        assert_eq!(
            actual.seed_to_soil.get(&99),
            Some(&51),
            "seed-to-soil: 50 98 2 #2"
        );
        // 52 50 48
        assert_eq!(
            actual.seed_to_soil.get(&50),
            Some(&52),
            "seed-to-soil: 52 50 48 #1"
        );
        assert_eq!(
            actual.seed_to_soil.get(&97),
            Some(&99),
            "seed-to-soil: 52 50 48 #2"
        );

        // soil-to-fertilizer
        assert_eq!(actual.soil_to_fertilizer.len(), 54, "soil-to-fertilizer");
        // 0 15 37
        assert_eq!(
            actual.soil_to_fertilizer.get(&15),
            Some(&0),
            "soil-to-fertilizer: 0 15 37 #1"
        );
        assert_eq!(
            actual.soil_to_fertilizer.get(&20),
            Some(&5),
            "soil-to-fertilizer: 0 15 37 #2"
        );
        assert_eq!(
            actual.soil_to_fertilizer.get(&51),
            Some(&36),
            "soil-to-fertilizer: 0 15 37 #3"
        );
        // 37 52 2
        assert_eq!(
            actual.soil_to_fertilizer.get(&52),
            Some(&37),
            "soil-to-fertilizer: 37 52 2 #1"
        );
        assert_eq!(
            actual.soil_to_fertilizer.get(&53),
            Some(&38),
            "soil-to-fertilizer: 37 52 2 #2"
        );
        // 39 0 15
        assert_eq!(
            actual.soil_to_fertilizer.get(&0),
            Some(&39),
            "soil-to-fertilizer: 39 0 15 #1"
        );
        assert_eq!(
            actual.soil_to_fertilizer.get(&10),
            Some(&49),
            "soil-to-fertilizer: 39 0 15 #2"
        );
        assert_eq!(
            actual.soil_to_fertilizer.get(&14),
            Some(&53),
            "soil-to-fertilizer: 39 0 15 #3"
        );

        // fertilizer-to-water
        assert_eq!(actual.fertilizer_to_water.len(), 61, "fertilizer-to-water");

        // water-to-light
        assert_eq!(actual.water_to_light.len(), 77, "water-to-light");

        // light-to-temperature
        assert_eq!(
            actual.light_to_temperature.len(),
            55,
            "light-to-temperature"
        );

        // temperature-to-humidity
        assert_eq!(
            actual.temperature_to_humidity.len(),
            70,
            "temperature-to-humidity"
        );

        // humidity-to-location
        assert_eq!(
            actual.humidity_to_location.len(),
            41,
            "humidity-to-location"
        );
    }
    
    #[test]
    fn test_getter() {
        let almanac = Almanac {
            seeds: vec![1, 2, 3],
            seed_to_soil: vec![(1, 100)].into_iter().collect(),
            soil_to_fertilizer: vec![(1, 200)].into_iter().collect(),
            fertilizer_to_water: vec![(1, 300)].into_iter().collect(),
            water_to_light: vec![(1, 350)].into_iter().collect(),
            light_to_temperature: vec![(1, 500)].into_iter().collect(),
            temperature_to_humidity: vec![(1, 600)].into_iter().collect(),
            humidity_to_location: vec![(1, 700)].into_iter().collect(),
        };

        assert_eq!(almanac.soil(&1), 100);
        assert_eq!(almanac.soil(&2), 2);

        assert_eq!(almanac.fertilizer(&1), 200);
        assert_eq!(almanac.fertilizer(&2), 2);

        assert_eq!(almanac.water(&1), 300);
        assert_eq!(almanac.water(&2), 2);

        assert_eq!(almanac.light(&1), 350);
        assert_eq!(almanac.light(&2), 2);

        assert_eq!(almanac.temperature(&1), 500);
        assert_eq!(almanac.temperature(&2), 2);

        assert_eq!(almanac.humidity(&1), 600);
        assert_eq!(almanac.humidity(&2), 2);

        assert_eq!(almanac.location(&1), 700);
        assert_eq!(almanac.location(&2), 2);
    }
}
