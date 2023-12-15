
use std::collections::HashMap;

use runner::aoc;

use aoc_lib::{Grid, Coord};

fn parse(input: &str) -> Grid<i64> {
    let mut grid = Grid::build();
    for l in input.trim().lines().map(str::trim) {
        let mut row = Vec::new();
        for c in l.chars() {
            let n = c as u8 - b'0';
            row.push(n as i64);
        }
        grid.push_row(row)
        
    }
    grid.finish()
}

#[derive(Hash, PartialEq, Eq)]
struct State {
    location: Coord,
    direction: Coord
}

fn highest(cache: &mut HashMap<State, i64>, grid: &Grid<i64>, loc: Coord, dir: Coord) -> i64 {

    if grid.get(loc).is_none() {
        return -1;
    }

    let state = State {location: loc, direction: dir};
    if let Some(&v) = cache.get(&state) {
        return v;
    };

    let next = loc + dir;


    let v = grid[loc].max(highest(cache, grid, next, dir));

    cache.insert(state, v);
    v

}

fn visible(cache: &mut HashMap<State, i64>, grid: &Grid<i64>, loc: Coord) -> bool {
    let dirs = [Coord::N, Coord:: E, Coord::S, Coord::W];
    let cur_height = grid[loc];
    for dir in dirs {
        let h = highest(cache, grid, loc + dir, dir);
        if h < cur_height {
            return true;
        }
    }
    false
}

fn viewing_distance(grid: &Grid<i64>, loc: Coord, dir: Coord) -> i64 {
    let mut distance = 0;
    let mut tree = loc + dir;
    let own_h = grid[loc];
    while let Some(&h) = grid.get(tree) {
        distance += 1;
        if h >= own_h {
            break;
        }

        tree += dir;
    }

    distance
}

fn scenic_score(grid: &Grid<i64>, loc: Coord) -> i64 {
    let dirs = [Coord::N, Coord:: E, Coord::S, Coord::W];
    let mut total = 1;

    for dir in dirs {
        total *= viewing_distance(grid, loc, dir);
    }

    total
}

#[aoc(day8, part1)]
fn part1(input: &str) -> usize {
    let grid = parse(input);
    let mut cache = HashMap::new();
    grid.coords().map(|c| visible(&mut cache, &grid, c)).filter(|b| *b).count()
}

#[aoc(day8, part2)]
fn part2(input: &str) -> i64 {
    let grid = parse(input);
    grid.coords().map(|c| scenic_score(&grid, c)).max().unwrap()
}

#[cfg(test)]
mod tests {
    
    const INPUT: &str = "
        30373
        25512
        65332
        33549
        35390
    ";

    #[test]
    fn part1() {
        assert_eq!(super::part1(INPUT.trim()), 21)
    }

    #[test]
    fn part2() {
        assert_eq!(super::part2(INPUT.trim()), 8)
    }
}