use aoc_runner_derive::aoc;

#[derive(Debug)]
struct Board {
    board: Vec<Vec<Element>>,
}

impl Board {
    fn parse(input: &str) -> Self {
        let mut board = Vec::new();

        let mut gear_id = 0;

        for line in input.trim().lines() {
            let line = line.trim();
            let mut row = Vec::new();
            let mut prev = None;
            for (i, c) in line.chars().enumerate() {
                match c {
                    '.' => {
                        row.push(Element::Empty);
                        prev = Some(Element::Empty);
                    }
                    '0'..='9' => {
                        if let Some(Element::Number { value, id, .. }) = prev {
                            let e = Element::Number {
                                digit: c,
                                value,
                                id,
                            };
                            prev = Some(e);
                            row.push(e);
                        } else {
                            let s = line[i..]
                                .split_once(|c: char| !c.is_numeric())
                                .map(|(f, _)| f)
                                .unwrap_or(&line[i..]);
                            let num: u64 = s.parse().unwrap();
                            let e = Element::Number {
                                digit: c,
                                value: num,
                                id: gear_id,
                            };
                            gear_id += 1;
                            row.push(e);
                            prev = Some(e);
                        }
                    }
                    c => {
                        row.push(Element::Part(c));
                        prev = Some(Element::Part(c));
                    }
                }
            }

            board.push(row);
        }
        Self { board }
    }

    fn get(&self, row: isize, col: isize) -> Option<Element> {
        let row: usize = row.try_into().ok()?;
        let col: usize = col.try_into().ok()?;

        self.board.get(row)?.get(col).copied()
    }

    fn gear_ratio(&self, row: isize, col: isize) -> Option<u64> {
        let e = self.get(row, col)?;
        if !e.is_symbol() {
            return None;
        }

        let deltas = [
            (-1isize, -1isize),
            (-1, 0),
            (-1, 1),
            (0, -1),
            (0, 1),
            (1, -1),
            (1, 0),
            (1, 1),
        ];

        let mut numbers = 0;
        let mut ratio = 1;

        let mut ids = Vec::new();

        for e in deltas
            .iter()
            .filter_map(|(dr, dc)| self.get(row + dr, col + dc))
        {
            match e {
                Element::Number { value, id, .. } => {
                    if !ids.contains(&id) {
                        numbers += 1;
                        ratio *= value;
                        ids.push(id)
                    }
                }
                _ => {}
            }
        }

        if numbers == 2 {
            return Some(ratio);
        } else {
            None
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Element {
    Empty,
    Number { digit: char, value: u64, id: u64 },
    Part(char),
}

impl Element {
    fn to_char(self) -> char {
        match self {
            Element::Empty => '.',
            Element::Part(c) => c,
            Element::Number { digit, .. } => digit,
        }
    }

    fn is_symbol(self) -> bool {
        match self {
            Element::Part(_) => true,
            _ => false,
        }
    }
}

#[aoc(day3, part1)]
fn part1(input: &str) -> u64 {
    let board = Board::parse(input);

    let mut matches = Vec::new();

    let rows = board.board.len() as isize;
    let cols = board.board[0].len() as isize;

    let mut iter = (0..rows).flat_map(|r| (0..cols).map(move |c| (r, c)));

    while let Some((row, col)) = iter.next() {
        if let Some(v) = p1_recurse_search(&board, row, col) {
            matches.push(v);
            while let Some((row, col)) = iter.next() {
                match board.get(row, col).unwrap() {
                    Element::Number { .. } => {}
                    _ => break,
                }
            }
        }
    }

    let mut output_board = String::new();

    for r in 0..rows {
        for c in 0..cols {
            let e = board.get(r, c).unwrap();
            match e {
                Element::Number { digit, .. } => {
                    if p1_recurse_search(&board, r, c).is_some() {
                        output_board.push('F');
                    } else {
                        output_board.push(digit)
                    }
                }
                e => output_board.push(e.to_char()),
            }
        }
        output_board.push('\n');
    }
    println!("{:?}", std::env::current_dir());

    matches.iter().sum()
}

/// Returns true if there is a symbol around a number at (row, col)
fn p1_recurse_search(board: &Board, row: isize, col: isize) -> Option<u64> {
    fn recurse_inner(board: &Board, row: isize, col: isize) -> Option<u64> {
        let cur_e = board.get(row, col)?;

        let v = if let Element::Number { value, .. } = cur_e {
            value
        } else {
            return None;
        };
        let deltas = [
            (-1isize, -1isize),
            (-1, 0),
            (-1, 1),
            (0, 1),
            (1, -1),
            (1, 0),
            (1, 1),
        ];

        for (row, col) in deltas
            .into_iter()
            .filter_map(|(dr, dc)| Some((row + dr, col + dc)))
        {
            let e = if let Some(e) = board.get(row, col) {
                e
            } else {
                continue;
            };

            match e {
                Element::Part(_) => return Some(v),
                Element::Number { .. } => {
                    if let Some(v) = p1_recurse_search(board, row, col) {
                        return Some(v);
                    }
                }
                _ => {}
            }
        }
        None
    }

    fn check_first(board: &Board, row: isize, col: isize) -> Option<u64> {
        let cur_e = board.get(row, col)?;

        let v = if let Element::Number { value, .. } = cur_e {
            value
        } else {
            return None;
        };

        let col = col - 1;

        let e = board.get(row, col)?;

        match e {
            Element::Part { .. } => Some(v),
            _ => None,
        }
    }

    if let Some(v) = check_first(board, row, col) {
        return Some(v);
    }

    recurse_inner(board, row, col)
}

#[aoc(day3, part2)]
fn part2(input: &str) -> u64 {
    let board = Board::parse(input);

    let mut total = 0;

    let rows = board.board.len() as isize;
    let cols = board.board[0].len() as isize;

    for row in 0..rows {
        for col in 0..cols {
            let ratio = board.gear_ratio(row, col);
            match ratio {
                Some(v) => {
                    total += v;
                }
                None => {}
            }
        }
    }

    total
}

#[cfg(test)]
mod tests {

    #[test]
    fn part1() {
        let input = "
            467..114..
            ...*......
            ..35..633.
            ......#...
            617*......
            .....+.58.
            ..592.....
            ......755.
            ...$.*....
            .664.598..
        ";

        assert_eq!(4361, super::part1(input))
    }

    #[test]
    fn part2() {
        let input = "
        467..114..
        ...*......
        ..35..633.
        ......#...
        617*......
        .....+.58.
        ..592.....
        ......755.
        ...$.*....
        .664.598..
    ";

        assert_eq!(467835, super::part2(input))
    }
}
