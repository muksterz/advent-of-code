use std::collections::HashMap;

use runner::aoc;

#[derive(Debug)]
struct Springs {
    s: Vec<Spring>,
}

impl Springs {
    fn parse(input: &str, part2: bool) -> Self {
        Self {
            s: input.trim().lines().map(|s| Spring::parse(s, part2)).collect(),
        }
    }
}

#[derive(Debug)]
struct Spring {
    parts: Vec<Part>,
    lengths: Vec<usize>,
}

impl Spring {
    fn parse(input: &str, part2: bool) -> Self {
        let (s, nums_s) = input.trim().split_once(' ').unwrap();
        let mut nums : Vec<usize> = nums_s
            .split(',')
            .map(|s| usize::from_str_radix(s, 10).unwrap())
            .collect();
        let mut parts: Vec<Part> = s.chars().map(Part::from_char).collect();

        let old_nums = nums.clone();
        let old_parts = parts.clone();
        if part2 {
            for _ in 0..4 {
                nums.extend(old_nums.clone());
                parts.push(Part::Unknown);
                parts.extend(old_parts.clone());
            }
        }

        Self {
            parts,
            lengths: nums,
        }
    }

    fn arrangements(&self) -> u64 {

        let mut cache = HashMap::new();

        
        Spring::solve(&mut cache, &self.parts, 0, &self.lengths, 0)
    }

    fn solve(mut cache: &mut HashMap<HashStruct, u64>,  parts: &[Part], index: usize, lengths: &[usize], lengths_index: usize) -> u64 {

        fn check(index: usize, len: usize, parts: &[Part], last: bool) -> bool {
            if index > 0 && parts[index-1] == Part::Broken {
                return false;
            }
            if last && parts[index+len..].iter().any(|&p| p == Part::Broken){
                return false
            }

            parts.iter().skip(index).take(len).copied().all(|p| p != Part::Working) && parts.get(index + len) != Some(&Part::Broken)
        }

        if lengths.len() == lengths_index {
            return 1;
        }

        let hash = HashStruct {
            start_index: index, lengths_index
        };
        if let Some(&a) = cache.get(&hash) {
            return a;
        }



        let min_len = index + lengths[lengths_index..].iter().sum::<usize>() + lengths[lengths_index..].len() - 1;

        if parts.len() < min_len {
            cache.insert(hash, 0);
            return 0;
        }


        let len = lengths[lengths_index];

        let mut total = 0;

        if check(index, len, parts, lengths[lengths_index..].len() == 1) {
            let new_index = index + len + 1;
            total += Spring::solve(&mut cache, parts, new_index, lengths, lengths_index + 1);
        }
        if parts[index] != Part::Broken {
            total += Spring::solve(&mut cache, parts, index + 1, lengths, lengths_index);
        }


        cache.insert(hash, total);
                
        total

    }

}

#[derive(Hash, Copy, Clone, PartialEq, Eq)]
struct HashStruct {
    start_index: usize,
    lengths_index: usize
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum Part {
    Broken,
    Working,
    Unknown,
}

impl Part {
    fn from_char(c: char) -> Self {
        match c {
            '.' => Self::Working,
            '#' => Self::Broken,
            '?' => Self::Unknown,
            _ => panic!(),
        }
    }
}

#[aoc(day12, part1)]
fn part1(input: &str) -> u64 {
    let springs = Springs::parse(input, false);

    let mut total = 0;

    for s in springs.s.iter() {
        let a = s.arrangements();
        total += a;
    }

    total
}

#[aoc(day12, part2)]
fn part2(input: &str) -> u64 {
    let springs = Springs::parse(input, true);


    springs.s.iter().map(|s| {
        
        s.arrangements()
    }).reduce( |a, b| a + b).unwrap()
}

#[cfg(test)]
mod tests {

    #[test]
    fn part1() {
        let input = "
            ???.### 1,1,3
            .??..??...?##. 1,1,3
            ?#?#?#?#?#?#?#? 1,3,1,6
            ????.#...#... 4,1,1
            ????.######..#####. 1,6,5
            ?###???????? 3,2,1
        "
        .trim();

        let inputs = [
            "???.### 1,1,3",
            ".??..??...?##. 1,1,3",
            "?#?#?#?#?#?#?#? 1,3,1,6",
            "????.#...#... 4,1,1",
            "????.######..#####. 1,6,5",
            "?###???????? 3,2,1"
        ];

        let results = [1, 4, 1, 1, 4, 10];
        assert_eq!(super::part1("????.#...#... 4,1,1"), 1);
        assert_eq!(super::part1("?#??????##? 1,1,2"), 4);


        assert_eq!(super::part1("..???.??.? 1,1,1"), 9);

        assert_eq!(super::part1("?#?#?#?#?#?#?#? 1,3,1,6"), 1);


        for (i, r) in inputs.into_iter().zip(results) {
            assert_eq!(super::part1(i), r, "Expected: {r} with {i}");
        }


        assert_eq!(super::part1(input), 21);
        //panic!();
    }

    #[test]
    #[allow(unused)]
    fn part2() {
        let input = "
        ???.### 1,1,3
        .??..??...?##. 1,1,3
        ?#?#?#?#?#?#?#? 1,3,1,6
        ????.#...#... 4,1,1
        ????.######..#####. 1,6,5
        ?###???????? 3,2,1
        "
    .trim();


    assert_eq!(super::part2("???.### 1,1,3"), 1);
    assert_eq!(super::part2(input), 525152);


    }
}