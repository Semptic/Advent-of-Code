use anyhow::{bail, Context, Result};
use derive_more::Display;
use std::collections::HashMap;

#[derive(Debug, Display)]
enum BlockType {
    SeedToSoil,
    SoilToFertilizer,
    FertilizerToWater,
    WaterToLight,
    LightToTemperature,
    TemperatureToHumidity,
    HumidityToLocation,
}

#[derive(Debug, Display, Clone, Copy, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub struct Seed(u32);

impl From<u32> for Seed {
    fn from(num: u32) -> Self {
        Seed(num)
    }
}

#[derive(Debug, Display, Clone, Copy, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub struct Soil(u32);

impl From<u32> for Soil {
    fn from(num: u32) -> Self {
        Soil(num)
    }
}

impl From<Seed> for Soil {
    fn from(seed: Seed) -> Self {
        Soil(seed.0)
    }
}

#[derive(Debug, Display, Clone, Copy, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub struct Fertilizer(u32);

impl From<u32> for Fertilizer {
    fn from(num: u32) -> Self {
        Fertilizer(num)
    }
}

impl From<Soil> for Fertilizer {
    fn from(soil: Soil) -> Self {
        Fertilizer(soil.0)
    }
}

#[derive(Debug, Display, Clone, Copy, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub struct Water(u32);

impl From<u32> for Water {
    fn from(num: u32) -> Self {
        Water(num)
    }
}

impl From<Fertilizer> for Water {
    fn from(fertilizer: Fertilizer) -> Self {
        Water(fertilizer.0)
    }
}
#[derive(Debug, Display, Clone, Copy, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub struct Light(u32);

impl From<u32> for Light {
    fn from(num: u32) -> Self {
        Light(num)
    }
}

impl From<Water> for Light {
    fn from(water: Water) -> Self {
        Light(water.0)
    }
}

#[derive(Debug, Display, Clone, Copy, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub struct Temperature(u32);
impl From<u32> for Temperature {
    fn from(num: u32) -> Self {
        Temperature(num)
    }
}

impl From<Light> for Temperature {
    fn from(light: Light) -> Self {
        Temperature(light.0)
    }
}

#[derive(Debug, Display, Clone, Copy, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub struct Humidity(u32);

impl From<u32> for Humidity {
    fn from(num: u32) -> Self {
        Humidity(num)
    }
}

impl From<Temperature> for Humidity {
    fn from(temperature: Temperature) -> Self {
        Humidity(temperature.0)
    }
}
#[derive(Debug, Display, Clone, Copy, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub struct Location(u32);

impl From<u32> for Location {
    fn from(num: u32) -> Self {
        Location(num)
    }
}

impl From<Humidity> for Location {
    fn from(humidity: Humidity) -> Self {
        Location(humidity.0)
    }
}

#[derive(Debug, PartialEq)]
pub struct Almanac {
    pub seeds: Vec<Seed>,
    seed_to_soil: HashMap<Seed, Soil>,
    soil_to_fertilizer: HashMap<Soil, Fertilizer>,
    fertilizer_to_water: HashMap<Fertilizer, Water>,
    water_to_light: HashMap<Water, Light>,
    light_to_temperature: HashMap<Light, Temperature>,
    temperature_to_humidity: HashMap<Temperature, Humidity>,
    humidity_to_location: HashMap<Humidity, Location>,
}

impl Almanac {
    pub fn soil(&self, seed: Seed) -> Soil {
        self.seed_to_soil.get(&seed).cloned().unwrap_or(seed.into())
    }
    pub fn fertilizer(&self, soil: Soil) -> Fertilizer {
        self.soil_to_fertilizer
            .get(&soil)
            .cloned()
            .unwrap_or(soil.into())
    }
    pub fn water(&self, fertilizer: Fertilizer) -> Water {
        self.fertilizer_to_water
            .get(&fertilizer)
            .cloned()
            .unwrap_or(fertilizer.into())
    }
    pub fn light(&self, water: Water) -> Light {
        self.water_to_light
            .get(&water)
            .cloned()
            .unwrap_or(water.into())
    }
    pub fn temperature(&self, light: Light) -> Temperature {
        self.light_to_temperature
            .get(&light)
            .cloned()
            .unwrap_or(light.into())
    }
    pub fn humidity(&self, temperature: Temperature) -> Humidity {
        self.temperature_to_humidity
            .get(&temperature)
            .cloned()
            .unwrap_or(temperature.into())
    }
    pub fn location(&self, humidity: Humidity) -> Location {
        self.humidity_to_location
            .get(&humidity)
            .cloned()
            .unwrap_or(humidity.into())
    }
}

fn extract_seeds(input: &str) -> Result<Vec<Seed>> {
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

    let mut seed_to_soil = HashMap::new();
    let mut soil_to_fertilizer = HashMap::new();
    let mut fertilizer_to_water = HashMap::new();
    let mut water_to_light = HashMap::new();
    let mut light_to_temperature = HashMap::new();
    let mut temperature_to_humidity = HashMap::new();
    let mut humidity_to_location = HashMap::new();

    let mut block: Option<BlockType> = None;
    for line in input.lines().map(|line| line.trim()).skip(1) {
        if block.is_some() {
            if line.is_empty() {
                block = None;
            } else if let Some(block_type) = &block {
                let map = extract_map(line)?;
                match block_type {
                    BlockType::SeedToSoil => seed_to_soil.extend(map),
                    BlockType::SoilToFertilizer => soil_to_fertilizer.extend(map),
                    BlockType::FertilizerToWater => fertilizer_to_water.extend(map),
                    BlockType::WaterToLight => water_to_light.extend(map),
                    BlockType::LightToTemperature => light_to_temperature.extend(map),
                    BlockType::TemperatureToHumidity => temperature_to_humidity.extend(map),
                    BlockType::HumidityToLocation => humidity_to_location.extend(map),
                }
            }
        } else if line.contains("seed-to-soil map") {
            println!("seed-to-soil");
            block = Some(BlockType::SeedToSoil);
        } else if line.contains("soil-to-fertilizer map") {
            println!("soil-to-fertilizer");
            block = Some(BlockType::SoilToFertilizer);
        } else if line.contains("fertilizer-to-water map") {
            println!("fertilizer-to-water");
            block = Some(BlockType::FertilizerToWater);
        } else if line.contains("water-to-light map") {
            println!("water-to-light");
            block = Some(BlockType::WaterToLight);
        } else if line.contains("light-to-temperature map") {
            println!("light-to-temperature");
            block = Some(BlockType::LightToTemperature);
        } else if line.contains("temperature-to-humidity map") {
            println!("temperature-to-humidity");
            block = Some(BlockType::TemperatureToHumidity);
        } else if line.contains("humidity-to-location map") {
            println!("humidity-to-location");
            block = Some(BlockType::HumidityToLocation);
        }
    }

    Ok(Almanac {
        seeds,
        seed_to_soil: seed_to_soil
            .into_iter()
            .map(|(k, v)| (k.into(), v.into()))
            .collect(),
        soil_to_fertilizer: soil_to_fertilizer
            .into_iter()
            .map(|(k, v)| (k.into(), v.into()))
            .collect(),
        fertilizer_to_water: fertilizer_to_water
            .into_iter()
            .map(|(k, v)| (k.into(), v.into()))
            .collect(),
        water_to_light: water_to_light
            .into_iter()
            .map(|(k, v)| (k.into(), v.into()))
            .collect(),
        light_to_temperature: light_to_temperature
            .into_iter()
            .map(|(k, v)| (k.into(), v.into()))
            .collect(),
        temperature_to_humidity: temperature_to_humidity
            .into_iter()
            .map(|(k, v)| (k.into(), v.into()))
            .collect(),
        humidity_to_location: humidity_to_location
            .into_iter()
            .map(|(k, v)| (k.into(), v.into()))
            .collect(),
    })
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

        assert_eq!(
            extract_seeds(input).unwrap(),
            vec![Seed(79), Seed(14), Seed(55), Seed(1213)]
        );
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
        assert_eq!(
            actual.seeds,
            vec![Seed(79), Seed(14), Seed(55), Seed(13)],
            "Seed extraction"
        );

        // seed-to-soil
        assert_eq!(actual.seed_to_soil.len(), 50, "seed-to-soil");
        // 50 98 2
        assert_eq!(
            actual.seed_to_soil.get(&Seed(98)),
            Some(&Soil(50)),
            "seed-to-soil: 50 98 2 #1"
        );
        assert_eq!(
            actual.seed_to_soil.get(&Seed(99)),
            Some(&Soil(51)),
            "seed-to-soil: 50 98 2 #2"
        );
        // 52 50 48
        assert_eq!(
            actual.seed_to_soil.get(&Seed(50)),
            Some(&Soil(52)),
            "seed-to-soil: 52 50 48 #1"
        );
        assert_eq!(
            actual.seed_to_soil.get(&Seed(97)),
            Some(&Soil(99)),
            "seed-to-soil: 52 50 48 #2"
        );

        // soil-to-fertilizer
        assert_eq!(actual.soil_to_fertilizer.len(), 54, "soil-to-fertilizer");
        // 0 15 37
        assert_eq!(
            actual.soil_to_fertilizer.get(&Soil(15)),
            Some(&Fertilizer(0)),
            "soil-to-fertilizer: 0 15 37 #1"
        );
        assert_eq!(
            actual.soil_to_fertilizer.get(&Soil(20)),
            Some(&Fertilizer(5)),
            "soil-to-fertilizer: 0 15 37 #2"
        );
        assert_eq!(
            actual.soil_to_fertilizer.get(&Soil(51)),
            Some(&Fertilizer(36)),
            "soil-to-fertilizer: 0 15 37 #3"
        );
        // 37 52 2
        assert_eq!(
            actual.soil_to_fertilizer.get(&Soil(52)),
            Some(&Fertilizer(37)),
            "soil-to-fertilizer: 37 52 2 #1"
        );
        assert_eq!(
            actual.soil_to_fertilizer.get(&Soil(53)),
            Some(&Fertilizer(38)),
            "soil-to-fertilizer: 37 52 2 #2"
        );
        // 39 0 15
        assert_eq!(
            actual.soil_to_fertilizer.get(&Soil(0)),
            Some(&Fertilizer(39)),
            "soil-to-fertilizer: 39 0 15 #1"
        );
        assert_eq!(
            actual.soil_to_fertilizer.get(&Soil(10)),
            Some(&Fertilizer(49)),
            "soil-to-fertilizer: 39 0 15 #2"
        );
        assert_eq!(
            actual.soil_to_fertilizer.get(&Soil(14)),
            Some(&Fertilizer(53)),
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
            seeds: vec![Seed(1), Seed(2), Seed(3)],
            seed_to_soil: vec![(Seed(1), Soil(100))].into_iter().collect(),
            soil_to_fertilizer: vec![(Soil(1), Fertilizer(200))].into_iter().collect(),
            fertilizer_to_water: vec![(Fertilizer(1), Water(300))].into_iter().collect(),
            water_to_light: vec![(Water(1), Light(350))].into_iter().collect(),
            light_to_temperature: vec![(Light(1), Temperature(500))].into_iter().collect(),
            temperature_to_humidity: vec![(Temperature(1), Humidity(600))].into_iter().collect(),
            humidity_to_location: vec![(Humidity(1), Location(700))].into_iter().collect(),
        };

        assert_eq!(almanac.soil(Seed(1)), Soil(100));
        assert_eq!(almanac.soil(Seed(2)), Soil(2));

        assert_eq!(almanac.fertilizer(Soil(1)), Fertilizer(200));
        assert_eq!(almanac.fertilizer(Soil(2)), Fertilizer(2));

        assert_eq!(almanac.water(Fertilizer(1)), Water(300));
        assert_eq!(almanac.water(Fertilizer(2)), Water(2));

        assert_eq!(almanac.light(Water(1)), Light(350));
        assert_eq!(almanac.light(Water(2)), Light(2));

        assert_eq!(almanac.temperature(Light(1)), Temperature(500));
        assert_eq!(almanac.temperature(Light(2)), Temperature(2));

        assert_eq!(almanac.humidity(Temperature(1)), Humidity(600));
        assert_eq!(almanac.humidity(Temperature(2)), Humidity(2));

        assert_eq!(almanac.location(Humidity(1)), Location(700));
        assert_eq!(almanac.location(Humidity(2)), Location(2));
    }
}
