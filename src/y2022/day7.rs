use std::{collections::HashMap, iter::Peekable};

use runner::aoc;

#[derive(Default)]
struct FileSystem {
    files: HashMap<Path, File>,
    directories: HashMap<Path, Directory>,
}

impl FileSystem {
    fn compute_size(&mut self, dir_p: &Path) -> u64 {
        let mut size = 0;

        let dir = self.directories.get(dir_p).cloned().unwrap();

        if let Some(s) = dir.size {
            return s;
        }

        for f in dir.files.iter().map(|f| self.files.get(f).unwrap()) {
            size += f.size;
        }

        for p in dir.directories.iter() {
            size += self.compute_size(p);
        }

        self.directories.get_mut(dir_p).unwrap().size = Some(size);

        size
    }
}

#[derive(Clone, Hash, PartialEq, Eq, Debug)]
struct Path {
    parts: Vec<String>,
}

impl Path {
    fn root() -> Self {
        Self { parts: Vec::new() }
    }

    fn pop(&mut self) {
        self.parts.pop();
    }
    fn push(&mut self, s: String) {
        self.parts.push(s)
    }
}

#[derive(Default, Debug, Clone)]
struct Directory {
    files: Vec<Path>,
    directories: Vec<Path>,
    size: Option<u64>,
}

#[derive(Default, Debug)]
struct File {
    size: u64,
}

fn ls<'a>(fs: &mut FileSystem, dir: &Path, iter: &mut Peekable<impl Iterator<Item = &'a str>>) {
    while iter.peek().is_some() && !iter.peek().unwrap().starts_with('$') {
        let file = iter.next().unwrap();
        if file.starts_with("dir") {
            let (_, name) = file.split_once(' ').unwrap();
            let mut p = dir.clone();
            p.push(name.into());
            fs.directories.insert(p.clone(), Directory::default());
            let c_dir = fs.directories.get_mut(dir).unwrap();
            c_dir.directories.push(p);
        } else {
            let (size, name) = file.split_once(' ').unwrap();
            let size = size.parse().unwrap();
            let mut p = dir.clone();
            p.push(name.into());
            fs.files.insert(p.clone(), File { size });
            let c_dir = fs.directories.get_mut(dir).unwrap();
            c_dir.files.push(p);
        }
    }
}

fn create_fs(input: &str) -> FileSystem {
    let mut commands = input.trim().lines().map(str::trim).skip(1).peekable();

    let mut fs = FileSystem::default();
    fs.directories.insert(Path::root(), Directory::default());

    let mut working_dir = Path::root();

    while let Some(c) = commands.next() {
        if c.starts_with("$ ls") {
            ls(&mut fs, &working_dir, &mut commands);
        } else if c.starts_with("$ cd") {
            let d = c.split_whitespace().last().unwrap();
            if d == ".." {
                working_dir.pop();
            } else {
                working_dir.push(d.into())
            }
        }
    }
    fs
}

#[aoc(day7, part1)]
fn part1(input: &str) -> u64 {
    let mut fs = create_fs(input);
    println!();
    let dirs: Vec<Path> = fs.directories.keys().cloned().collect();

    let mut total = 0;

    for d in dirs.iter() {
        let size = fs.compute_size(d);

        if size <= 100000 {
            total += size
        }
    }

    total
}

#[aoc(day7, part2)]
fn part2(input: &str) -> u64 {
    let mut fs = create_fs(input);
    let total_size = 70000000;
    let space_needed = 30000000;
    let remaining = total_size - fs.compute_size(&Path::root());

    let space_needed = space_needed - remaining;

    let mut smallest_size = u64::MAX;

    let dirs = fs.directories.keys().cloned().collect::<Vec<_>>();

    for p in dirs.iter() {
        let size = fs.compute_size(p);
        if size >= space_needed {
            smallest_size = smallest_size.min(size);
        }
    }

    smallest_size
}

#[cfg(test)]
mod tests {

    #[test]
    fn part1() {
        let input = "
            $ cd /
            $ ls
            dir a
            14848514 b.txt
            8504156 c.dat
            dir d
            $ cd a
            $ ls
            dir e
            29116 f
            2557 g
            62596 h.lst
            $ cd e
            $ ls
            584 i
            $ cd ..
            $ cd ..
            $ cd d
            $ ls
            4060174 j
            8033020 d.log
            5626152 d.ext
            7214296 k
        "
        .trim();

        assert_eq!(super::part1(input), 95437);
    }

    #[test]
    fn part2() {
        let input = "
            $ cd /
            $ ls
            dir a
            14848514 b.txt
            8504156 c.dat
            dir d
            $ cd a
            $ ls
            dir e
            29116 f
            2557 g
            62596 h.lst
            $ cd e
            $ ls
            584 i
            $ cd ..
            $ cd ..
            $ cd d
            $ ls
            4060174 j
            8033020 d.log
            5626152 d.ext
            7214296 k
        "
        .trim();

        assert_eq!(super::part2(input), 24933642);
    }
}
