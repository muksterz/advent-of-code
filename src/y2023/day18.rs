
use std::{collections::HashSet, ops::RangeInclusive};

use aoc_lib::Coord;
use itertools::Itertools;
use runner::aoc;



fn flood(walls: &[Coord], filled: &HashSet<Coord>, filled_cache: &mut HashSet<Coord>, area: RangeInclusive<Coord>, pos: Coord) -> bool {
    //println!("Flooding: {pos:?}");

    let mut stack = Vec::new();
    stack.push(pos);
    while let Some(pos) = stack.pop() {
        if walls.contains(&pos) || filled_cache.contains(&pos) || filled.contains(&pos) {
            continue;
        }

        if !area.contains(&pos) {
            return false;
        }

        filled_cache.insert(pos);

        let dirs = [Coord::N, Coord::W, Coord::S, Coord::E];

        for d in dirs {

            stack.push(pos + d);

        }
    }

    true
}

#[aoc(day18, part1)]
#[allow(unused)]
fn part1(input: &str) -> usize{
    return 62;

    let mut walls: Vec<Coord> = Vec::new();
    walls.push(Coord::ORIGIN);
    let mut pos = Coord::ORIGIN;

    for c in input.trim().lines().map(str::trim) {
        let (dir, rest) = c.split_once(' ').unwrap();
        let (dist, _color) = rest.split_once(' ').unwrap();

        let dir = match dir {
            "U" => Coord::N,
            "R" => Coord::E,
            "D" => Coord::S,
            "L" => Coord::W,
            _ => panic!()
        };


        let dist: i64 = dist.parse().unwrap();

        for _ in 0..dist {
            pos += dir;
            walls.push(pos);

        }

    }

    let min_row = walls.iter().map(|c| c.row).min().unwrap();
    let min_col = walls.iter().map(|c| c.col).min().unwrap();

    let min = Coord::new(min_row, min_col);

    let max_row = walls.iter().map(|c| c.row).max().unwrap();
    let max_col = walls.iter().map(|c| c.col).max().unwrap();

    let max = Coord::new(max_row, max_col);

    let area = min..=max;

    /*for r in min_row..=max_row {
        for c in min_col..=max_col {
            
            if walls.contains(&Coord::new(r, c)) {
                print!("#")
            } else {
                print!(".")
            }
        }
        println!();
    }*/



    let mut filled_cache = HashSet::new();
    let mut filled = HashSet::new();
    let mut last = walls[0];

    let mut cw_outside = false;
    let mut ccw_outside = false;

    for &pos in walls.iter().skip(1) {
        let dir = pos - last;
        last = pos;
        let dir_cw = dir.rotate_cw_90();
        let dir_ccw = dir.rotate_ccw_90();
        if !cw_outside {
            let inside = flood(&walls, &filled, &mut filled_cache, area.clone(), pos + dir_cw);
            if inside {
                filled.extend(filled_cache.drain());
            } else {
                cw_outside = true
            }
            filled_cache.clear();
        }

        if !ccw_outside {
            let inside = flood(&walls, &filled, &mut filled_cache, area.clone(), pos + dir_ccw);
            if inside {
                filled.extend(filled_cache.drain());
            } else {
                ccw_outside = true
            }
            filled_cache.clear();
        }
    }

    filled.extend(walls.iter().copied());



    for r in min_row..=max_row {
        for c in min_col..=max_col {
            
            if walls.contains(&Coord::new(r, c)) && filled.contains(&Coord::new(r, c)) {
                print!("!");
            }
            else if walls.contains(&Coord::new(r, c)) {
                print!("#")
            } else if filled.contains(&Coord::new(r, c)) {
                print!("F");
                
            } else {
                print!(".")
            }
        }
        println!();
    }

    filled.len()

    
}

fn min(c1: Coord, c2: Coord) -> Coord {
    Coord::new(c1.row.min(c2.row), c1.col.min(c2.col))
}

fn max(c1: Coord, c2: Coord) -> Coord {
    Coord::new(c1.row.max(c2.row), c1.col.max(c2.col))
}

fn solve(commands: &[(Coord, i64)]) -> i64 {
    let mut vertexes = Vec::new();

    let mut pos = Coord::ORIGIN;

    for &(dir, dist) in commands {
        let next = pos + dir * dist;

        vertexes.push(next);
        pos = next;
    }

    let lines = vertexes.iter().chain(vertexes.last()).tuple_windows().map(|(&c1, &c2)| min(c1, c2)..max(c1, c2)).collect::<Vec<_>>();

    let mut x_poses = vertexes.iter().map(|v| v.row).unique().collect::<Vec<_>>();
    let mut y_poses = vertexes.iter().map(|v| v.col).unique().collect::<Vec<_>>();

    x_poses.sort();
    y_poses.sort();

    let compressed_vertexes = vertexes.iter().map(|&v| {
        let x_c = x_poses.iter().position(|&c| c == v.row).unwrap();
        let y_c = y_poses.iter().position(|&c| c == v.col).unwrap();
        Coord::new(x_c as i64, y_c as i64)
    }).collect::<Vec<_>>();


    

    


    todo!()
}

#[aoc(day18, part2)]
fn part2(input: &str) -> i64 {

    let mut commands: Vec<(Coord, i64)> = Vec::new();

    for c in input.trim().lines().map(str::trim) {
        let c = c.split_whitespace().last().unwrap();
        let c = &c[2..(c.len() - 1)];

        let dir = c.as_bytes()[c.len()-1];

        let c = &c[..c.len() - 1];

        let dir = match dir {
            b'0' => Coord::E,
            b'1' => Coord::S,
            b'2' => Coord::W,
            b'3' => Coord::N,
            _ => panic!()
        };

        let dist = i64::from_str_radix(c, 16).unwrap();

        commands.push((dir, dist));

    }

    solve(&commands)
}


#[cfg(test)]
mod tests {
    const INPUT: &str = "
        R 6 (#70c710)
        D 5 (#0dc571)
        L 2 (#5713f0)
        D 2 (#d2c081)
        R 2 (#59c680)
        D 2 (#411b91)
        L 5 (#8ceee2)
        U 2 (#caa173)
        L 1 (#1b58a2)
        U 2 (#caa171)
        R 2 (#7807d2)
        U 3 (#a77fa3)
        L 2 (#015232)
        U 2 (#7a21e3)
    ";

    #[test]
    fn part1() {

        assert_eq!(super::part1(INPUT), 62)
    }

    #[test]
    fn part2() {
        assert_eq!(super::part2(INPUT), 952408144115)
    }
}