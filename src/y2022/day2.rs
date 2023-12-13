
use runner::aoc;

fn score(p1: char, p2: char) -> u64 {
    match(p1, p2)  {
        ('A', 'X') => 1 + 3,
        ('A', 'Y') => 2 + 6,
        ('A', 'Z') => 3 + 0,
        ('B', 'X') => 1 + 0,
        ('B', 'Y') => 2 + 3,
        ('B', 'Z') => 3 + 6,
        ('C', 'X') => 1 + 6,
        ('C', 'Y') => 2 + 0,
        ('C', 'Z') => 3 + 3,
        _ => panic!()
    }
}

#[aoc(day2, part1)]
fn part1(input: &str) -> u64 {
    
    input.trim().lines().map(|l| {
        let l = l.trim();
        score(l.chars().nth(0).unwrap(), l.chars().nth(2).unwrap())
    }).sum()
}

fn score_p2(p1: char, p2: char) -> u64 {
    let win_score = match p2 {
        'X' => 0,
        'Y' => 3,
        'Z' => 6,
        _ => panic!()
    };

    // 'X' is lose
    // 'Y' is draw
    // 'Z' is win
    win_score + match(p1, p2)  {
        ('A', 'X') => 3, // Scissors
        ('A', 'Y') => 1, // Rock
        ('A', 'Z') => 2, // Paper
        ('B', 'X') => 1, // Rock
        ('B', 'Y') => 2, // Paper
        ('B', 'Z') => 3, // Scissors
        ('C', 'X') => 2, // Paper
        ('C', 'Y') => 3, // Scissors
        ('C', 'Z') => 1, // Rock
        _ => panic!()
    }
}

#[aoc(day2, part2)]
fn part2(input: &str) -> u64 {
    input.trim().lines().map(|l| {
        let l = l.trim();
        score_p2(l.chars().nth(0).unwrap(), l.chars().nth(2).unwrap())
    }).sum()
}

#[cfg(test)]
mod tests {

    #[test]
    fn part1() {
        let input = "
            A Y
            B X
            C Z
        ".trim();

        assert_eq!(super::part1(input), 15);
    }

    #[test]
    fn part2() {
        let input = "
            A Y
            B X
            C Z
        ".trim();

        assert_eq!(super::part2(input), 12);
    }
}