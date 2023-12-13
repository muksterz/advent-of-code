use std::{
    cmp::{Ordering, Reverse},
    collections::HashMap,
    fmt::Display,
};

use runner::aoc;

#[derive(PartialEq, Eq, Debug, Clone, Copy, Hash)]
enum Card {
    A,
    K,
    Q,
    J,
    T,
    N(u64),
}

impl Ord for Card {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl Card {
    fn to_num(self) -> u64 {
        match self {
            Card::A => 14,
            Card::K => 13,
            Card::Q => 12,
            Card::J => 0,
            Card::T => 10,
            Card::N(n) => n,
        }
    }
}

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.to_num().partial_cmp(&other.to_num())
    }
}

impl Card {
    fn to_char(self) -> char {
        match self {
            Card::A => 'A',
            Card::K => 'K',
            Card::Q => 'Q',
            Card::J => 'J',
            Card::T => 'T',
            Card::N(n) => (n + 48) as u8 as char,
        }
    }
}

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
struct Hand {
    cards: [Card; 5],
    bid: u64,
    max_cards: [Card; 5],
}

impl Display for Hand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for c in self.cards {
            write!(f, "{}", c.to_char())?;
        }
        write!(f, "\tmax: ")?;

        for c in self.max_cards {
            write!(f, "{}", c.to_char())?;
        }

        write!(f, " {}", self.bid)?;

        Ok(())
    }
}

impl Hand {
    fn rank(&self) -> Type {
        let mut set: HashMap<Card, u64> = HashMap::new();

        for c in self.max_cards.iter().copied() {
            let entry = set.entry(c).or_default();
            *entry += 1;
        }

        let mut cs: Vec<_> = set.into_iter().collect();
        cs.sort_by_key(|(_, n)| Reverse(*n));

        let highest = cs[0].1;
        let second = cs.get(1).map(|l| l.1).unwrap_or_default();

        let t = match (highest, second) {
            (5, _) => Type::FiveOfAKind,
            (4, _) => Type::FourOfAKind,
            (3, 2) => Type::FullHouse,
            (3, _) => Type::ThreeOfAKind,
            (2, 2) => Type::TwoPair,
            (2, _) => Type::OnePair,
            (1, _) => Type::HighCard,
            _ => unreachable!(),
        };

        t
    }

    fn maximize(self) -> Self {
        let mut map: HashMap<_, u64> = HashMap::new();
        for c in self.cards.iter().copied() {
            if c != Card::J {
                let entry = map.entry(c).or_default();
                *entry += 1;
            }
        }

        let mut cs: Vec<_> = map.into_iter().collect();
        cs.sort_by_key(|(_, n)| Reverse(*n));

        let card = if cs.is_empty() { Card::A } else { cs[0].0 };

        Self {
            cards: self.cards,
            bid: self.bid,
            max_cards: self.cards.map(|c| if c == Card::J { card } else { c }),
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        let t = self.rank();
        let t_o = other.rank();

        match t.cmp(&t_o) {
            Ordering::Less => Ordering::Less,
            Ordering::Greater => Ordering::Greater,
            Ordering::Equal => {
                for (c, c_o) in self.cards.iter().copied().zip(other.cards) {
                    match c.cmp(&c_o) {
                        Ordering::Less => return Ordering::Less,
                        Ordering::Greater => return Ordering::Greater,
                        Ordering::Equal => {}
                    }
                }
                Ordering::Equal
            }
        }
    }
}

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
enum Type {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard,
}

impl PartialOrd for Type {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Type {
    fn cmp(&self, other: &Self) -> Ordering {
        fn to_num(t: Type) -> u64 {
            match t {
                Type::FiveOfAKind => 7,
                Type::FourOfAKind => 6,
                Type::FullHouse => 5,
                Type::ThreeOfAKind => 4,
                Type::TwoPair => 3,
                Type::OnePair => 2,
                Type::HighCard => 1,
            }
        }

        to_num(*self).cmp(&to_num(*other))
    }
}

fn parse_hand(input: &str) -> Hand {
    let (hand, bid) = input.trim().split_once(' ').unwrap();

    let mut cards = [Card::A; 5];

    for (i, c) in hand.chars().enumerate() {
        let card = match c {
            'A' => Card::A,
            'K' => Card::K,
            'Q' => Card::Q,
            'J' => Card::J,
            'T' => Card::T,
            c => Card::N(u64::from_str_radix(&c.to_string(), 10).unwrap()),
        };

        cards[i] = card;
    }

    Hand {
        cards,
        bid: u64::from_str_radix(bid, 10).unwrap(),
        max_cards: cards,
    }
    .maximize()
}

#[aoc(day7, part2)]
fn part1(input: &str) -> u64 {
    let cards = input.trim().lines().map(parse_hand).collect::<Vec<_>>();

    let mut sorted = cards.clone();
    sorted.sort();

    let mut total = 0;

    for (i, b) in sorted.iter().enumerate() {
        let i = i as u64 + 1;
        let b = b.bid;

        total += b * i;
    }

    total
}

#[cfg(test)]
mod tests {

    #[test]
    fn part2() {
        let input = "
            32T3K 765
            T55J5 684
            KK677 28
            KTJJT 220
            QQQJA 483
        "
        .trim();

        assert_eq!(super::part1(input), 5905);
    }
}
