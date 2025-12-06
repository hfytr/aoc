use std::mem::MaybeUninit;

pub fn solve(s: String) -> (usize, usize) {
    (part1(&s), part2(&s))
}

macro_rules! input_len {
    () => { 4 };
}

fn part1<'a>(s: &str) -> usize {
    let mut lines = s.lines().rev();
    let ops = lines.next().unwrap().split_whitespace().map(|s| s == "+");
    let mut nums = lines.map(|l| l.split_whitespace().map(|x| x.parse::<usize>().unwrap()));
    let mut itrs = [nums.next(), nums.next(), nums.next(), nums.next()];
    let mut get_itr = |i: usize| itrs[i].take().unwrap();
    let input = get_itr(0)
        .zip(get_itr(1))
        .zip(get_itr(2))
        .zip(get_itr(3))
        .zip(ops)
        .map(|((((a, b), c), d), op)| (a, b, c, d, op));
    input.fold(0, |ret, (a, b, c, d, op)| {
        if op {
            ret + a + b + c + d
        } else {
            ret + a * b * c * d
        }
    })
}

fn part2(s: &str) -> usize {
    let mut lines = s.lines().rev();
    let mut ops = lines.next().unwrap().split_whitespace().map(|s| s == "+");
    let mut input: [MaybeUninit<Vec<u8>>; input_len!()] = unsafe { MaybeUninit::uninit().assume_init() };
    lines.enumerate().take(input_len!()).for_each(|(i, l)| {
        input[i] = MaybeUninit::new(l.bytes().chain(std::iter::once(b' ')).collect());
    });
    let input: [Vec<u8>; input_len!()] = unsafe { std::mem::transmute(input) };
    let mut nums = vec![];
    let mut ret: usize = 0;
    for i in 0..input[0].len() {
        if input.iter().all(|row| row[i] == b' ') {
            if let Some(true) = ops.next() {
                ret += nums.iter().sum::<usize>();
            } else {
                ret += nums.iter().fold(1, |acc, x| acc * x);
            }
            nums.clear();
        } else {
            nums.push(input.iter().map(|r| r[i]).rev().fold(0, |acc, x| {
                if x == b' ' {
                    acc
                } else {
                    acc * 10 + (x - b'0') as usize
                }
            }));
        }
    }
    ret
}
