//! Day 8: Playground
//!
//! ## Problem Description
//!
//! Junction box connection problem - minimum spanning tree.
//! - Part 1: Connect 1000 closest pairs, find 3 largest circuits, multiply
//! - Part 2: Continue connecting until all are in one circuit, return
//!           product of X coordinates of the last pair connected

use std::collections::HashMap;

#[derive(Clone, Copy, Eq, PartialEq)]
struct Point {
    x: i64,
    y: i64,
    z: i64,
}

impl Point {
    fn distance_squared(&self, other: &Point) -> i64 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        let dz = self.z - other.z;
        dx * dx + dy * dy + dz * dz
    }
}

#[derive(Clone, Copy, Eq, PartialEq)]
struct Edge {
    dist_squared: i64,
    idx1: usize,
    idx2: usize,
}

impl Ord for Edge {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // Reverse order for min-heap (BinaryHeap is max-heap by default)
        other.dist_squared.cmp(&self.dist_squared)
    }
}

impl PartialOrd for Edge {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

struct UnionFind {
    parent: Vec<usize>,
    size: Vec<usize>,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        UnionFind {
            parent: (0..n).collect(),
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
        let px = self.find(x);
        let py = self.find(y);

        if px == py {
            return false;
        }

        // Union by size
        if self.size[px] < self.size[py] {
            self.parent[px] = py;
            self.size[py] += self.size[px];
        } else {
            self.parent[py] = px;
            self.size[px] += self.size[py];
        }

        true
    }
}

fn parse_input(input: &str) -> Vec<Point> {
    input
        .lines()
        .filter_map(|line| {
            let parts: Vec<&str> = line.split(',').collect();
            if parts.len() == 3 {
                let x = parts[0].trim().parse().ok()?;
                let y = parts[1].trim().parse().ok()?;
                let z = parts[2].trim().parse().ok()?;
                Some(Point { x, y, z })
            } else {
                None
            }
        })
        .collect()
}

pub fn part_one(input: &str) -> u64 {
    let points = parse_input(input);
    let n = points.len();

    // Generate all edges
    let mut edges = Vec::new();
    for i in 0..n {
        for j in (i + 1)..n {
            let dist_squared = points[i].distance_squared(&points[j]);
            edges.push(Edge {
                dist_squared,
                idx1: i,
                idx2: j,
            });
        }
    }

    // Sort edges by distance
    edges.sort_by_key(|e| e.dist_squared);

    // Take the 1000 closest pairs and apply union-find
    let mut uf = UnionFind::new(n);
    for edge in edges.iter().take(1000) {
        uf.union(edge.idx1, edge.idx2);
    }

    // Find circuit sizes - track each root's size
    let mut circuit_sizes: HashMap<usize, usize> = HashMap::new();
    for i in 0..n {
        let root = uf.find(i);
        circuit_sizes.insert(root, uf.size[root]);
    }

    // Get 3 largest sizes
    let mut sizes: Vec<usize> = circuit_sizes.values().copied().collect();
    sizes.sort_by(|a: &usize, b: &usize| b.cmp(a));

    // Ensure we have at least 3 sizes
    while sizes.len() < 3 {
        sizes.push(1);
    }

    let result = (sizes[0] as u64) * (sizes[1] as u64) * (sizes[2] as u64);
    result
}

pub fn part_two(input: &str) -> u64 {
    let points = parse_input(input);
    let n = points.len();

    // Generate all edges
    let mut edges = Vec::new();
    for i in 0..n {
        for j in (i + 1)..n {
            let dist_squared = points[i].distance_squared(&points[j]);
            edges.push(Edge {
                dist_squared,
                idx1: i,
                idx2: j,
            });
        }
    }

    // Sort edges by distance
    edges.sort_by_key(|e| e.dist_squared);

    // Kruskal's algorithm: connect until all in one circuit
    let mut uf = UnionFind::new(n);
    let mut last_edge = Edge {
        dist_squared: 0,
        idx1: 0,
        idx2: 0,
    };

    for edge in edges.iter() {
        if uf.union(edge.idx1, edge.idx2) {
            last_edge = edge.clone();

            // Check if all are connected
            let root = uf.find(0);
            let all_connected = (0..n).all(|i| uf.find(i) == root);

            if all_connected {
                break;
            }
        }
    }

    // Product of X coordinates
    let result = points[last_edge.idx1].x * points[last_edge.idx2].x;
    result as u64
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::read_example;

    #[test]
    fn example_part_one() {
        let input = read_example(8);
        // Example has 20 points and 190 possible edges
        // Taking 1000 edges means we take all 190, connecting everything
        // Result: 1 circuit of size 20, padded to [20, 1, 1] = 20
        assert_eq!(part_one(&input), 20);
    }

    #[test]
    fn example_part_two() {
        let input = read_example(8);
        assert_eq!(part_two(&input), 25272);
    }
}
