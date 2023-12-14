use runner::aoc;

#[derive(Debug)]
struct Card {
    num: u64,
    wins: u64,
}

impl Card {
    fn parse(input: &str) -> Self {
        let (card, nums) = input.trim().split_once(':').unwrap();

        let (_, card_num) = card.trim().split_once(' ').unwrap();
        let card_num: u64 = card_num.trim().parse().unwrap();

        let (winning_nums, drew_nums) = nums.trim().split_once('|').unwrap();

        let winning_nums: Vec<u64> = winning_nums
            .split_whitespace()
            .map(|n| n.parse().unwrap())
            .collect();
        let drew_nums: Vec<u64> = drew_nums
            .split_whitespace()
            .map(|n| n.parse().unwrap())
            .collect();

        let matches = drew_nums
            .iter()
            .filter(|n| winning_nums.contains(n))
            .count() as u64;

        Card {
            num: card_num,
            wins: matches,
        }
    }

    fn score(&self) -> u64 {
        if self.wins() > 0 {
            2u64.pow(self.wins() as u32 - 1)
        } else {
            0
        }
    }

    fn wins(&self) -> u64 {
        self.wins
    }
}

#[aoc(day4, part1)]
fn part1(input: &str) -> u64 {
    let mut total = 0;

    for line in input.lines() {
        total += Card::parse(line).score();
    }

    total
}

#[aoc(day4, part2)]
fn part2(input: &str) -> u64 {
    let cards: Vec<_> = input.lines().map(Card::parse).collect();
    let mut card_amounts: Vec<_> = std::iter::repeat(1u64).take(cards.len()).collect();

    for (i, card) in cards.iter().enumerate() {
        let amount = card_amounts[i];
        let score = card.wins() as usize;
        for i in 0..score {
            let i = i + card.num as usize;
            card_amounts[i] += amount;
        }
    }

    card_amounts.iter().sum()
}

#[cfg(test)]
mod tests {

    #[test]
    fn part1() {
        let input = "
            Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
            Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
            Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
            Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
            Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
            Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
        "
        .trim();

        assert_eq!(super::part1(input), 13);
    }

    #[test]
    fn part2() {
        let input = "
            Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
            Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
            Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
            Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
            Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
            Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
        "
        .trim();

        assert_eq!(super::part2(input), 30)
    }
}
