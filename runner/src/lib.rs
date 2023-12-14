use std::time::Instant;

pub use macros::*;

#[linkme::distributed_slice]
pub static PROBLEMS: [Problem];

pub struct Problem {
    pub year: u64,
    pub day: u64,
    pub part: u64,
    pub f: fn(&str) -> String,
    pub input: &'static str,
}

#[doc(hidden)]
pub mod __internals {
    pub use linkme;
}

pub fn run_recent() {
    let year = PROBLEMS.iter().map(|p| p.year).max().unwrap();
    run_year(year);
}

fn run_problem(p: &Problem) {
    let time = Instant::now();
    let result = (p.f)(p.input);
    let elapsed = time.elapsed();
    println!("[AOC - {} - day {} - part {}]", p.year, p.day, p.part);
    println!("\tresult: {result}");
    println!("\ttime: {}\n", humantime::format_duration(elapsed))
}

pub fn run_year(year: u64) {
    let day = PROBLEMS
        .iter()
        .filter(|p| p.year == year)
        .map(|p| p.day)
        .max()
        .unwrap();

    let part1 = PROBLEMS
        .iter()
        .filter(|p| p.day == day && p.year == year && p.part == 1)
        .next();
    let part2 = PROBLEMS
        .iter()
        .filter(|p| p.day == day && p.year == year && p.part == 2)
        .next();

    if let Some(p1) = part1 {
        run_problem(p1);
    }

    if let Some(p2) = part2 {
        run_problem(p2)
    }
}
