use aoc_runner_derive::aoc;

fn parse_nums(input: &str) -> Vec<u64> {
    let (_, nums) = input.split_once(':').unwrap();
    nums.trim()
        .split_whitespace()
        .map(|n| u64::from_str_radix(n, 10).unwrap())
        .collect()
}

#[aoc(day6, part1)]
fn part1(input: &str) -> u64 {
    let mut lines = input.lines();
    let times = parse_nums(lines.next().unwrap());
    let distances = parse_nums(lines.next().unwrap());

    let mut total = 1;
    // t = race time, d = race distance, c = charge time
    for (t, d) in times.into_iter().zip(distances) {
        let dis = f64::sqrt((t * t - 4 * d) as f64);
        let t = t as f64;
        let mut min_c = (t - dis) / 2.0;
        let mut max_c = (t + dis) / 2.0;

        if min_c.ceil() == min_c {
            min_c += 1.0;
        }
        if max_c.floor() == max_c {
            max_c -= 1.0;
        }

        let min_c = min_c.ceil() as u64;
        let max_c = max_c.floor() as u64;

        let ways = max_c - min_c + 1;
        total *= ways;
    }

    total
}

#[aoc(day6, part2)]
fn part2(input: &str) -> u64 {
    let mut lines = input.lines();
    let time: u64 = lines
        .next()
        .unwrap()
        .split(':')
        .skip(1)
        .next()
        .unwrap()
        .replace(' ', "")
        .parse()
        .unwrap();
    let distance: u64 = lines
        .next()
        .unwrap()
        .split(':')
        .skip(1)
        .next()
        .unwrap()
        .replace(' ', "")
        .parse()
        .unwrap();
    let t = time;
    let d = distance;

    let dis = f64::sqrt((t * t - 4 * d) as f64);
    let t = t as f64;
    let mut min_c = (t - dis) / 2.0;
    let mut max_c = (t + dis) / 2.0;

    if min_c.ceil() == min_c {
        min_c += 1.0;
    }
    if max_c.floor() == max_c {
        max_c -= 1.0;
    }

    let min_c = min_c.ceil() as u64;
    let max_c = max_c.floor() as u64;

    let ways = max_c - min_c + 1;

    ways
}

#[cfg(test)]
mod tests {

    #[test]
    fn part1() {
        let input = "
        Time:      7  15   30
        Distance:  9  40  200
        "
        .trim();

        assert_eq!(super::part1(input), 288)
    }
    #[test]
    fn part2() {
        let input = "
        Time:      7  15   30
        Distance:  9  40  200
        "
        .trim();

        assert_eq!(super::part2(input), 71503)
    }
}
