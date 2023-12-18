use indicatif::ParallelProgressIterator;
use rayon::prelude::*;

fn main() {
    let input = include_str!("input.txt");
    let mut lines = input.lines();

    let seeds = parse_one_list(&lines.next().unwrap()[7..]);
    skip(&mut lines, 2);
    let almanac = Almanac::parse(lines);

    // let n = part1(almanac, seeds);
    let n = part2(almanac, seeds);
    println!("{n}");
}

fn part1(a: Almanac, seeds: Vec<u64>) -> u64 {
    let mut smallest_location = u64::MAX;

    for seed in seeds {
        let soil = a.seeds_to_soil.map(seed);
        let fertiliser = a.soil_to_fertiliser.map(soil);
        let water = a.fertiliser_to_water.map(fertiliser);
        let light = a.water_to_light.map(water);
        let temperature = a.light_to_temperature.map(light);
        let humidity = a.temperature_to_humidity.map(temperature);
        let location = a.humidity_to_location.map(humidity);

        smallest_location = smallest_location.min(location);
    }

    smallest_location
}

fn part2(a: Almanac, seeds: Vec<u64>) -> u64 {
    let seeds = seeds
        .chunks_exact(2)
        .map(|window| {
            let [a, b] = window else {
            unreachable!()
        };
            (*a, *a + *b)
        })
        .collect::<Vec<_>>();
    let seeds_len = seeds.len();

    seeds
        .into_par_iter()
        .progress_count(seeds_len as u64)
        .map(|(start, end)| {
            (start..end)
                .map(|seed| {
                    let soil = a.seeds_to_soil.map(seed);
                    let fertiliser = a.soil_to_fertiliser.map(soil);
                    let water = a.fertiliser_to_water.map(fertiliser);
                    let light = a.water_to_light.map(water);
                    let temperature = a.light_to_temperature.map(light);
                    let humidity = a.temperature_to_humidity.map(temperature);
                    a.humidity_to_location.map(humidity)
                })
                .min()
                .unwrap()
        })
        .min()
        .unwrap()
}

#[derive(Copy, Clone, Debug)]
pub struct Mapping {
    src_start: u64,
    dst_start: u64,
    len: u64,
}

pub struct Mappings(Vec<Mapping>);

pub struct Almanac {
    pub seeds_to_soil: Mappings,
    pub soil_to_fertiliser: Mappings,
    pub fertiliser_to_water: Mappings,
    pub water_to_light: Mappings,
    pub light_to_temperature: Mappings,
    pub temperature_to_humidity: Mappings,
    pub humidity_to_location: Mappings,
}

pub fn parse_one_list(i: &str) -> Vec<u64> {
    let mut list = vec![];

    let mut accumulator = String::new();
    for ch in i.chars() {
        if ch.is_ascii_digit() {
            accumulator.push(ch);
        } else if !accumulator.is_empty() {
            list.push(accumulator.parse().unwrap());
            accumulator.clear();
        }
    }

    if !accumulator.is_empty() {
        list.push(accumulator.parse().unwrap());
    }

    list
}

pub fn skip<I: Iterator>(i: &mut I, n: usize) {
    for _ in 0..n {
        let _ = i.next();
    }
}

impl Mappings {
    pub fn parse<'a>(lines: &mut impl Iterator<Item = &'a str>) -> Self {
        let mut line = lines.next().unwrap();
        let mut mappings = vec![];

        while !line.is_empty() {
            let mut list = parse_one_list(line);
            debug_assert_eq!(list.len(), 3);

            let dst_start = list.remove(0);
            let src_start = list.remove(0);
            let len = list.remove(0);

            mappings.push(Mapping {
                src_start,
                dst_start,
                len,
            });

            match lines.next() {
                Some(a) => line = a,
                None => break,
            }
        }

        Self(mappings)
    }

    pub fn map(&self, src: u64) -> u64 {
        let mut out = None;

        for Mapping {
            src_start,
            dst_start,
            len,
        } in self.0.iter().copied()
        {
            if src >= src_start && src < src_start + len {
                let offset = src - src_start;
                out = Some(dst_start + offset);
                break;
            }
        }

        out.unwrap_or(src)
    }
}

impl Almanac {
    pub fn parse<'a>(mut lines: impl Iterator<Item = &'a str>) -> Self {
        let seeds_to_soil = Mappings::parse(&mut lines);
        skip(&mut lines, 2);
        let soil_to_fertiliser = Mappings::parse(&mut lines);
        skip(&mut lines, 2);
        let fertiliser_to_water = Mappings::parse(&mut lines);
        skip(&mut lines, 2);
        let water_to_light = Mappings::parse(&mut lines);
        skip(&mut lines, 2);
        let light_to_temperature = Mappings::parse(&mut lines);
        skip(&mut lines, 2);
        let temperature_to_humidity = Mappings::parse(&mut lines);
        skip(&mut lines, 2);
        let humidity_to_location = Mappings::parse(&mut lines);

        Self {
            seeds_to_soil,
            soil_to_fertiliser,
            fertiliser_to_water,
            water_to_light,
            light_to_temperature,
            temperature_to_humidity,
            humidity_to_location,
        }
    }
}
