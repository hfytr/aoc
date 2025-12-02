use std::time::Instant;

const DAYS: [fn(String) -> (usize, usize); 2] = [day01::solve, day02::solve];

mod day01;
mod day02;

pub fn solve_day(day: usize) {
    let input = std::fs::read_to_string(&format!("inputs/{:02}", day)).unwrap();
    let start = Instant::now();
    let (part1, part2) = DAYS[day - 1](input);
    let end = Instant::now();
    println!("solved day {:02} in {:?}.", day, end - start);
    println!("part1: {part1}\npart2: {part2}\n");
}

pub fn all_days() {
    (1..=DAYS.len()).for_each(solve_day);
}
