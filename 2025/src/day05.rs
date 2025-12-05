pub fn solve(file: String) -> (usize, usize) {
    let (ranges_str, avail_str) = file.split_once("\n\n").unwrap();
    let mut ranges: Vec<(usize, usize)> = ranges_str
        .lines()
        .map(|l| {
            let (start, end) = l.split_once('-').unwrap();
            (start.parse().unwrap(), end.parse().unwrap())
        })
        .collect();
    let avail: Vec<usize> = avail_str.lines().map(|l| l.parse().unwrap()).collect();
    ranges.sort_by_key(|r| r.0);
    let part1 = avail
        .iter()
        .filter(|a| ranges.iter().take_while(|r| **a > r.0).any(|r| (r.0..=r.1).contains(a)))
        .count();
    let part2 = ranges.into_iter().fold((0, 0), |(res, end), r| {
        (res + end.max(r.1) - end.max(r.0 - 1), end.max(r.1))
    });
    (part1, part2.0)
}
