use std::collections::HashSet;

use runner::aoc;

use aoc_lib::{Coord, Grid};

fn parse(input: &str) -> Grid<char> {
    let mut grid = Grid::build();
    for l in input.trim().lines().map(str::trim) {
        grid.push_row(l.chars().collect())
    }
    grid.finish()
}

fn follow(cache: &mut HashSet<(Coord, Coord)>, grid: &Grid<char>, loc: Coord, dir: Coord) {
    cache.clear();
    follow_beam(cache, grid, loc, dir)
}

fn follow_beam(cache: &mut HashSet<(Coord, Coord)>, grid: &Grid<char>, loc: Coord, dir: Coord) {
    let mut loc = loc;
    let mut dir = dir;

    loop {
        if cache.contains(&(loc, dir)) {
            return;
        }
        cache.insert((loc, dir));
        let c = if let Some(&c) = grid.get(loc) {
            c
        } else {
            return;
        };
        (loc, dir) = match c {
            '.' => (loc + dir, dir),
            '|' => match dir {
                Coord::E | Coord::W => {
                    follow_beam(cache, grid, loc + Coord::N, Coord::N);
                    (loc + Coord::S, Coord::S)
                }
                Coord::N | Coord::S => (loc + dir, dir),
                _ => panic!(),
            },
            '-' => match dir {
                Coord::N | Coord::S => {
                    follow_beam(cache, grid, loc + Coord::E, Coord::E);
                    (loc + Coord::W, Coord::W)
                }
                Coord::E | Coord::W => (loc + dir, dir),
                _ => panic!(),
            },
            '/' => match dir {
                Coord::N => (loc + Coord::E, Coord::E),
                Coord::S => (loc + Coord::W, Coord::W),
                Coord::W => (loc + Coord::S, Coord::S),
                Coord::E => (loc + Coord::N, Coord::N),
                _ => panic!(),
            },
            '\\' => match dir {
                Coord::N => (loc + Coord::W, Coord::W),
                Coord::S => (loc + Coord::E, Coord::E),
                Coord::E => (loc + Coord::S, Coord::S),
                Coord::W => (loc + Coord::N, Coord::N),
                _ => panic!(),
            },
            _ => panic!(),
        }
    }
}

fn count(cache: &HashSet<(Coord, Coord)>, grid: &Grid<char>, scratch: &mut HashSet<Coord>) -> i64 {
    scratch.clear();
    scratch.extend(cache.iter().map(|p| p.0));

    let mut total = 0;

    let cols = grid.num_cols();

    for c in grid.coords() {
        if scratch.contains(&c) {
            total += 1;
            //print!("#");
        } else {
            //print!(".");
        }

        if c.col == cols - 1 {
            //println!();
        }
    }
    total
}

#[aoc(day16, part1)]
fn part1(input: &str) -> i64 {
    let grid = parse(input);
    let mut cache = HashSet::new();
    let mut scratch = HashSet::new();
    follow_beam(&mut cache, &grid, Coord::new(0, 0), Coord::E);

    count(&cache, &grid, &mut scratch)
}

#[aoc(day16, part2)]
fn part2(input: &str) -> i64 {
    let grid = parse(input);
    let cols = grid.num_cols();
    let rows = grid.num_rows();
    let iter = (0..cols)
        .map(|c| (Coord::new(0, c), Coord::S))
        .chain((0..rows).map(|r| (Coord::new(r, 0), Coord::E)))
        .chain((0..cols).map(|c| (Coord::new(rows - 1, c), Coord::N)))
        .chain((0..rows).map(|r| (Coord::new(r, cols - 1), Coord::W)));

    let mut cache = HashSet::new();
    let mut scratch = HashSet::new();

    let mut out = 0;
    for (start, dir) in iter {
        follow(&mut cache, &grid, start, dir);
        let c = count(&cache, &grid, &mut scratch);
        out = out.max(c);
    }
    out
}

#[cfg(test)]
mod tests {
    const INPUT: &str = "
            .|...\\....
            |.-.\\.....
            .....|-...
            ........|.
            ..........
            .........\\
            ..../.\\\\..
            .-.-/..|..
            .|....-|.\\
            ..//.|....
        ";

    #[test]
    fn part1() {
        assert_eq!(super::part1(INPUT.trim()), 46);
    }

    #[test]
    fn part2() {
        assert_eq!(super::part2(INPUT.trim()), 51)
    }
}
