use anyhow::{bail, Context, Result};
use derive_more::Display;
use std::fmt::Debug;

pub type Id = u64;

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
pub struct Seed(pub Id);

impl From<Id> for Seed {
    fn from(val: Id) -> Self {
        Seed(val)
    }
}
impl From<Seed> for Id {
    fn from(val: Seed) -> Self {
        val.0
    }
}

#[derive(Debug, Display, Clone, Copy, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub struct Soil(pub Id);

impl From<Id> for Soil {
    fn from(val: Id) -> Self {
        Soil(val)
    }
}
impl From<Soil> for Id {
    fn from(val: Soil) -> Self {
        val.0
    }
}
impl From<Seed> for Soil {
    fn from(seed: Seed) -> Self {
        Soil(seed.0)
    }
}

#[derive(Debug, Display, Clone, Copy, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub struct Fertilizer(pub Id);

impl From<Id> for Fertilizer {
    fn from(val: Id) -> Self {
        Fertilizer(val)
    }
}
impl From<Fertilizer> for Id {
    fn from(val: Fertilizer) -> Self {
        val.0
    }
}
impl From<Soil> for Fertilizer {
    fn from(soil: Soil) -> Self {
        Fertilizer(soil.0)
    }
}

#[derive(Debug, Display, Clone, Copy, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub struct Water(pub Id);

impl From<Id> for Water {
    fn from(val: Id) -> Self {
        Water(val)
    }
}
impl From<Water> for Id {
    fn from(val: Water) -> Self {
        val.0
    }
}
impl From<Fertilizer> for Water {
    fn from(fertilizer: Fertilizer) -> Self {
        Water(fertilizer.0)
    }
}

#[derive(Debug, Display, Clone, Copy, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub struct Light(pub Id);

impl From<Id> for Light {
    fn from(val: Id) -> Self {
        Light(val)
    }
}
impl From<Light> for Id {
    fn from(val: Light) -> Self {
        val.0
    }
}
impl From<Water> for Light {
    fn from(water: Water) -> Self {
        Light(water.0)
    }
}

#[derive(Debug, Display, Clone, Copy, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub struct Temperature(pub Id);

impl From<Id> for Temperature {
    fn from(val: Id) -> Self {
        Temperature(val)
    }
}
impl From<Temperature> for Id {
    fn from(val: Temperature) -> Self {
        val.0
    }
}
impl From<Light> for Temperature {
    fn from(light: Light) -> Self {
        Temperature(light.0)
    }
}

#[derive(Debug, Display, Clone, Copy, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub struct Humidity(pub Id);

impl From<Id> for Humidity {
    fn from(val: Id) -> Self {
        Humidity(val)
    }
}
impl From<Humidity> for Id {
    fn from(val: Humidity) -> Self {
        val.0
    }
}
impl From<Temperature> for Humidity {
    fn from(temperature: Temperature) -> Self {
        Humidity(temperature.0)
    }
}
#[derive(Debug, Display, Clone, Copy, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub struct Location(pub Id);

impl From<Id> for Location {
    fn from(val: Id) -> Self {
        Location(val)
    }
}
impl From<Location> for Id {
    fn from(val: Location) -> Self {
        val.0
    }
}
impl From<Humidity> for Location {
    fn from(humidity: Humidity) -> Self {
        Location(humidity.0)
    }
}
#[derive(PartialEq)]
pub struct Range<S: Into<Id>, D: From<Id> + From<S>> {
    pub source: Id,
    pub destination: Id,
    pub length: Id,
    phantom: std::marker::PhantomData<(S, D)>,
}

impl<S: Into<Id>, D: From<Id> + From<S>> Debug for Range<S, D> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct(
            format!(
                "Range<{},{}>",
                std::any::type_name::<S>(),
                std::any::type_name::<D>()
            )
            .as_str(),
        )
        .field("source", &self.source)
        .field("destination", &self.destination)
        .field("length", &self.length)
        .finish()
    }
}

impl<S: Into<Id> + Copy, D: From<Id> + From<S>> Range<S, D> {
    pub fn new(source: Id, destination: Id, length: Id) -> Self {
        Self {
            source,
            destination,
            length,
            phantom: std::marker::PhantomData,
        }
    }
    pub fn contains(&self, source: S) -> bool {
        source.into() >= self.source && source.into() < self.source + self.length
    }

    pub fn get_destination(&self, source: S) -> Option<D> {
        if self.contains(source) {
            Some((source.into() - self.source + self.destination).into())
        } else {
            None
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Almanac {
    seed_to_soil: Vec<Range<Seed, Soil>>,
    soil_to_fertilizer: Vec<Range<Soil, Fertilizer>>,
    fertilizer_to_water: Vec<Range<Fertilizer, Water>>,
    water_to_light: Vec<Range<Water, Light>>,
    light_to_temperature: Vec<Range<Light, Temperature>>,
    temperature_to_humidity: Vec<Range<Temperature, Humidity>>,
    humidity_to_location: Vec<Range<Humidity, Location>>,
}

impl Almanac {
    pub fn soil(&self, seed: Seed) -> Soil {
        self.seed_to_soil
            .iter()
            .filter_map(|range| range.get_destination(seed))
            .next()
            .unwrap_or_else(|| seed.into())
    }
    pub fn fertilizer(&self, soil: Soil) -> Fertilizer {
        self.soil_to_fertilizer
            .iter()
            .filter_map(|range| range.get_destination(soil))
            .next()
            .unwrap_or_else(|| soil.into())
    }
    pub fn water(&self, fertilizer: Fertilizer) -> Water {
        self.fertilizer_to_water
            .iter()
            .filter_map(|range| range.get_destination(fertilizer))
            .next()
            .unwrap_or_else(|| fertilizer.into())
    }
    pub fn light(&self, water: Water) -> Light {
        self.water_to_light
            .iter()
            .filter_map(|range| range.get_destination(water))
            .next()
            .unwrap_or_else(|| water.into())
    }
    pub fn temperature(&self, light: Light) -> Temperature {
        self.light_to_temperature
            .iter()
            .filter_map(|range| range.get_destination(light))
            .next()
            .unwrap_or_else(|| light.into())
    }
    pub fn humidity(&self, temperature: Temperature) -> Humidity {
        self.temperature_to_humidity
            .iter()
            .filter_map(|range| range.get_destination(temperature))
            .next()
            .unwrap_or_else(|| temperature.into())
    }
    pub fn location(&self, humidity: Humidity) -> Location {
        self.humidity_to_location
            .iter()
            .filter_map(|range| range.get_destination(humidity))
            .next()
            .unwrap_or_else(|| humidity.into())
    }
}

fn extract_mapping<S: Into<Id> + Copy, D: From<Id> + From<S>>(line: &str) -> Result<Range<S, D>> {
    let trimmed = line.trim();

    if trimmed.is_empty() {
        bail!("Empty line");
    }

    let parts: Result<Vec<Id>> = trimmed
        .split(' ')
        .map(|num| num.trim())
        .filter(|num| !num.is_empty())
        .map(|num| {
            num.parse()
                .with_context(|| format!("Failed to parse {num}"))
        })
        .collect();

    let parts = parts.with_context(|| format!("Failed to parse {line}"))?;

    if parts.len() != 3 {
        bail!("Invalid line: {}", line);
    }

    let destination = parts[0];
    let source = parts[1];
    let length = parts[2];

    Ok(Range::<S, D>::new(source, destination, length))
}

pub fn parse_input(input: &[&str]) -> Result<Almanac> {
    let mut seed_to_soil = Vec::new();
    let mut soil_to_fertilizer = Vec::new();
    let mut fertilizer_to_water = Vec::new();
    let mut water_to_light = Vec::new();
    let mut light_to_temperature = Vec::new();
    let mut temperature_to_humidity = Vec::new();
    let mut humidity_to_location = Vec::new();

    let mut block: Option<BlockType> = None;
    for line in input {
        let line = line.trim();

        if block.is_some() {
            if line.is_empty() {
                block = None;
            } else if let Some(block_type) = &block {
                match block_type {
                    BlockType::SeedToSoil => seed_to_soil.push(extract_mapping(line)?),
                    BlockType::SoilToFertilizer => soil_to_fertilizer.push(extract_mapping(line)?),
                    BlockType::FertilizerToWater => {
                        fertilizer_to_water.push(extract_mapping(line)?)
                    }
                    BlockType::WaterToLight => water_to_light.push(extract_mapping(line)?),
                    BlockType::LightToTemperature => {
                        light_to_temperature.push(extract_mapping(line)?)
                    }
                    BlockType::TemperatureToHumidity => {
                        temperature_to_humidity.push(extract_mapping(line)?)
                    }
                    BlockType::HumidityToLocation => {
                        humidity_to_location.push(extract_mapping(line)?)
                    }
                }
            }
        } else if line.contains("seed-to-soil map") {
            block = Some(BlockType::SeedToSoil);
        } else if line.contains("soil-to-fertilizer map") {
            block = Some(BlockType::SoilToFertilizer);
        } else if line.contains("fertilizer-to-water map") {
            block = Some(BlockType::FertilizerToWater);
        } else if line.contains("water-to-light map") {
            block = Some(BlockType::WaterToLight);
        } else if line.contains("light-to-temperature map") {
            block = Some(BlockType::LightToTemperature);
        } else if line.contains("temperature-to-humidity map") {
            block = Some(BlockType::TemperatureToHumidity);
        } else if line.contains("humidity-to-location map") {
            block = Some(BlockType::HumidityToLocation);
        }
    }

    seed_to_soil.sort_by_key(|range| range.source);
    soil_to_fertilizer.sort_by_key(|range| range.source);
    fertilizer_to_water.sort_by_key(|range| range.source);
    water_to_light.sort_by_key(|range| range.source);
    light_to_temperature.sort_by_key(|range| range.source);
    temperature_to_humidity.sort_by_key(|range| range.source);
    humidity_to_location.sort_by_key(|range| range.source);

    Ok(Almanac {
        seed_to_soil,
        soil_to_fertilizer,
        fertilizer_to_water,
        water_to_light,
        light_to_temperature,
        temperature_to_humidity,
        humidity_to_location,
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
    fn test_extract_mapping() {
        assert_eq!(
            extract_mapping::<Seed, Soil>("50 98 2").unwrap(),
            Range::<Seed, Soil>::new(98, 50, 2)
        );
        assert_eq!(
            extract_mapping::<Water, Light>("10 2 2").unwrap(),
            Range::<Water, Light>::new(2, 10, 2)
        );
        assert_eq!(
            extract_mapping::<Humidity, Location>("121 199 80").unwrap(),
            Range::<Humidity, Location>::new(199, 121, 80)
        );
    }

    #[test]
    fn test_parse_input() {
        let input = input();
        let lines: Vec<_> = input.lines().skip(1).collect();

        let actual = parse_input(&lines).unwrap();

        // seed-to-soil
        // 50 98 2
        // 52 50 48
        assert_eq!(
            actual.seed_to_soil,
            vec![
                Range::<Seed, Soil>::new(50, 52, 48),
                Range::<Seed, Soil>::new(98, 50, 2),
            ],
        );

        // soil-to-fertilizer
        // 0 15 37
        // 37 52 2
        // 39 0 15
        assert_eq!(
            actual.soil_to_fertilizer,
            vec![
                Range::<Soil, Fertilizer>::new(0, 39, 15),
                Range::<Soil, Fertilizer>::new(15, 0, 37),
                Range::<Soil, Fertilizer>::new(52, 37, 2),
            ],
        );

        // fertilizer-to-water
        // 49 53 8
        // 0 11 42
        // 42 0 7
        // 57 7 4
        assert_eq!(
            actual.fertilizer_to_water,
            vec![
                Range::<Fertilizer, Water>::new(0, 42, 7),
                Range::<Fertilizer, Water>::new(7, 57, 4),
                Range::<Fertilizer, Water>::new(11, 0, 42),
                Range::<Fertilizer, Water>::new(53, 49, 8),
            ],
        );

        // water-to-light
        // 88 18 7
        // 18 25 70
        assert_eq!(
            actual.water_to_light,
            vec![
                Range::<Water, Light>::new(18, 88, 7),
                Range::<Water, Light>::new(25, 18, 70),
            ],
        );

        // light-to-temperature
        // 45 77 23
        // 81 45 19
        // 68 64 13
        assert_eq!(
            actual.light_to_temperature,
            vec![
                Range::<Light, Temperature>::new(45, 81, 19),
                Range::<Light, Temperature>::new(64, 68, 13),
                Range::<Light, Temperature>::new(77, 45, 23),
            ],
        );

        // temperature-to-humidity
        //  0 69 1
        //  1 0 69
        assert_eq!(
            actual.temperature_to_humidity,
            vec![
                Range::<Temperature, Humidity>::new(0, 1, 69),
                Range::<Temperature, Humidity>::new(69, 0, 1),
            ],
        );

        // humidity-to-location
        //  60 56 37
        //  56 93 4"
        assert_eq!(
            actual.humidity_to_location,
            vec![
                Range::<Humidity, Location>::new(56, 60, 37),
                Range::<Humidity, Location>::new(93, 56, 4),
            ],
        );
    }

    #[test]
    fn test_getter() {
        let almanac = Almanac {
            seed_to_soil: vec![Range::<Seed, Soil>::new(1, 100, 1)],
            soil_to_fertilizer: vec![Range::<Soil, Fertilizer>::new(1, 200, 1)],
            fertilizer_to_water: vec![Range::<Fertilizer, Water>::new(1, 300, 1)],
            water_to_light: vec![Range::<Water, Light>::new(1, 350, 1)],
            light_to_temperature: vec![Range::<Light, Temperature>::new(1, 500, 1)],
            temperature_to_humidity: vec![Range::<Temperature, Humidity>::new(1, 600, 1)],
            humidity_to_location: vec![
                Range::<Humidity, Location>::new(1, 700, 1),
                Range::<Humidity, Location>::new(700, 0, 700),
            ],
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

        assert_eq!(almanac.location(Humidity(700)), Location(0));
        assert_eq!(almanac.location(Humidity(1399)), Location(699));

        assert_eq!(almanac.location(Humidity(1400)), Location(1400));
    }
}
