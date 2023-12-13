use std::collections::HashMap;

use runner::aoc;

peg::parser! {
    grammar node() for str {
        rule node_name() = ['A'..='Z' | '0'..='9']+
        pub rule node() -> (String, String, String) =
            s1:$node_name() " = (" s2:$node_name() ", " s3:$node_name() ")" {
                (s1.into(), s2.into(), s3.into())
            }
    }
}

#[derive(Debug)]
struct Node {
    node: String,
    left: String,
    right: String,
}

#[derive(Debug)]
struct Tree {
    nodes: HashMap<String, Node>,
}

impl Tree {
    fn parse(input: &str) -> Self {
        let mut nodes = HashMap::new();
        for line in input.lines() {
            let (node, l, r) = node::node(line.trim()).unwrap();
            nodes.insert(
                node.clone(),
                Node {
                    node,
                    left: l,
                    right: r,
                },
            );
        }

        Self { nodes }
    }
}

#[aoc(day8, part1)]
fn part1(input: &str) -> u64 {
    let (pattern, nodes) = input.split_once("\n\n").unwrap();

    let mut index = 0usize;

    let tree = Tree::parse(nodes);

    let mut current = tree.nodes.get("AAA").unwrap();

    let mut count = 0;

    while current.node != "ZZZ" {
        count += 1;
        let dir = pattern.chars().nth(index).unwrap();
        index += 1;
        if index == pattern.len() {
            index = 0;
        }

        //println!("At {}, going {dir}", current.node);
        if dir == 'R' {
            current = tree.nodes.get(&current.right).unwrap();
        } else if dir == 'L' {
            current = tree.nodes.get(&current.left).unwrap();
        } else {
            panic!()
        }
    }

    count
}

#[aoc(day8, part2)]
fn part2(input: &str) -> u64 {
    let (pattern, nodes) = input.split_once("\n\n").unwrap();

    let tree = Tree::parse(nodes);

    let starting_nodes: Vec<&Node> = tree
        .nodes
        .values()
        .filter(|n| n.node.contains('A'))
        .collect();
    let mut cycles = vec![0u64; starting_nodes.len()];

    for (i, start) in starting_nodes.iter().copied().enumerate() {
        let mut index = 0;

        let mut current = start;

        let mut count = 0;

        while !current.node.contains('Z') {
            count += 1;
            let dir = pattern.chars().nth(index).unwrap();
            index += 1;
            if index == pattern.len() {
                index = 0;
            }

            if dir == 'R' {
                current = tree.nodes.get(&current.right).unwrap();
            } else if dir == 'L' {
                current = tree.nodes.get(&current.left).unwrap();
            } else {
                panic!()
            }
        }

        cycles[i] = count;
    }

    cycles.iter().copied().reduce(lcm).unwrap()
}

fn lcm(a: u64, b: u64) -> u64 {
    let gcd = gcd(a, b);

    a * b / gcd
}

fn gcd(a: u64, b: u64) -> u64 {
    let mut result = a.min(b);

    while result > 0 {
        if a % result == 0 && b % result == 0 {
            break;
        }
        result -= 1;
    }
    result
}

#[cfg(test)]
mod tests {

    #[test]
    fn part1() {
        let input = "
            RL

            AAA = (BBB, CCC)
            BBB = (DDD, EEE)
            CCC = (ZZZ, GGG)
            DDD = (DDD, DDD)
            EEE = (EEE, EEE)
            GGG = (GGG, GGG)
            ZZZ = (ZZZ, ZZZ)
        "
        .trim();
        assert_eq!(super::part1(input), 2)
    }

    #[test]
    fn part2() {
        let input = "
            LR

            11A = (11B, XXX)
            11B = (XXX, 11Z)
            11Z = (11B, XXX)
            22A = (22B, XXX)
            22B = (22C, 22C)
            22C = (22Z, 22Z)
            22Z = (22B, 22B)
            XXX = (XXX, XXX)
        "
        .trim();

        assert_eq!(super::part2(input), 6);
    }
}
