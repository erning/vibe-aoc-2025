//! Day 8: Playground
//!
//! Connect junction boxes in 3D space by shortest Euclidean distance.
//!
//! ## Solution Approach
//!
//! This is a Minimum Spanning Tree (MST) problem solved with Kruskal's
//! algorithm using Union-Find for efficient component tracking.
//!
//! **Input Parsing**: Parse 3D coordinates (x, y, z) for each junction box.
//!
//! **Part 1 Strategy**: Process the 1000 closest pairs of boxes.
//! - Generate all pairs with squared Euclidean distances (avoid floats)
//! - Sort pairs by distance
//! - Use Union-Find to track connected components
//! - Process 1000 edges (some may connect already-connected nodes)
//! - Return product of 3 largest component sizes
//! - Complexity: O(n^2 log n) for pair generation and sorting
//!
//! **Part 2 Strategy**: Continue connecting until all boxes form one circuit.
//! - Continue Kruskal's algorithm until single component
//! - Track the last edge that completes the MST
//! - Return product of X coordinates of the last two boxes connected
//! - Complexity: O(n^2 log n) dominated by sorting

/// Union-Find (Disjoint Set Union) with path compression and union by rank
struct UnionFind {
    parent: Vec<usize>,
    rank: Vec<usize>,
    size: Vec<usize>,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        Self {
            parent: (0..n).collect(),
            rank: vec![0; n],
            size: vec![1; n],
        }
    }

    fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            self.parent[x] = self.find(self.parent[x]); // path compression
        }
        self.parent[x]
    }

    fn union(&mut self, x: usize, y: usize) -> bool {
        let px = self.find(x);
        let py = self.find(y);
        if px == py {
            return false; // already in same component
        }
        // union by rank
        match self.rank[px].cmp(&self.rank[py]) {
            std::cmp::Ordering::Less => {
                self.parent[px] = py;
                self.size[py] += self.size[px];
            }
            std::cmp::Ordering::Greater => {
                self.parent[py] = px;
                self.size[px] += self.size[py];
            }
            std::cmp::Ordering::Equal => {
                self.parent[py] = px;
                self.size[px] += self.size[py];
                self.rank[px] += 1;
            }
        }
        true
    }
}

/// Parse input into 3D coordinates
fn parse_input(input: &str) -> Vec<(i64, i64, i64)> {
    input
        .lines()
        .filter(|l| !l.is_empty())
        .map(|line| {
            let parts: Vec<i64> =
                line.split(',').map(|s| s.parse().unwrap()).collect();
            (parts[0], parts[1], parts[2])
        })
        .collect()
}

/// Calculate squared Euclidean distance (avoids floating point)
fn squared_distance(a: &(i64, i64, i64), b: &(i64, i64, i64)) -> i64 {
    let dx = a.0 - b.0;
    let dy = a.1 - b.1;
    let dz = a.2 - b.2;
    dx * dx + dy * dy + dz * dz
}

/// Generate all pairs sorted by distance
fn generate_sorted_edges(
    points: &[(i64, i64, i64)],
) -> Vec<(i64, usize, usize)> {
    let n = points.len();
    let mut edges = Vec::with_capacity(n * (n - 1) / 2);

    for i in 0..n {
        for j in (i + 1)..n {
            let dist = squared_distance(&points[i], &points[j]);
            edges.push((dist, i, j));
        }
    }

    edges.sort_unstable();
    edges
}

/// Part 1: After connecting 1000 closest pairs, return product of 3 largest
/// component sizes
pub fn part_one(input: &str) -> u64 {
    let points = parse_input(input);
    let n = points.len();
    let edges = generate_sorted_edges(&points);

    let mut uf = UnionFind::new(n);

    // For example with 20 points, we process 10 edges
    // For real input with 1000 points, we process 1000 edges
    let target_edges = if n <= 20 { 10 } else { 1000 };

    // Process the first target_edges edges (attempt connections)
    // Some edges may connect already-connected nodes, which is expected
    for (_dist, i, j) in edges.iter().take(target_edges) {
        uf.union(*i, *j);
    }

    // Get all unique component sizes
    let mut sizes: Vec<usize> = (0..n)
        .map(|i| uf.find(i))
        .collect::<std::collections::HashSet<_>>()
        .into_iter()
        .map(|root| uf.size[root])
        .collect();

    sizes.sort_unstable_by(|a, b| b.cmp(a)); // descending

    // Product of 3 largest
    sizes.iter().take(3).map(|&s| s as u64).product()
}

/// Part 2: Continue connecting until one circuit, return product of X coords
/// of the last two boxes connected
pub fn part_two(input: &str) -> u64 {
    let points = parse_input(input);
    let n = points.len();
    let edges = generate_sorted_edges(&points);

    let mut uf = UnionFind::new(n);
    let mut last_edge = (0, 0);

    for (_, i, j) in edges {
        if uf.union(i, j) {
            last_edge = (i, j);
            // Check if all connected (n-1 unions needed for MST)
            let root = uf.find(0);
            if uf.size[root] == n {
                break;
            }
        }
    }

    let x1 = points[last_edge.0].0 as u64;
    let x2 = points[last_edge.1].0 as u64;
    x1 * x2
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::read_example;

    #[test]
    fn example_part1() {
        let input = read_example(8);
        assert_eq!(part_one(&input), 40);
    }

    #[test]
    fn example_part2() {
        let input = read_example(8);
        assert_eq!(part_two(&input), 25272);
    }
}
