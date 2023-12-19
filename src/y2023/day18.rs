use std::collections::HashSet;

use aoc_lib::{grid::AABB, Coord};
use itertools::Itertools;
use runner::aoc;



#[aoc(day18, part1)]
#[allow(unused)]
fn part1(input: &str) -> i64 {
    
    let mut commands = Vec::new();



    for c in input.trim().lines().map(str::trim) {
        let (dir, rest) = c.split_once(' ').unwrap();
        let (dist, _color) = rest.split_once(' ').unwrap();

        let dir = match dir {
            "U" => Coord::N,
            "R" => Coord::E,
            "D" => Coord::S,
            "L" => Coord::W,
            _ => panic!(),
        };

        let dist: i64 = dist.parse().unwrap();

        commands.push((dir, dist));
    }
    solve(&commands)
}


fn solve(commands: &[(Coord, i64)]) -> i64 {
    let mut vertexes = Vec::new();

    let mut pos = Coord::ORIGIN;

    for &(dir, dist) in commands {
        let next = pos + dir * dist;

        vertexes.push(next);
        pos = next;
    }

    let mut x_poses = vertexes.iter().map(|v| v.row).chain(vertexes.iter().map(|v| v.row + 1)).unique().collect::<Vec<_>>();
    let mut y_poses = vertexes.iter().map(|v| v.col).chain(vertexes.iter().map(|v| v.col + 1)).unique().collect::<Vec<_>>();
    dbg!();

    x_poses.sort();
    y_poses.sort();

    let compressed_vertexes = vertexes
        .iter()
        .map(|&v| {
            let x_c = x_poses.iter().position(|&c| c == v.row).unwrap();
            let y_c = y_poses.iter().position(|&c| c == v.col).unwrap();
            Coord::new(x_c as i64, y_c as i64)
        })
        .collect::<Vec<_>>();

    let mut walls: Vec<Coord> = Vec::new();

    let origin = Coord::new(x_poses.iter().position(|&v| v == 0).unwrap() as i64, y_poses.iter().position(|&v| v == 0).unwrap() as i64);

    let mut current = origin;

    let mut area = AABB::new();
    area.add(Coord::new(0, 0));
    area.add(Coord::new(x_poses.len() as i64, y_poses.len() as i64));


    for v in compressed_vertexes.iter().copied() {
        let dist = v - current;
        let dir = Coord::new(dist.row.signum(), dist.col.signum());

        while current != v {
            current += dir;
            walls.push(current);
        }
    }

    let mut filled = HashSet::new();



    let mut last = origin;

    for w in walls.iter().copied() {
        let dir = w - last;
        let dir = dir.rotate_cw_90();
        let c = w + dir;
        flood2(&walls, &mut filled, area, c);
        last = w;
    }

    /*for r in area.rows() {
        for c in r {
            if compressed_vertexes.contains(&c) {
                print!("V")
            } else if walls.contains(&c) {
                print!("#")
            } else if filled.contains(&c) {
                print!("F")
            } else {
                print!(".")
            }
        }
        println!()
    }*/

    let mut area = 0;

    for c in filled.iter().chain(walls.iter()).copied() {
        //println!("Adding area of interior {c:?}");
        area += compressed_area(c, &x_poses, &y_poses);
    }

    

    

    area
}

fn compressed_area(c: Coord, x_c: &[i64], y_c: &[i64]) -> i64 {
    let width = x_c[c.row as usize + 1] - x_c[c.row as usize];
    let height = y_c[c.col as usize + 1] - y_c[c.col as usize];

    width * height
}

fn flood2(walls: &[Coord], filled: &mut HashSet<Coord>, area: AABB, c: Coord) {

    let mut stack = vec![c];
    while let Some(c) = stack.pop() {

        
        if filled.contains(&c) || walls.contains(&c) {
            continue;
        }
        if !area.contains(c) {
            panic!("Out of bounds")
        }

        filled.insert(c);

        let dirs = [Coord::N, Coord::W, Coord::E, Coord::S];

        for d in dirs {
            stack.push(c + d);
        }
    }
}

#[aoc(day18, part2)]
fn part2(input: &str) -> i64 {
    let mut commands: Vec<(Coord, i64)> = Vec::new();

    for c in input.trim().lines().map(str::trim) {
        let c = c.split_whitespace().last().unwrap();
        let c = &c[2..(c.len() - 1)];

        let dir = c.as_bytes()[c.len() - 1];

        let c = &c[..c.len() - 1];

        let dir = match dir {
            b'0' => Coord::E,
            b'1' => Coord::S,
            b'2' => Coord::W,
            b'3' => Coord::N,
            _ => panic!(),
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
