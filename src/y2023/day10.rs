use std::ops::{Add, Index, IndexMut, Sub};

use runner::aoc;

#[derive(Debug)]
struct Grid {
    cells: Vec<Vec<Cell>>,
    outside: Sector,
}

impl Grid {
    fn parse(input: &str) -> Self {
        let mut rows = Vec::new();
        for l in input.trim().lines() {
            let mut row = Vec::new();
            for c in l.trim().chars() {
                row.push(Cell::new(Element::from_char(c)));
            }
            rows.push(row);
        }

        Self {
            cells: rows,
            outside: Sector::Unknown,
        }
    }

    fn get(&self, c: Coord) -> Option<&Cell> {
        self.cells
            .get(c.row as usize)
            .and_then(|s| s.get(c.col as usize))
    }

    fn start_pos(&self) -> Coord {
        let rows = self.cells.len();
        let cols = self.cells[0].len();

        for r in 0..rows {
            for c in 0..cols {
                if self.cells[r][c].ty == Element::Start {
                    return Coord::new(r as isize, c as isize);
                }
            }
        }

        panic!()
    }

    fn start_offsets(&self, start: Coord) -> [Coord; 2] {
        assert_eq!(self[start].ty, Element::Start);

        let mut matches = Vec::new();

        if matches!(
            self.get(start.north()).map(|c| c.ty),
            Some(Element::Vertical) | Some(Element::SE) | Some(Element::SW)
        ) {
            matches.push(start.north());
        }

        if matches!(
            self.get(start.south()).map(|c| c.ty),
            Some(Element::Vertical) | Some(Element::NE) | Some(Element::NW)
        ) {
            matches.push(start.south())
        }

        if matches!(
            self.get(start.east()).map(|c| c.ty),
            Some(Element::Horizontal) | Some(Element::NW) | Some(Element::SW)
        ) {
            matches.push(start.east())
        }

        if matches!(
            self.get(start.west()).map(|c| c.ty),
            Some(Element::Horizontal) | Some(Element::NE) | Some(Element::SE)
        ) {
            matches.push(start.west())
        }

        assert!(matches.len() == 2);

        [matches[0], matches[1]]
    }

    fn find_next(&self, pos: Coord, from: Coord) -> Coord {
        let offsets = self[pos].offsets();

        offsets
            .into_iter()
            .map(|c| c + pos)
            .find(|&n| n != from)
            .unwrap()
    }
}

impl Index<Coord> for Grid {
    type Output = Cell;

    fn index(&self, coord: Coord) -> &Self::Output {
        &self.cells[coord.row as usize][coord.col as usize]
    }
}

impl IndexMut<Coord> for Grid {
    fn index_mut(&mut self, coord: Coord) -> &mut Self::Output {
        &mut self.cells[coord.row as usize][coord.col as usize]
    }
}

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
struct Cell {
    dist: u64,
    ty: Element,
    sector: Sector,
}

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
enum Sector {
    Line,
    S1,
    S2,
    Unknown,
}

impl Cell {
    fn new(e: Element) -> Self {
        Self {
            dist: u64::MAX,
            ty: e,
            sector: Sector::Unknown,
        }
    }

    fn offsets(&self) -> [Coord; 2] {
        self.ty.offsets()
    }
}

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
enum Element {
    Horizontal,
    Vertical,
    NE,
    NW,
    SE,
    SW,
    Ground,
    Start,
}

impl Element {
    fn from_char(c: char) -> Self {
        match c {
            '|' => Element::Vertical,
            '-' => Element::Horizontal,
            'L' => Element::NE,
            'J' => Element::NW,
            '7' => Element::SW,
            'F' => Element::SE,
            '.' => Element::Ground,
            'S' => Element::Start,
            _ => panic!(),
        }
    }

    // Row Col
    fn offsets(self) -> [Coord; 2] {
        let zero = Coord::new(0, 0);

        match self {
            Element::Vertical => [zero.north(), zero.south()],
            Element::Horizontal => [zero.east(), zero.west()],
            Element::NE => [zero.north(), zero.east()],
            Element::NW => [zero.north(), zero.west()],
            Element::SW => [zero.south(), zero.west()],
            Element::SE => [zero.south(), zero.east()],
            _ => panic!("No valid offsets for {self:?}"),
        }
    }
}

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
struct Coord {
    row: isize,
    col: isize,
}

impl Coord {
    const ZERO: Self = Self::new(0, 0);

    const N: Self = Self::ZERO.north();
    const NE: Self = Self::ZERO.north().east();
    const E: Self = Self::ZERO.east();
    const SE: Self = Self::ZERO.south().east();
    const S: Self = Self::ZERO.south();
    const SW: Self = Self::ZERO.south().west();
    const W: Self = Self::ZERO.west();
    const NW: Self = Self::ZERO.north().west();

    const fn new(row: isize, col: isize) -> Self {
        Self { row, col }
    }

    const fn north(mut self) -> Self {
        self.row -= 1;
        self
    }

    const fn south(mut self) -> Self {
        self.row += 1;
        self
    }

    const fn east(mut self) -> Self {
        self.col += 1;
        self
    }

    const fn west(mut self) -> Self {
        self.col -= 1;
        self
    }

    fn offset(self, d: Direction) -> Self {
        match d {
            Direction::N => self.north(),
            Direction::NE => self.north().east(),
            Direction::E => self.east(),
            Direction::SE => self.south().east(),
            Direction::S => self.south(),
            Direction::SW => self.south().west(),
            Direction::W => self.west(),
            Direction::NW => self.north().west(),
        }
    }

    fn dir(self, next: Coord) -> Direction {
        let diff = next - self;

        match diff {
            Self::N => Direction::N,
            Self::NE => Direction::NE,
            Self::E => Direction::E,
            Self::SE => Direction::SE,
            Self::S => Direction::S,
            Self::SW => Direction::SW,
            Self::W => Direction::W,
            Self::NW => Direction::NW,
            _ => panic!(),
        }
    }
}

impl Add for Coord {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            row: self.row + rhs.row,
            col: self.col + rhs.col,
        }
    }
}

impl Sub for Coord {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            row: self.row - rhs.row,
            col: self.col - rhs.col,
        }
    }
}

#[aoc(day10, part1)]
fn part1(input: &str) -> u64 {
    let mut grid = Grid::parse(input);

    let start = grid.start_pos();

    let start_offsets = grid.start_offsets(start);

    for c in start_offsets {
        let mut dist = 1;

        let mut last = start;
        let mut current = c;

        grid[current].dist = dist.min(grid[current].dist);

        while grid[current].ty != Element::Start {
            let next = grid.find_next(current, last);

            last = current;
            current = next;
            dist += 1;
            grid[current].dist = dist.min(grid[current].dist);
        }
    }

    grid[start].dist = 0;

    grid.cells
        .into_iter()
        .flatten()
        .map(|g| g.dist)
        .filter(|&d| d != u64::MAX)
        .max()
        .unwrap()
}

#[repr(i64)]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Direction {
    N = 0,
    NE,
    E,
    SE,
    S,
    SW,
    W,
    NW,
}

impl Direction {
    fn from_i64(i: i64) -> Self {
        match i {
            0 => Self::N,
            1 => Self::NE,
            2 => Self::E,
            3 => Self::SE,
            4 => Self::S,
            5 => Self::SW,
            6 => Self::W,
            7 => Self::NW,
            _ => {
                panic!()
            }
        }
    }

    fn sweep_cw(self, to: Direction) -> Vec<Direction> {
        let mut out = Vec::new();

        let mut pos = self as i64;

        while pos != to as i64 {
            pos += 1;
            if pos >= 8 {
                pos = 0;
            }
            out.push(Direction::from_i64(pos));
        }

        out
    }

    fn sweep_ccw(self, to: Direction) -> Vec<Direction> {
        let mut out = Vec::new();

        let mut pos = self as i64;

        while pos != to as i64 {
            pos -= 1;
            if pos < 0 {
                pos = 7;
            }
            out.push(Direction::from_i64(pos));
        }

        out
    }
}

#[aoc(day10, part2)]
fn part2(input: &str) -> u64 {
    let mut grid = Grid::parse(input);

    let start = grid.start_pos();

    let offset = grid.start_offsets(start)[0];

    let mut last = start;
    let mut current = offset;
    grid[start].sector = Sector::Line;
    grid[current].sector = Sector::Line;

    while grid[current].ty != Element::Start {
        let next = grid.find_next(current, last);
        grid[next].sector = Sector::Line;

        last = current;
        current = next;
    }

    let mut last = start;
    let mut current = offset;

    while grid[current].ty != Element::Start {
        let next = grid.find_next(current, last);

        let dir = current.dir(last);
        let dir_n = current.dir(next);

        let s1 = dir.sweep_cw(dir_n);

        for d in s1 {
            let c = current.offset(d);
            flood(&mut grid, Sector::S1, c);
        }
        let s2 = dir.sweep_ccw(dir_n);
        for d in s2 {
            let c = current.offset(d);
            flood(&mut grid, Sector::S2, c);
        }

        last = current;
        current = next;
    }

    let outside = grid.outside;

    let inside = if outside == Sector::S1 {
        Sector::S2
    } else {
        Sector::S1
    };

    vis(&grid);

    grid.cells
        .into_iter()
        .flatten()
        .filter(|c| c.sector == inside)
        .count() as u64
}

fn flood(grid: &mut Grid, value: Sector, coord: Coord) {
    if grid.get(coord).is_none() {
        grid.outside = value;
        return;
    }
    if grid.get(coord).is_some() && grid[coord].sector != Sector::Unknown {
        return;
    }

    grid[coord].sector = value;

    flood(grid, value, coord.north());
    flood(grid, value, coord.south());
    flood(grid, value, coord.east());
    flood(grid, value, coord.west());
}

fn vis(grid: &Grid) {
    let mut s = String::new();

    for r in &grid.cells {
        for c in r {
            let c = match c.sector {
                Sector::Line => 'L',
                Sector::S1 => '1',
                Sector::S2 => '2',
                Sector::Unknown => 'U',
            };
            s.push(c);
        }
        s.push('\n');
    }
    //println!("{s}");
}

#[cfg(test)]
mod tests {

    #[test]
    fn part1() {
        let input = "7-F7-
        .FJ|7
        SJLL7
        |F--J
        LJ.LJ"
            .trim();

        assert_eq!(super::part1(input), 8)
    }

    #[test]
    fn part2() {
        let input = "FF7FSF7F7F7F7F7F---7
        L|LJ||||||||||||F--J
        FL-7LJLJ||||||LJL-77
        F--JF--7||LJLJ7F7FJ-
        L---JF-JLJ.||-FJLJJ7
        |F|F-JF---7F7-L7L|7|
        |FFJF7L7F-JF7|JL---7
        7-L-JL7||F7|L7F-7F7|
        L.L7LFJ|||||FJL7||LJ
        L7JLJL-JLJLJL--JLJ.L"
            .trim();

        assert_eq!(10, super::part2(input))
    }
}
