pub fn solve(input: String) -> (usize, usize) {
    let mut grid: Vec<Vec<bool>> = input
        .lines()
        .map(|l| l.bytes().map(|b| b == b'@').collect())
        .collect();
    (get_removable(&grid).count(), part2(&mut grid))
}

fn get_removable<'a>(grid: &'a Vec<Vec<bool>>) -> impl Iterator<Item = (usize, usize)> + use<'a> {
    (0..grid.len())
        .flat_map(|i| (0..grid[0].len()).map(move |j| (i, j)))
        .filter(|(i, j)| grid[*i][*j] && surrounding(grid, (*i as isize, *j as isize)) < 4)
}

fn part2(grid: &mut Vec<Vec<bool>>) -> usize {
    let mut done = false;
    let mut ret = 0;
    while !done {
        done = true;
        let to_remove: Vec<_> = get_removable(grid).collect();
        to_remove.iter().for_each(|(i, j)| {
            done = false;
            grid[*i][*j] = false;
            ret += 1;
        });
    }
    ret
}

fn surrounding(grid: &Vec<Vec<bool>>, pos: (isize, isize)) -> usize {
    const DIFFS: [(isize, isize); 8] = [
        (0, 1),
        (0, -1),
        (1, 1),
        (1, -1),
        (1, 0),
        (-1, 1),
        (-1, -1),
        (-1, 0),
    ];
    // underflow is fine because we do bounds checking (grid.len() > isize::MAX is unsolvable)
    DIFFS
        .iter()
        .map(|(di, dj)| ((pos.0 + *di) as usize, (pos.1 + *dj) as usize))
        .filter(|(i, j)| {
            (0..grid.len()).contains(i) && (0..grid[0].len()).contains(j) && grid[*i][*j]
        })
        .count()
}
