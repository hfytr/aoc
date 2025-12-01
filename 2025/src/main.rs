fn main() {
    let day = std::env::args().nth(1);
    if let Some(day) = day {
        let day = day.parse::<usize>().unwrap();
        aoc::solve_day(day);
    } else {
        aoc::all_days();
    }
}
