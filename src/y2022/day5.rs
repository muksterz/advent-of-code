use runner::aoc;

struct Crates {
    stacks: Vec<Vec<char>>,
}

impl Crates {
    fn parse(crates: &str) -> Crates {
        let mut stacks = Vec::new();

        for l in crates.lines() {
            if l.is_empty() {
                break;
            }
            if l.chars().nth(1).unwrap().is_numeric() {
                break;
            }

            let mut iter = l.chars().skip(1);
            let mut index = 0;
            while let Some(c) = iter.next() {
                let v = if let Some(v) = stacks.get_mut(index) {
                    v
                } else {
                    stacks.push(Vec::new());
                    &mut stacks[index]
                };
                if !c.is_whitespace() {
                    v.insert(0, c);
                }
                index += 1;

                iter.next();
                iter.next();
                iter.next();
            }
        }

        Self { stacks }
    }

    fn execute_command(&mut self, c: Command) {
        for _ in 0..c.amount {
            let krate = self.stacks[c.from].pop().unwrap();
            self.stacks[c.to].push(krate)
        }
    }

    fn execute_command_2(&mut self, c: Command) {
        let mut temp = Vec::new();
        let start = self.stacks[c.from].len() - c.amount;

        temp.extend(self.stacks[c.from].drain(start..));

        self.stacks[c.to].extend(temp)
    }
}

#[derive(Debug, Clone, Copy)]
struct Command {
    amount: usize,
    from: usize,
    to: usize,
}

fn parse_commands(input: &str) -> Vec<Command> {
    let iter = input.lines().skip_while(|l| !l.is_empty()).skip(1);
    let mut commands = Vec::new();

    for l in iter {
        let mut nums = l.split(' ').flat_map(|n| n.parse().ok());
        commands.push(Command {
            amount: nums.next().unwrap(),
            from: nums.next().unwrap() - 1,
            to: nums.next().unwrap() - 1,
        })
    }

    commands
}

#[aoc(day5, part1)]
fn part1(input: &str) -> String {
    let mut crates = Crates::parse(input);
    let commands = parse_commands(input);

    for c in commands {
        crates.execute_command(c);
    }

    let mut output = String::new();

    for stack in crates.stacks.iter() {
        output.push(*stack.last().unwrap())
    }

    output
}

#[aoc(day5, part2)]
fn part2(input: &str) -> String {
    let mut crates = Crates::parse(input);
    let commands = parse_commands(input);

    for c in commands {
        crates.execute_command_2(c);
    }

    let mut output = String::new();

    for stack in crates.stacks.iter() {
        output.push(*stack.last().unwrap())
    }

    output
}

#[cfg(test)]
mod tests {
    const INPUT: &str = include_str!("day5_test.txt");
    #[test]
    fn part1() {
        assert_eq!(super::part1(INPUT), "CMZ")
    }

    #[test]
    fn part2() {
        assert_eq!(super::part2(INPUT), "MCD")
    }
}
