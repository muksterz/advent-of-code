use std::collections::{BinaryHeap, HashSet};

use aoc_lib::{Coord, Grid};
use runner::aoc;

fn parse(input: &str) -> Grid<i64> {
    let mut grid = Grid::build();

    for l in input.trim().lines().map(str::trim) {
        grid.push_row(l.chars().map(|c| c.to_string().parse().unwrap()).collect());
    }

    grid.finish()
}

#[derive(Hash, PartialEq, Eq, Clone, Copy, Debug)]
struct State {
    loc: Coord,
    run_len: i64,
    run_dir: Coord,
    heat: i64,
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.heat.cmp(&self.heat)
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct CachedState {
    loc: Coord,
    run_len: i64,
    run_dir: Coord,
}

fn solve1(grid: &Grid<i64>) -> i64 {
    let mut visited: HashSet<CachedState> = HashSet::new();

    let mut queue = BinaryHeap::new();

    let end = Coord::new(grid.num_rows() - 1, grid.num_cols() - 1);
    let start = State {
        loc: Coord::ORIGIN,
        run_len: 0,
        run_dir: Coord::E,
        heat: 0,
    };

    queue.push(start);


    while let Some(c) = queue.pop() {
        let cached_state = CachedState {
            loc: c.loc,
            run_len: c.run_len,
            run_dir: c.run_dir,
        };

        if !visited.insert(cached_state) {
            continue;
        }

        if c.loc == end {
            return c.heat;
        }

        let dirs = [
            c.run_dir.rotate_ccw_90(),
            c.run_dir.rotate_cw_90(),
            c.run_dir,
        ];

        for d in dirs {
            let run_len = if d == c.run_dir {
                if c.run_len == 3 {
                    continue;
                }
                c.run_len + 1
            } else {
                1
            };

            let next_loc = c.loc + d;
            let next_loss = if let Some(&h) = grid.get(next_loc) {
                h
            } else {
                continue;
            };

            let state = State {
                loc: next_loc,
                run_len,
                run_dir: d,
                heat: c.heat + next_loss,
            };


            queue.push(state)
        }
    }

    panic!()
}

fn solve2(grid: &Grid<i64>) -> i64 {
    let mut visited: HashSet<CachedState> = HashSet::new();

    let mut queue = BinaryHeap::new();

    let end = Coord::new(grid.num_rows() - 1, grid.num_cols() - 1);
    let start = State {
        loc: Coord::ORIGIN,
        run_len: 0,
        run_dir: Coord::E,
        heat: 0,
    };

    queue.push(start);

    let start = State {
        loc: Coord::ORIGIN,
        run_len: 0,
        run_dir: Coord::S,
        heat: 0,
    };

    queue.push(start);

    while let Some(c) = queue.pop() {
        let cached_state = CachedState {
            loc: c.loc,
            run_len: c.run_len,
            run_dir: c.run_dir,
        };

        if !visited.insert(cached_state) {
            continue;
        }

        if c.loc == end {
            if c.run_len >= 4 {
                return c.heat;
            } else {
                continue;
            }
        }

        let dirs = [
            c.run_dir.rotate_ccw_90(),
            c.run_dir.rotate_cw_90(),
            c.run_dir,
        ];

        for d in dirs {
            let run_len = if c.run_len < 4 {
                if d != c.run_dir {
                    continue;
                }
                c.run_len + 1
            } else if d == c.run_dir {
                if c.run_len == 10 {
                    continue;
                }
                c.run_len + 1
            } else {
                1
            };

            let next_loc = c.loc + d;
            let next_loss = if let Some(&h) = grid.get(next_loc) {
                h
            } else {
                continue;
            };


            let state = State {
                loc: next_loc,
                run_len,
                run_dir: d,
                heat: c.heat + next_loss,
            };



            queue.push(state)
        }
    }

    panic!()
}

#[aoc(day17, part1)]
fn part1(input: &str) -> i64 {
    let grid = parse(input);

    solve1(&grid)
}

#[aoc(day17, part2)]
fn part2(input: &str) -> i64 {
    let grid = parse(input);
    solve2(&grid)
}

#[cfg(test)]
mod tests {
    const INPUT: &str = "
    2413432311323
    3215453535623
    3255245654254
    3446585845452
    4546657867536
    1438598798454
    4457876987766
    3637877979653
    4654967986887
    4564679986453
    1224686865563
    2546548887735
    4322674655533
    ";

    #[test]
    fn part1() {
        assert_eq!(super::part1(INPUT), 102)
    }

    #[test]
    fn part2() {
        //assert_eq!(super::part2(INPUT), 94);
        let input = "
        111111111111
        999999999991
        999999999991
        999999999991
        999999999991
        ";

        assert_eq!(super::part2(input), 71);
    }
}
