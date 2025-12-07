pub fn solve(s: String) -> (usize, usize) {
    let mut part1 = 0;
    let traverse_part1 = |next: &mut Vec<usize>, cur: &Vec<usize>, split: bool, i| {
        if split && cur[i] == 1 {
            next.get_mut(i - 1).map(|ni| *ni |= 1);
            next.get_mut(i + 1).map(|ni| *ni |= 1);
            part1 += 1;
        } else {
            next[i] |= cur[i];
        }
    };
    let traverse_part2 = |next: &mut Vec<usize>, cur: &Vec<usize>, split: bool, i: usize| {
        if split {
            next.get_mut(i - 1).map(|ni| *ni += cur[i]);
            next.get_mut(i + 1).map(|ni| *ni += cur[i]);
        } else {
            next[i] += cur[i];
        }
    };
    let _ = traverse(&s, traverse_part1);
    (part1, traverse(&s, traverse_part2).iter().sum())
}

fn traverse<F>(s: &str, mut callback: F) -> Vec<usize>
where
    F: FnMut(&mut Vec<usize>, &Vec<usize>, bool, usize),
{
    let mut lines = s.lines();
    let first_line = lines.next().unwrap();
    let mut containers = [vec![0; first_line.len()], vec![0; first_line.len()]];
    let mut cur_container = 0;
    let start = first_line
        .bytes()
        .enumerate()
        .find(|c| c.1 == b'S')
        .unwrap()
        .0;
    containers[cur_container][start] = 1;
    for line in lines {
        let next_container = 1 - cur_container;
        containers[next_container].iter_mut().for_each(|x| {
            *x = 0;
        });
        for (i, c) in line.bytes().enumerate() {
            let [next_mut, cur_mut] = containers
                .get_disjoint_mut([next_container, cur_container])
                .unwrap();
            callback(next_mut, &cur_mut, c == b'^', i);
        }
        cur_container = next_container;
    }
    let [ret, _] = containers;
    ret
}
