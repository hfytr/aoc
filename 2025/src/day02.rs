pub fn solve(file: String) -> (usize, usize) {
    file.lines()
        .next()
        .unwrap()
        .split(',')
        .flat_map(|range| {
            let dash = range.find('-').unwrap();
            range[0..dash].parse().unwrap()..=range[dash + 1..].parse().unwrap()
        })
        .fold((0, 0), |(mut acc1, mut acc2), n| {
            acc1 += solve_part(true, n) as usize * n;
            acc2 += solve_part(false, n) as usize * n;
            (acc1, acc2)
        })
}

fn solve_part(only_2: bool, n: usize) -> bool {
    let digits = 1 + n.ilog10() as usize;
    let half_digits = digits / 2;
    let mut interval = if only_2 {
        half_digits..=half_digits
    } else {
        1..=half_digits
    };
    interval.any(|i| {
        if i == 0 || digits % i != 0 {
            return false;
        }
        let repeating = n % 10usize.pow(i as u32);
        let mut state = repeating;
        let mut cur = n;
        (0..digits).all(|j| {
            if j % i == 0 {
                state = repeating;
            }
            let ret = state % 10 == cur % 10;
            state /= 10;
            cur /= 10;
            ret
        })
    })
}
