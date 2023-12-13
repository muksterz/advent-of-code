use runner::aoc;
use itertools::Itertools;

#[derive(Debug, Clone)]
struct Grid {
    places: Vec<Place>,
    rows: usize,
    cols: usize,
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
struct Place {
    x: i64,
    y: i64,
}

impl Grid {
    fn parse(input: &str) -> Self {
        let input = input.trim();
        let mut vec = Vec::new();
        let lines: Vec<&str> = input.trim().lines().map(|l| l.trim()).collect();

        let rows = lines.len();
        let cols = lines[0].len();

        for y in 0..rows {
            for x in 0..cols {
                if lines[y].chars().nth(x).unwrap() == '#' {
                    vec.push(Place {
                        x: x as i64,
                        y: y as i64,
                    });
                }
            }
        }

        Self {
            places: vec,
            rows,
            cols,
        }
    }

    fn expand(&mut self, by: i64) {
        let by = by - 1;

        for c in (0..self.cols).rev() {
            if self.places.iter().all(|p| p.x != c as i64) {
                self.cols += by as usize;
                for p in self.places.iter_mut() {
                    if p.x > c as i64 {
                        p.x += by;
                    }
                }
            }
        }

        for r in (0..self.rows).rev() {
            if self.places.iter().all(|p| p.y != r as i64) {
                self.rows += by as usize;
                for p in self.places.iter_mut() {
                    if p.y > r as i64 {
                        p.y += by;
                    }
                }
            }
        }
    }
}

#[aoc(day11, part1)]
fn part1(input: &str) -> u64 {
    let mut grid = Grid::parse(input);
    grid.expand(2);

    let mut total = 0;

    for v in grid.places.iter().combinations(2) {
        let p1 = v[0];
        let p2 = v[1];

        let dx = p1.x - p2.x;
        let dy = p1.y - p2.y;

        let dx = dx.abs() as u64;
        let dy = dy.abs() as u64;

        total += dy + dx;
    }

    total
}

#[aoc(day11, part2)]
fn part2(input: &str) -> u64 {
    let mut grid = Grid::parse(input);
    grid.expand(1000000);

    let mut total = 0;

    for v in grid.places.iter().combinations(2) {
        let p1 = v[0];
        let p2 = v[1];

        let dx = p1.x - p2.x;
        let dy = p1.y - p2.y;

        let dx = dx.abs() as u64;
        let dy = dy.abs() as u64;

        let d = dx + dy;

        total += d;
    }

    total
}

#[cfg(test)]
mod tests {
    #[test]
    fn part1() {
        let input = "
        ...#......
        .......#..
        #.........
        ..........
        ......#...
        .#........
        .........#
        ..........
        .......#..
        #...#.....
    "
        .trim();

        assert_eq!(super::part1(input), 374)
    }

    #[test]
    #[ignore = "Only works when expand = 1000"]
    fn part2() {
        let input = "
            ...#......
            .......#..
            #.........
            ..........
            ......#...
            .#........
            .........#
            ..........
            .......#..
            #...#.....
        "
        .trim();

        assert_eq!(super::part2(input), 8410)
    }
}
