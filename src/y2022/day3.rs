use runner::aoc;

fn priority(c: char) -> u64 {
    if c.is_ascii_uppercase() {
        c as u8 as u64 - 64 + 26
    } else if c.is_ascii_lowercase() {
        c as u8 as u64 - 96
    } else {
        panic!()
    }
}

#[aoc(day3, part1)]
fn part1(input: &str) -> u64 {
    let mut total = 0;
    for l in input.trim().lines() {
        let l = l.trim();
        let split = l.len() / 2;
        let s1 = &l[0..split];
        let s2 = &l[split..];
        let mut p = 0;
        for c in s1.chars() {
            if s2.contains(c) {
                p = p.max(priority(c));
            }
        }

        total += p;
    }

    total
}

#[aoc(day3, part2)]
fn part2(input: &str) -> u64 {
    let mut total = 0;
    let mut lines = input.trim().lines().map(|l| l.trim());
    while let Some(l1) = lines.next() {
        let l2 = lines.next().unwrap();
        let l3 = lines.next().unwrap();

        let mut p = 0;
        for c in l1.chars() {
            if l2.contains(c) && l3.contains(c) {
                p = p.max(priority(c))
            }
        }
        total += p;
    }

    total
}

#[cfg(test)]
mod tests {

    #[test]
    fn part1() {
        let input = "
            vJrwpWtwJgWrhcsFMMfFFhFp
            jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
            PmmdzqPrVvPwwTWBwg
            wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
            ttgJtRGJQctTZtZT
            CrZsJsPPZsGzwwsLwLmpwMDw
        "
        .trim();

        assert_eq!(super::part1(input), 157)
    }

    #[test]
    fn part2() {
        let input = "
            vJrwpWtwJgWrhcsFMMfFFhFp
            jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
            PmmdzqPrVvPwwTWBwg
            wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
            ttgJtRGJQctTZtZT
            CrZsJsPPZsGzwwsLwLmpwMDw
        "
        .trim();

        assert_eq!(super::part2(input), 70)
    }
}
