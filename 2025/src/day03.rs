pub fn solve(input: String) -> (usize, usize) {
    let part = |b| input.lines().map(max_pack(b)).map(|x| x.1).sum();
    (part(2), part(12))
}

#[rustfmt::skip]
fn max_pack(batteries: usize) -> impl Fn(&str) -> (usize, usize) {
    move |pack| (0..batteries).fold((0, 0), |(min, result), i| {
        pack.as_bytes()[min..(pack.len() - batteries + i + 1)]
            .iter()
            .enumerate()
            .rev()
            .max_by_key(|x| x.1)
            .map(|(ind, digit)| (ind + min + 1, result * 10 + (*digit - b'0') as usize))
            .unwrap()
    })
}
