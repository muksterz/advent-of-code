use std::collections::HashMap;

use runner::aoc;

peg::parser! {

    grammar game() for str {

        rule eof() = ![_]

        rule num() -> u64
            = n:$(['0' ..='9']+) { n.parse().unwrap()} /expected!("number")

        rule color() -> String
            = s:$(['a'..='z']+) {s.into()}

        rule draw() -> (String, u64) // Color, Amount
            = n:num() " " c:color() {(c, n)}

        rule round() -> Round
            = r:draw() ** (", ") {

                let mut map = HashMap::new();

                for (c, n) in r {
                    map.insert(c, n);
                }
                Round{draws: map}
            }


        pub rule game() -> Game
            = "Game " id:num() ": " r:(round() ** ("; ")) eof() {
                Game {id, rounds: r}
            }
    }

}

#[derive(Debug)]
pub struct Game {
    id: u64,
    rounds: Vec<Round>,
}

#[derive(Debug)]
pub struct Round {
    draws: HashMap<String, u64>,
}

#[aoc(day2, part1)]
fn part1(input: &str) -> u64 {
    let mut total = 0;

    for l in input.lines().map(str::trim) {
        let mut red = true;
        let mut green = true;
        let mut blue = true;

        let game = game::game(l).unwrap();

        let id = game.id;

        for round in game.rounds {
            for (c, n) in round.draws {
                if c == "red" {
                    red = red & (n <= 12);
                } else if c == "green" {
                    green = green & (n <= 13);
                } else if c == "blue" {
                    blue = blue & (n <= 14);
                }
            }
        }

        if red & green & blue {
            total += id;
        }
    }

    total
}

#[aoc(day2, part2)]
fn part2(input: &str) -> u64 {
    let mut total = 0;

    for l in input.lines().map(str::trim) {
        let mut red = 0;
        let mut green = 0;
        let mut blue = 0;

        let game = game::game(l).unwrap();

        for round in game.rounds {
            for (c, n) in round.draws {
                if c == "red" {
                    red = red.max(n);
                } else if c == "green" {
                    green = green.max(n);
                } else if c == "blue" {
                    blue = blue.max(n);
                }
            }
        }

        total += red * green * blue;
    }

    total
}

#[cfg(test)]
mod tests {

    #[test]
    fn part1() {
        let input = "
        Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
        Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
        Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
        Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
        Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

        assert_eq!(super::part1(input.trim()), 8);
    }

    #[test]
    fn part2() {
        let input = "
            Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
            Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
            Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
            Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
            Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
        ";

        assert_eq!(super::part2(input.trim()), 2286);
    }

    #[test]
    fn parse_test() {
        super::game::game("Game 1: 1 blue, 1 red; 1 blue, 1 green").unwrap();
    }
}
