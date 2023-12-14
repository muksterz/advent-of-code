
use runner::aoc;


#[derive(Clone, Copy, PartialEq, Eq)]
enum Direction {
    N, S, E, W
}

impl Direction {
    fn next(self, mut row: usize, mut col: usize) -> (usize, usize) {
        match self {
            Direction::N => row = row.saturating_sub(1),
            Direction::S => row += 1,
            Direction::W => col = col.saturating_sub(1),
            Direction::E => col += 1
        }

        (row, col)
    }

    fn on_edge(self, row: usize, col: usize, rows: usize, cols: usize) -> bool {
        match self {
            Direction::N => row == 0,
            Direction::E => col == cols-1,
            Direction::S => row == rows - 1,
            Direction::W => col == 0,
        }
    }
}

#[derive(PartialEq, Eq, Clone)]
struct Grid {
    rows: Vec<Vec<char>>
}

impl Grid {
    fn parse(input: &str) -> Self {
        let mut rows = Vec::new();
        for l in input.trim().lines().map(str::trim) {
            let mut row = Vec::new();
            for c in l.chars() {
                row.push(c);
            }
            rows.push(row)
        }

        Self { rows }
    }

    fn next_free(&self, mut row: usize, mut col: usize, dir: Direction) -> (usize, usize) {

        let rows = self.rows.len();
        let cols = self.rows[0].len();

        assert!(self.rows[row][col] == 'O');


        while !dir.on_edge(row, col, rows, cols) {
            let (new_row, new_col) = dir.next(row, col);
            if self.rows[new_row][new_col] != '.' {
                return (row, col)
            }
            (row, col) = (new_row, new_col);
        }
        (row, col)
    }

    fn slide(&mut self, dir: Direction) {
        let rows = self.rows.len();
        let cols = self.rows[0].len();

        let mut row_iter = Box::new(0..rows) as Box<dyn Iterator<Item = usize>>;
        let mut col_iter: Box<dyn Fn() -> Box<dyn Iterator<Item=usize>>> = Box::new(|| Box::new(0..cols) as Box<dyn Iterator<Item = usize>>);

        match dir {
            Direction::S => row_iter = Box::new((0..rows).rev()) as Box<dyn Iterator<Item = usize>>,
            Direction::E => col_iter = Box::new(|| Box::new((0..cols).rev()) as Box<dyn Iterator<Item = usize>>),
            _ => {}
        }

        for row in row_iter {
            for col in (col_iter)() {
                if self.rows[row][col] == 'O' {
                    let (new_row, new_col) = self.next_free(row, col, dir);
                    self.rows[row][col] = '.';
                    self.rows[new_row][new_col] = 'O';

                }
            }
        }
    }

    fn moment(&self) -> u64 {
        let rows = self.rows.len();
        let mut total = 0;
        for (i, r) in self.rows.iter().enumerate() {
            let moment = rows - i;
            total += r.iter().filter(|&&c| c == 'O').count() * moment;
        }

        total as u64
    }
}

#[aoc(day14, part1)]
fn part1(input: &str) -> u64 {
    let mut grid = Grid::parse(input);

    grid.slide(Direction::N);
    grid.moment()
}

#[aoc(day14, part2)]
fn part2(input: &str) -> u64 {
    let mut grid = Grid::parse(input);

    let mut old = Vec::new();
    old.push(grid.clone());
    let last;

    loop {
        grid.slide(Direction::N);
        grid.slide(Direction::W);
        grid.slide(Direction::S);
        grid.slide(Direction::E);

        if old.contains(&grid) {
            last = grid.clone();
            break;
        } else {
            old.push(grid.clone())
        }

    }

    let mut cycle_start = 0;

    while old[0] != last {
        cycle_start += 1;
        old.remove(0);
    }

    let cycle_len = old.len();
    println!("{cycle_len} {cycle_start}");

    let index = (1_000_000_000 - cycle_start) % cycle_len;

    println!("{index}");

    /*for g in old.iter() {
        println!("\n{}", g.moment());
        for r in g.rows.iter() {
            println!("{}", r.iter().collect::<String>())
        }
    }
    println!("\nLast\n{}:", last.moment());
    for r in last.rows.iter() {
        println!("{}", r.iter().collect::<String>())
    }*/


    old[index].moment()
}

#[cfg(test)]
mod tests {
    const INPUT: &str = "
        O....#....
        O.OO#....#
        .....##...
        OO.#O....O
        .O.....O#.
        O.#..O.#.#
        ..O..#O..O
        .......O..
        #....###..
        #OO..#....
    ";

    #[test]
    fn part1() {
        assert_eq!(super::part1(INPUT.trim()), 136)
    }

    #[test]
    fn part2() {
        assert_eq!(super::part2(INPUT), 64)
    }
}