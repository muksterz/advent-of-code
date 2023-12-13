use runner::aoc;

#[aoc(day1, part1)]
fn part1(input: &str) -> u64 {
    let mut elfs: Vec<u64> = Vec::new();

    for e in input.trim().split("\n\n") {
        let n = e
            .trim()
            .lines()
            .map(|l| u64::from_str_radix(l, 10).unwrap());
        elfs.push(n.sum())
    }

    elfs.into_iter().max().unwrap()
}

#[aoc(day1, part2)]
fn part2(input: &str) -> u64 {
    let mut elfs: Vec<u64> = Vec::new();

    for e in input.trim().split("\n\n") {
        let n = e
            .trim()
            .lines()
            .map(|l| u64::from_str_radix(l, 10).unwrap());
        elfs.push(n.sum())
    }

    elfs.sort();
    elfs.into_iter().rev().take(3).sum()
}
