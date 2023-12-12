use std::ops::Range;

use aoc_runner_derive::aoc;

#[derive(Clone, Debug)]
struct Map {
    inputs: Range<u64>,
    output: Range<u64>,
}

impl Map {
    // dest src len
    fn parse(input: &str) -> Self {
        let mut nums = input
            .trim()
            .split_whitespace()
            .map(|n| u64::from_str_radix(n, 10).unwrap());
        let dest = nums.next().unwrap();
        let src = nums.next().unwrap();
        let len = nums.next().unwrap();

        Self {
            inputs: src..(src + len),
            output: dest..(dest + len),
        }
    }

    fn map(&self, input: u64) -> Option<u64> {
        if self.inputs.contains(&input) {
            let offset = input - self.inputs.start;
            Some(self.output.start + offset)
        } else {
            None
        }
    }

    fn map_range(&self, input: Range<u64>) -> RangeMapOutput {
        let m = match (
            self.inputs.contains(&input.start),
            self.inputs.contains(&(input.end - 1)),
        ) {
            (true, true) => RangeMapOutput::InputFullyContained(self.map_contained_range(input)),
            // input starts before map
            (false, true) => {
                let out_range = input.start..self.inputs.start;
                let in_range = self.inputs.start..input.end;
                RangeMapOutput::Overlapping {
                    mapped: self.map_contained_range(in_range),
                    unmapped: out_range,
                }
            }
            // input ends after map
            (true, false) => {
                let out_range = self.inputs.end..input.end;
                let in_range = input.start..self.inputs.end;
                RangeMapOutput::Overlapping {
                    mapped: self.map_contained_range(in_range),
                    unmapped: out_range,
                }
            }
            (false, false) => {
                if input.contains(&self.inputs.start) {
                    let out_head = input.start..self.inputs.start;
                    let out_tail = self.inputs.end..input.end;

                    RangeMapOutput::OutputFullyContained {
                        mapped: self.output.clone(),
                        unmapped_head: out_head,
                        unmapped_tail: out_tail,
                    }
                } else {
                    RangeMapOutput::Disjoint
                }
            }
        };
        m
    }

    fn map_contained_range(&self, input: Range<u64>) -> Range<u64> {
        let start = self.map(input.start).expect("Range Not contained");
        let end = self.map(input.end - 1).expect("Range Not contained") + 1;
        start..end
    }
}

#[derive(Debug, PartialEq, Eq)]
enum RangeMapOutput {
    Disjoint,
    InputFullyContained(Range<u64>),
    Overlapping {
        mapped: Range<u64>,
        unmapped: Range<u64>,
    },
    OutputFullyContained {
        mapped: Range<u64>,
        unmapped_head: Range<u64>,
        unmapped_tail: Range<u64>,
    },
}

impl RangeMapOutput {
    fn push_results(&self, output: &mut Vec<Range<u64>>, stack: &mut Vec<Range<u64>>) {
        match self {
            Self::Disjoint => {}
            Self::InputFullyContained(r) => output.push(r.clone()),
            Self::Overlapping { mapped, unmapped } => {
                output.push(mapped.clone());
                stack.push(unmapped.clone())
            }
            Self::OutputFullyContained {
                mapped,
                unmapped_head,
                unmapped_tail,
            } => {
                output.push(mapped.clone());
                stack.push(unmapped_head.clone());
                stack.push(unmapped_tail.clone());
            }
        }
    }
}

#[derive(Debug)]
struct MapSet {
    maps: Vec<Map>,
}

impl MapSet {
    fn parse(input: &str) -> Self {
        let maps = input.trim().lines().skip(1).map(Map::parse);
        Self {
            maps: maps.collect(),
        }
    }

    fn map(&self, input: u64) -> u64 {
        for map in self.maps.iter() {
            if let Some(v) = map.map(input) {
                return v;
            }
        }
        input
    }

    fn map_ranges(&self, input: Vec<Range<u64>>) -> Vec<Range<u64>> {
        let mut output = Vec::new();
        let mut stack = input;
        while let Some(r) = stack.pop() {
            if r.start >= r.end {
                continue;
            }
            let mut match_found = false;
            for map in self.maps.iter() {
                let r = map.map_range(r.clone());
                r.push_results(&mut output, &mut stack);
                if r != RangeMapOutput::Disjoint {
                    match_found = true;
                }
            }
            if !match_found {
                output.push(r)
            }
        }

        output
    }
}

#[derive(Debug)]
struct Maps {
    maps: Vec<MapSet>,
}

impl Maps {
    fn parse(input: &str) -> Self {
        let sets = input.trim().split("\n\n").map(MapSet::parse);
        Self {
            maps: sets.collect(),
        }
    }

    fn seed_to_loc(&self, seed: u64) -> u64 {
        let mut temp = seed;
        for map in self.maps.iter() {
            temp = map.map(temp);
        }
        temp
    }

    fn map_ranges(&self, ranges: &Vec<Range<u64>>) -> Vec<Range<u64>> {
        let mut ranges = ranges.clone();
        for map in self.maps.iter() {
            ranges = map.map_ranges(ranges);
        }
        ranges
    }
}

fn parse_seeds(input: &str) -> Vec<u64> {
    let nums = input
        .trim()
        .split_whitespace()
        .skip(1)
        .map(|s| u64::from_str_radix(s, 10).unwrap());
    nums.collect()
}

#[aoc(day5, part1)]
fn part1(input: &str) -> u64 {
    let (seeds, maps) = input.split_once("\n\n").unwrap();

    let seeds = parse_seeds(seeds);

    let maps = Maps::parse(maps);

    let lowest = seeds
        .iter()
        .copied()
        .map(|s| maps.seed_to_loc(s))
        .min()
        .unwrap();

    lowest
}

fn parse_seed_ranges(input: &str) -> Vec<Range<u64>> {
    let mut nums = input
        .trim()
        .split_whitespace()
        .skip(1)
        .map(|s| u64::from_str_radix(s, 10).unwrap());

    let mut output = Vec::new();

    while let Some(n) = nums.next() {
        output.push(n..(n + nums.next().unwrap()))
    }

    output
}

#[aoc(day5, part2)]
fn part2(input: &str) -> u64 {
    let (ranges, maps) = input.split_once("\n\n").unwrap();

    let ranges = parse_seed_ranges(ranges);
    //let t = 79..93u64;
    let maps = Maps::parse(maps);
    let ranges = maps.map_ranges(&ranges);

    println!("{}", ranges.len());
    ranges.iter().map(|r| r.start).min().unwrap()
}

#[cfg(test)]
mod tests {

    #[test]
    fn part1() {
        let input = "
        seeds: 79 14 55 13

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
        56 93 4
        "
        .trim();

        assert_eq!(35, super::part1(input))
    }

    #[test]
    fn part2() {
        let input = "
        seeds: 79 14 55 13

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
        56 93 4
        "
        .trim();

        assert_eq!(46, super::part2(input))
    }
}
