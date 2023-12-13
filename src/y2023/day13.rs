use runner::aoc;

struct Grid {
    row_major: Vec<Vec<char>>,
    col_major: Vec<Vec<char>>,
}

impl Grid {
    fn parse(input: &str) -> Self {
        let mut row_major: Vec<Vec<char>> = Vec::new();
        let l = input.trim().lines().next().unwrap().trim();
        let mut column_major = vec![Vec::new(); l.len()];

        for l in input.trim().lines() {
            let l = l.trim();

            row_major.push(l.chars().collect());
            for (i, c) in l.chars().enumerate() {
                column_major[i].push(c)
            }
        }

        Self {
            row_major,
            col_major: column_major,
        }
    }

    fn reflections(&self) -> u64 {
        let r = self.row_reflections();
        let c = self.col_reflections();
        c + r
    }

    fn reflections_smudge(&self) -> u64 {
        self.row_reflections_smudge() + self.col_reflections_smudge()
    }

    fn row_reflections(&self) -> u64 {
        let outer = self.row_major.len();
        for i in 1..outer {
            let mut mirror = true;
            for (i1, i2) in (0..i).rev().zip((i)..outer) {
                if i2 >= outer {
                    continue;
                }
                let v1 = &self.row_major[i1];
                let v2 = &self.row_major[i2];
                if v1 != v2 {
                    mirror = false;
                    break;
                }
            }
            if mirror {
                return i as u64 * 100;
            }
        }

        0
    }

    fn col_reflections(&self) -> u64 {
        let outer = self.col_major.len();
        for i in 1..outer {
            let mut mirror = true;
            for (i1, i2) in (0..i).rev().zip((i)..outer) {
                if i2 >= outer {
                    continue;
                }
                let v1 = &self.col_major[i1];
                let v2 = &self.col_major[i2];
                if v1 != v2 {
                    mirror = false;
                    break;
                }
            }
            if mirror {
                return i as u64;
            }
        }

        0
    }

    fn row_reflections_smudge(&self) -> u64 {
        let outer = self.row_major.len();
        for i in 1..outer {
            let mut diff = 0;
            for (i1, i2) in (0..i).rev().zip((i)..outer) {
                if i2 >= outer {
                    continue;
                }
                let v1 = &self.row_major[i1];
                let v2 = &self.row_major[i2];
                diff += v1.iter().zip(v2.iter()).filter(|(m1, m2)| m1 != m2).count();
            }
            if diff == 1 {
                return i as u64 * 100;
            }
        }

        0
    }

    fn col_reflections_smudge(&self) -> u64 {
        let outer = self.col_major.len();
        for i in 1..outer {
            let mut diff = 0;
            for (i1, i2) in (0..i).rev().zip((i)..outer) {
                if i2 >= outer {
                    continue;
                }
                let v1 = &self.col_major[i1];
                let v2 = &self.col_major[i2];
                diff += v1.iter().zip(v2.iter()).filter(|(m1, m2)| m1 != m2).count();
            }
            if diff == 1 {
                return i as u64;
            }
        }

        0
    }
}

#[aoc(day13, part1)]
fn part1(input: &str) -> u64 {
    let grids: Vec<Grid> = input.split("\n\n").map(Grid::parse).collect();
    let mut total = 0;
    for g in grids {
        total += g.reflections();
    }

    total
}

#[aoc(day13, part2)]
fn part2(input: &str) -> u64 {
    let grids: Vec<Grid> = input.split("\n\n").map(Grid::parse).collect();
    let mut total = 0;
    for g in grids {
        total += g.reflections_smudge();
    }

    total
}

#[cfg(test)]
mod tests {

    #[test]
    fn part1() {
        let input = "
            #.##..##.
            ..#.##.#.
            ##......#
            ##......#
            ..#.##.#.
            ..##..##.
            #.#.##.#.

            #...##..#
            #....#..#
            ..##..###
            #####.##.
            #####.##.
            ..##..###
            #....#..#
        "
        .trim();

        assert_eq!(super::part1(input), 405)
    }

    #[test]
    fn part2() {
        let input = "
            #.##..##.
            ..#.##.#.
            ##......#
            ##......#
            ..#.##.#.
            ..##..##.
            #.#.##.#.

            #...##..#
            #....#..#
            ..##..###
            #####.##.
            #####.##.
            ..##..###
            #....#..#
        "
        .trim();

        assert_eq!(super::part2(input), 400)
    }
}
