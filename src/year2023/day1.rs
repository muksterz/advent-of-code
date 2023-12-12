use aoc_runner_derive::aoc;

#[aoc(day1, part1)]
fn part1(input: &str) -> u64 {
    let mut total = 0;

    for line in input.lines() {
        let first = line
            .chars()
            .filter_map(|c| u64::from_str_radix(&c.to_string(), 10).ok())
            .next()
            .unwrap();
        let last = line
            .chars()
            .rev()
            .filter_map(|c| u64::from_str_radix(&c.to_string(), 10).ok())
            .next()
            .unwrap();
        total += first * 10 + last;
    }

    total
}

#[aoc(day1, part2)]
fn part2(input: &str) -> u64 {
    let mut total = 0;

    for l in input.lines().map(str::trim) {
        let first = l
            .char_indices()
            .filter_map(|(s, _)| numberfy(&l[s..], false))
            .next()
            .unwrap();

        let lr: String = l.chars().rev().collect();

        let last = lr
            .char_indices()
            .filter_map(|(s, _)| numberfy(&lr[s..], true))
            .next()
            .unwrap();

        total += first * 10 + last;
    }

    total
}

fn numberfy(s: &str, rev: bool) -> Option<u64> {
    let mut numbers = vec![
        ("one".to_string(), 1),
        ("two".to_string(), 2),
        ("three".to_string(), 3),
        ("four".to_string(), 4),
        ("five".to_string(), 5),
        ("six".to_string(), 6),
        ("seven".to_string(), 7),
        ("eight".to_string(), 8),
        ("nine".to_string(), 9),
    ];

    if rev {
        numbers = numbers
            .iter()
            .map(|(s, n)| (s.chars().rev().collect(), *n))
            .collect()
    }

    let mut output = None;

    for (m, n) in numbers {
        if s.starts_with(&m) {
            output = Some(n);
            break;
        }
    }

    if let Ok(n) = u64::from_str_radix(&s[0..1], 10) {
        output = Some(n);
    }

    output
}

#[cfg(test)]
mod tests {
    #[test]
    fn part1() {
        let input = "1abc2
        pqr3stu8vwx
        a1b2c3d4e5f
        treb7uchet";

        assert_eq!(super::part1(input), 142);
    }

    #[test]
    fn part2() {
        let input = "two1nine
        eightwothree
        abcone2threexyz
        xtwone3four
        4nineeightseven2
        zoneight234
        7pqrstsixteen";

        assert_eq!(super::part2(input), 281)
    }
}
