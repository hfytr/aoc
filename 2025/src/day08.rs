use std::{collections::BinaryHeap, ops::Mul};

#[derive(Debug)]
struct DisjointSets {
    // parent if not root, else subtree size
    nodes: Vec<usize>,
    pub is_root: Vec<bool>,
}

impl DisjointSets {
    fn new(size: usize) -> Self {
        Self {
            nodes: vec![1; size],
            is_root: vec![true; size],
        }
    }

    fn union(&mut self, i: usize, j: usize) {
        let i = self.get_root(i);
        let j = self.get_root(j);
        if i != j && self.nodes[i] >= self.nodes[j] {
            self.nodes[i] += self.nodes[j];
            self.nodes[j] = i;
            self.is_root[j] = false;
        } else if i != j {
            self.nodes[j] += self.nodes[i];
            self.nodes[i] = j;
            self.is_root[i] = false;
        }
    }

    fn get_root(&mut self, i: usize) -> usize {
        if self.is_root[i] {
            i
        } else {
            let res = self.get_root(self.nodes[i]);
            self.nodes[i] = res;
            self.nodes[i]
        }
    }
}

pub fn solve(s: String) -> (usize, usize) {
    let coords = s
        .lines()
        .map(|l| {
            let mut s = l.split(',').map(|c| c.parse::<usize>().unwrap());
            [s.next().unwrap(), s.next().unwrap(), s.next().unwrap()]
        })
        .collect::<Vec<_>>();
    let dists = (0..coords.len())
        .flat_map(|i| (0..i).map(move |j| (i, j)))
        .filter(|(i, j)| i != j)
        .map(|(i, j)| {
            (
                -coords[i]
                    .iter()
                    .zip(coords[j].iter())
                    .map(|(c1i, c2i)| c1i.abs_diff(*c2i).pow(2) as isize)
                    .sum::<isize>(),
                i,
                j,
            )
        })
        .collect::<BinaryHeap<_>>();
    (part1(dists.clone(), &coords), part2(dists, &coords))
}

fn part2(mut dists: BinaryHeap<(isize, usize, usize)>, coords: &Vec<[usize; 3]>) -> usize {
    let mut dsu = DisjointSets::new(coords.len());
    while let Some((_, c1, c2)) = dists.pop() {
        dsu.union(c1, c2);
        let maybe_root = dsu.get_root(0);
        if dsu.nodes[maybe_root] == coords.len() && dsu.is_root[maybe_root] {
            return coords[c1][0] * coords[c2][0];
        }
    }
    unreachable!()
}

fn part1(mut dists: BinaryHeap<(isize, usize, usize)>, coords: &Vec<[usize; 3]>) -> usize {
    let mut adj = vec![vec![]; dists.len()];
    for _ in 0..1000 {
        let (_, c1, c2) = dists.pop().unwrap();
        adj[c1].push(c2);
        adj[c2].push(c1);
    }
    let mut vis = vec![false; coords.len()];
    let mut sizes = vec![];
    for i in 0..coords.len() {
        if vis[i] {
            continue;
        }
        let mut size = 0;
        let mut stack = vec![i];
        while let Some(cur) = stack.pop() {
            if vis[cur] {
                continue;
            }
            stack.extend(adj[cur].iter().filter(|c| !vis[**c]));
            vis[cur] = true;
            size += 1;
        }
        sizes.push(size);
    }
    let mut largest: [usize; 3] = [0, 0, 0];
    for mut size in sizes {
        for large in largest.iter_mut() {
            if size > *large {
                std::mem::swap(large, &mut size);
            }
        }
    }
    largest.iter().fold(1, Mul::mul)
}
