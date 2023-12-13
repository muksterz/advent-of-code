use runner::aoc;

#[derive(Debug)]
struct Sequence {
    sequences: Vec<Vec<i64>>,
}

impl Sequence {
    fn parse(input: &str) -> Self {
        let nums = input
            .trim()
            .split_whitespace()
            .map(|s| i64::from_str_radix(s, 10).unwrap())
            .collect::<Vec<_>>();

        let mut diffs = vec![nums];

        let mut index = 0;

        while diffs[index].iter().any(|&n| n != 0) {
            let diff = diffs[index].windows(2).map(|s| s[1] - s[0]).collect();
            diffs.push(diff);
            index += 1;
        }

        Self { sequences: diffs }
    }
}

#[aoc(day9, part1)]
fn part1(input: &str) -> i64 {
    let sequences = input
        .trim()
        .lines()
        .map(Sequence::parse)
        .collect::<Vec<_>>();

    let mut total = 0;

    for seq in sequences {
        let v: i64 = seq.sequences.iter().map(|v| v.last().unwrap()).sum();

        total += v;
    }

    total
}

#[aoc(day9, part2)]
fn part2(input: &str) -> i64 {
    let sequences = input
        .trim()
        .lines()
        .map(Sequence::parse)
        .collect::<Vec<_>>();
    let mut total = 0;

    for seq in sequences {
        let mut v = 0;

        for diff in seq.sequences.iter().rev() {
            v = diff[0] - v;
        }

        total += v;
    }

    total
}

#[cfg(test)]
mod tests {
    #[test]
    fn part1() {
        let input = "
            0 3 6 9 12 15
            1 3 6 10 15 21
            10 13 16 21 30 45
        "
        .trim();

        assert_eq!(super::part1(input), 114);
    }

    #[test]
    fn part2() {
        let input = "
            0 3 6 9 12 15
            1 3 6 10 15 21
            10 13 16 21 30 45
        "
        .trim();

        assert_eq!(super::part2(input), 2);
    }
}
