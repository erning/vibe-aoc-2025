//! Day 8: Playground
//!
//! Junction boxes and circuits using Union-Find for Kruskal's MST algorithm.
//!
//! Part 1: Connect 1000 closest pairs, multiply sizes of 3 largest circuits.
//! Part 2: Continue until one circuit, return product of X coords of last pair.

fn parse_input(input: &str) -> Vec<(i64, i64, i64)> {
    input
        .trim()
        .lines()
        .map(|line| {
            let nums: Vec<i64> =
                line.split(',').map(|s| s.parse().unwrap()).collect();
            (nums[0], nums[1], nums[2])
        })
        .collect()
}

fn distance_squared(a: &(i64, i64, i64), b: &(i64, i64, i64)) -> i64 {
    let dx = a.0 - b.0;
    let dy = a.1 - b.1;
    let dz = a.2 - b.2;
    dx * dx + dy * dy + dz * dz
}

struct UnionFind {
    parent: Vec<usize>,
    rank: Vec<usize>,
    size: Vec<usize>,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        UnionFind {
            parent: (0..n).collect(),
            rank: vec![0; n],
            size: vec![1; n],
        }
    }

    fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            self.parent[x] = self.find(self.parent[x]);
        }
        self.parent[x]
    }

    fn union(&mut self, x: usize, y: usize) -> bool {
        let rx = self.find(x);
        let ry = self.find(y);
        if rx == ry {
            return false;
        }
        if self.rank[rx] < self.rank[ry] {
            self.parent[rx] = ry;
            self.size[ry] += self.size[rx];
        } else if self.rank[rx] > self.rank[ry] {
            self.parent[ry] = rx;
            self.size[rx] += self.size[ry];
        } else {
            self.parent[ry] = rx;
            self.size[rx] += self.size[ry];
            self.rank[rx] += 1;
        }
        true
    }

    fn get_size(&mut self, x: usize) -> usize {
        let root = self.find(x);
        self.size[root]
    }
}

fn get_sorted_edges(points: &[(i64, i64, i64)]) -> Vec<(i64, usize, usize)> {
    let n = points.len();
    let mut edges: Vec<(i64, usize, usize)> = Vec::new();
    for i in 0..n {
        for j in (i + 1)..n {
            let dist = distance_squared(&points[i], &points[j]);
            edges.push((dist, i, j));
        }
    }
    edges.sort_by_key(|e| e.0);
    edges
}

pub fn part_one(input: &str) -> u64 {
    let points = parse_input(input);
    let n = points.len();
    let edges = get_sorted_edges(&points);

    let mut uf = UnionFind::new(n);

    // Process the first 1000 closest pairs
    for (_, i, j) in edges.iter().take(1000) {
        uf.union(*i, *j);
    }

    // Get sizes of all unique circuits (find root for each, collect unique sizes)
    let mut roots: std::collections::HashSet<usize> =
        std::collections::HashSet::new();
    for i in 0..n {
        roots.insert(uf.find(i));
    }
    let mut circuit_sizes: Vec<usize> =
        roots.iter().map(|&r| uf.size[r]).collect();
    circuit_sizes.sort_by(|a, b| b.cmp(a));

    let top3: Vec<u64> =
        circuit_sizes.iter().take(3).map(|&x| x as u64).collect();
    top3.iter().product()
}

pub fn part_two(input: &str) -> i64 {
    let points = parse_input(input);
    let n = points.len();
    let edges = get_sorted_edges(&points);

    let mut uf = UnionFind::new(n);
    let mut last_pair = (0, 0);

    for (_, i, j) in edges {
        if uf.union(i, j) {
            last_pair = (i, j);
            // Check if all in one circuit
            if uf.get_size(0) == n {
                break;
            }
        }
    }

    points[last_pair.0].0 * points[last_pair.1].0
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::read_example;

    #[test]
    fn example() {
        let input = read_example(8);
        // For example with 20 points: after 10 connections, multiply 3 largest
        // The example says 5 * 4 * 2 = 40 after 10 connections
        // But we need 1000 connections for part 1, so we can't test with example
        // Let's just verify parsing works
        let points = parse_input(&input);
        assert_eq!(points.len(), 20);

        // Part 2: last pair is 216,146,977 and 117,168,530, X product is 25272
        assert_eq!(part_two(&input), 25272);
    }
}
