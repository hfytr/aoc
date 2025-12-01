pub fn solve(file: String) -> (usize, usize) {
    let ((part1, _), (part2, _)) = file
        .lines()
        .map(|line| {
            let factor = if line.as_bytes()[0] == b'L' { 1 } else { -1 };
            factor * line[1..].parse::<isize>().unwrap()
        })
        .fold(((0, 50), (0, 50)), |(acc1, acc2), diff| {
            let acc1 = part1(acc1, diff);
            let acc2 = part2(acc2, diff);
            (acc1, acc2)
        });
    (part1 as usize, part2 as usize)
}

fn part1((res, pos): (isize, isize), diff: isize) -> (isize, isize) {
    let extra = (pos + diff).rem_euclid(100) == 0;
    (res + extra as isize, (pos + diff).rem_euclid(100))
}

fn part2((res, pos): (isize, isize), diff: isize) -> (isize, isize) {
    let extra = !(1..100).contains(&(diff % 100 + pos)) && pos != 0;
    let res = res + (diff / 100).abs() + extra as isize;
    let pos = (pos + diff).rem_euclid(100);
    (res, pos)
}
