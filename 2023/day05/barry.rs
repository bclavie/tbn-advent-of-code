use std::{collections::HashMap, ops::RangeInclusive};

fn main() {
    let inputs = include_str!("inputs/05");
    let (init_seeds, mappings) = parse(inputs);

    // part1;
    let dest_locations: Vec<i64> = init_seeds
        .iter()
        .map(|seed| {
            let seed_to_soil = mappings.get(&(Thing::Seed, Thing::Soil)).unwrap();
            let mut soil = *seed;
            for map in seed_to_soil {
                if let Some(x) = map.try_map_num(*seed) {
                    soil = x;
                    break;
                }
            }

            let soil_to_fert = mappings.get(&(Thing::Soil, Thing::Fertilizer)).unwrap();
            let mut fert = soil;
            for map in soil_to_fert {
                if let Some(x) = map.try_map_num(soil) {
                    fert = x;
                    break;
                }
            }

            let fert_to_water = mappings.get(&(Thing::Fertilizer, Thing::Water)).unwrap();
            let mut water = fert;
            for map in fert_to_water {
                if let Some(x) = map.try_map_num(fert) {
                    water = x;
                    break;
                }
            }

            let water_to_light = mappings.get(&(Thing::Water, Thing::Light)).unwrap();
            let mut light: i64 = water;
            for map in water_to_light {
                if let Some(x) = map.try_map_num(water) {
                    light = x;
                    break;
                }
            }

            let light_to_temp = mappings.get(&(Thing::Light, Thing::Temperature)).unwrap();
            let mut temp: i64 = light;
            for map in light_to_temp {
                if let Some(x) = map.try_map_num(light) {
                    temp = x;
                    break;
                }
            }

            let temp_to_humidity = mappings
                .get(&(Thing::Temperature, Thing::Humidity))
                .unwrap();
            let mut humidity: i64 = temp;
            for map in temp_to_humidity {
                if let Some(x) = map.try_map_num(temp) {
                    humidity = x;
                    break;
                }
            }

            let humidity_to_location = mappings.get(&(Thing::Humidity, Thing::Location)).unwrap();
            let mut location: i64 = humidity;
            for map in humidity_to_location {
                if let Some(x) = map.try_map_num(humidity) {
                    location = x;
                    break;
                }
            }

            location
        })
        .collect();

    let part1 = dest_locations
        .iter()
        .reduce(|a, b| if a < b { a } else { b })
        .unwrap();

    println!("part1: {}", part1);

    // part2 start from 0 and do the inverse
    let mut seed_ranges: Vec<RangeInclusive<i64>> = vec![];
    let mut seed_iter = init_seeds.iter();
    while let Some(first) = seed_iter.next() {
        let range: &i64 = seed_iter.next().unwrap();
        seed_ranges.push(*first..=*first + range);
    }

    let mut location = 0;
    // Work backwards until we hit a seed that is in the initial ranges.
    loop {
        let humidity_to_location = mappings.get(&(Thing::Humidity, Thing::Location)).unwrap();
        let mut humidity: i64 = location;
        for map in humidity_to_location {
            if let Some(x) = map.try_map_inverse(location) {
                humidity = x;
                break;
            }
        }

        let temp_to_humidity = mappings
            .get(&(Thing::Temperature, Thing::Humidity))
            .unwrap();
        let mut temp: i64 = humidity;
        for map in temp_to_humidity {
            if let Some(x) = map.try_map_inverse(humidity) {
                temp = x;
                break;
            }
        }

        let light_to_temp = mappings.get(&(Thing::Light, Thing::Temperature)).unwrap();
        let mut light: i64 = temp;
        for map in light_to_temp {
            if let Some(x) = map.try_map_inverse(temp) {
                light = x;
                break;
            }
        }

        let water_to_light = mappings.get(&(Thing::Water, Thing::Light)).unwrap();
        let mut water: i64 = light;
        for map in water_to_light {
            if let Some(x) = map.try_map_inverse(light) {
                water = x;
                break;
            }
        }

        let fert_to_water = mappings.get(&(Thing::Fertilizer, Thing::Water)).unwrap();
        let mut fert = water;
        for map in fert_to_water {
            if let Some(x) = map.try_map_inverse(water) {
                fert = x;
                break;
            }
        }
        let soil_to_fert: &Vec<Map> = mappings.get(&(Thing::Soil, Thing::Fertilizer)).unwrap();
        let mut soil = fert;
        for map in soil_to_fert {
            if let Some(x) = map.try_map_inverse(fert) {
                soil = x;
                break;
            }
        }

        let seed_to_soil: &Vec<Map> = mappings.get(&(Thing::Seed, Thing::Soil)).unwrap();
        let mut seed = soil;
        for map in seed_to_soil {
            if let Some(x) = map.try_map_inverse(soil) {
                seed = x;
                break;
            }
        }

        let mut found = false;
        for seed_range in seed_ranges.iter() {
            if seed_range.contains(&seed) {
                found = true;
                break;
            }
        }

        if found {
            break;
        }

        location += 1;
    }

    println!("part2: {}", location);
}

fn parse(inputs: &str) -> (Vec<i64>, HashMap<(Thing, Thing), Vec<Map>>) {
    // Much faster to just hard code the line nos.
    let lines: Vec<&str> = inputs.split('\n').collect();
    let seeds = lines[0]
        .split(' ')
        .skip(1)
        .map(|s| s.parse::<i64>().unwrap())
        .collect();
    let mut mappings = HashMap::<(Thing, Thing), Vec<Map>>::new();

    let seed_to_soil_start = lines
        .iter()
        .position(|s| *s == "seed-to-soil map:")
        .unwrap();
    let seed_to_soil_end = lines
        .iter()
        .skip(seed_to_soil_start)
        .position(|l| l.is_empty())
        .unwrap();
    let seed_to_soil_mappings = lines
        .iter()
        .skip(seed_to_soil_start + 1)
        .take(seed_to_soil_end - 1)
        .map(|line| {
            let nums: Vec<&str> = line.split(' ').collect();
            Map {
                dest_start: nums[0].parse::<i64>().unwrap(),
                orig_start: nums[1].parse::<i64>().unwrap(),
                range: nums[2].parse::<i64>().unwrap(),
            }
        })
        .collect();

    mappings.insert((Thing::Seed, Thing::Soil), seed_to_soil_mappings);

    let soil_to_fertilizer_start = lines
        .iter()
        .position(|s| *s == "soil-to-fertilizer map:")
        .unwrap();
    let soil_to_fertilizer_end = lines
        .iter()
        .skip(soil_to_fertilizer_start)
        .position(|l| l.is_empty())
        .unwrap();
    let soil_to_fertilizer_mappings = lines
        .iter()
        .skip(soil_to_fertilizer_start + 1)
        .take(soil_to_fertilizer_end - 1)
        .map(|line| {
            let nums: Vec<&str> = line.split(' ').collect();
            Map {
                dest_start: nums[0].parse::<i64>().unwrap(),
                orig_start: nums[1].parse::<i64>().unwrap(),
                range: nums[2].parse::<i64>().unwrap(),
            }
        })
        .collect();
    mappings.insert(
        (Thing::Soil, Thing::Fertilizer),
        soil_to_fertilizer_mappings,
    );

    let fertilizer_to_water_start = lines
        .iter()
        .position(|s| *s == "fertilizer-to-water map:")
        .unwrap();
    let fertilizer_to_water_end = lines
        .iter()
        .skip(fertilizer_to_water_start)
        .position(|l| l.is_empty())
        .unwrap();
    let fertilizer_to_water_mappings = lines
        .iter()
        .skip(fertilizer_to_water_start + 1)
        .take(fertilizer_to_water_end - 1)
        .map(|line| {
            let nums: Vec<&str> = line.split(' ').collect();
            Map {
                dest_start: nums[0].parse::<i64>().unwrap(),
                orig_start: nums[1].parse::<i64>().unwrap(),
                range: nums[2].parse::<i64>().unwrap(),
            }
        })
        .collect();
    mappings.insert(
        (Thing::Fertilizer, Thing::Water),
        fertilizer_to_water_mappings,
    );

    let water_to_light_start = lines
        .iter()
        .position(|s| *s == "water-to-light map:")
        .unwrap();
    let water_to_light_end = lines
        .iter()
        .skip(water_to_light_start)
        .position(|l| l.is_empty())
        .unwrap();
    let water_to_light_mappings = lines
        .iter()
        .skip(water_to_light_start + 1)
        .take(water_to_light_end - 1)
        .map(|line| {
            let nums: Vec<&str> = line.split(' ').collect();
            Map {
                dest_start: nums[0].parse::<i64>().unwrap(),
                orig_start: nums[1].parse::<i64>().unwrap(),
                range: nums[2].parse::<i64>().unwrap(),
            }
        })
        .collect();
    mappings.insert((Thing::Water, Thing::Light), water_to_light_mappings);

    let light_to_temperature_start = lines
        .iter()
        .position(|s| *s == "light-to-temperature map:")
        .unwrap();
    let light_to_temperature_end = lines
        .iter()
        .skip(light_to_temperature_start)
        .position(|l| l.is_empty())
        .unwrap();
    let light_to_temperature_mappings = lines
        .iter()
        .skip(light_to_temperature_start + 1)
        .take(light_to_temperature_end - 1)
        .map(|line| {
            let nums: Vec<&str> = line.split(' ').collect();
            Map {
                dest_start: nums[0].parse::<i64>().unwrap(),
                orig_start: nums[1].parse::<i64>().unwrap(),
                range: nums[2].parse::<i64>().unwrap(),
            }
        })
        .collect();
    mappings.insert(
        (Thing::Light, Thing::Temperature),
        light_to_temperature_mappings,
    );

    let temperature_to_humidity_start = lines
        .iter()
        .position(|s| *s == "temperature-to-humidity map:")
        .unwrap();
    let temperature_to_humidity_end = lines
        .iter()
        .skip(temperature_to_humidity_start)
        .position(|l| l.is_empty())
        .unwrap();
    let temperature_to_humidity_mappings = lines
        .iter()
        .skip(temperature_to_humidity_start + 1)
        .take(temperature_to_humidity_end - 1)
        .map(|line| {
            let nums: Vec<&str> = line.split(' ').collect();
            Map {
                dest_start: nums[0].parse::<i64>().unwrap(),
                orig_start: nums[1].parse::<i64>().unwrap(),
                range: nums[2].parse::<i64>().unwrap(),
            }
        })
        .collect();
    mappings.insert(
        (Thing::Temperature, Thing::Humidity),
        temperature_to_humidity_mappings,
    );

    let humidity_to_location_start = lines
        .iter()
        .position(|s| *s == "humidity-to-location map:")
        .unwrap();
    let humidity_to_location_mappings = lines
        .iter()
        .skip(humidity_to_location_start + 1)
        .map(|line| {
            let nums: Vec<&str> = line.split(' ').collect();
            Map {
                dest_start: nums[0].parse::<i64>().unwrap(),
                orig_start: nums[1].parse::<i64>().unwrap(),
                range: nums[2].parse::<i64>().unwrap(),
            }
        })
        .collect();
    mappings.insert(
        (Thing::Humidity, Thing::Location),
        humidity_to_location_mappings,
    );

    (seeds, mappings)
}

#[derive(Eq, PartialEq, Hash, Debug)]
enum Thing {
    Seed,
    Soil,
    Fertilizer,
    Water,
    Light,
    Temperature,
    Humidity,
    Location,
}

#[derive(Debug)]
struct Map {
    dest_start: i64,
    orig_start: i64,
    range: i64,
}

impl Map {
    fn try_map_num(&self, num: i64) -> Option<i64> {
        if num >= self.orig_start && num <= self.orig_start + self.range {
            let diff = self.orig_start + self.range - num;
            Some(self.dest_start + self.range - diff)
        } else {
            None
        }
    }
    fn try_map_inverse(&self, num: i64) -> Option<i64> {
        if num >= self.dest_start && num <= self.dest_start + self.range {
            let diff = self.dest_start + self.range - num;
            Some(self.orig_start + self.range - diff)
        } else {
            None
        }
    }
}
