use runner::aoc;

type Range = std::ops::RangeInclusive<u64>;

fn parse_range(input: &str) -> (Range, Range) {
    fn parse(input: &str) -> Range {
        let (l, r) = input.trim().split_once('-').unwrap();

        let l = l.parse().unwrap();
        let r = r.parse().unwrap();
        l..=r
    }

    let (l, r) = input.trim().split_once(',').unwrap();

    (parse(l), parse(r))
}

#[aoc(day4, part1)]
fn part1(input: &str) -> u64 {
    let mut total = 0;

    for l in input.trim().lines().map(str::trim) {
        let (l, r) = parse_range(l);
        if (l.contains(r.start()) && l.contains(r.end()))
            || (r.contains(l.start()) && r.contains(l.end()))
        {
            total += 1;
        }
    }

    total
}

#[aoc(day4, part2)]
fn part2(input: &str) -> u64 {
    let mut total = 0;

    for l in input.trim().lines().map(str::trim) {
        let (l, r) = parse_range(l);
        if l.contains(r.start())
            || l.contains(r.end())
            || r.contains(l.start())
            || r.contains(l.end())
        {
            total += 1;
        }
    }

    total
}

#[cfg(test)]
mod tests {

    #[test]
    fn part1() {
        let input = "
            2-4,6-8
            2-3,4-5
            5-7,7-9
            2-8,3-7
            6-6,4-6
            2-6,4-8
        "
        .trim();

        assert_eq!(super::part1(input), 2);
    }

    #[test]
    fn part2() {
        let input = "
            2-4,6-8
            2-3,4-5
            5-7,7-9
            2-8,3-7
            6-6,4-6
            2-6,4-8
        "
        .trim();

        assert_eq!(super::part2(input), 4);
    }
}
