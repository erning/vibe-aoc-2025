//! Day 8: Playground
//!
//! Part 1: Connect 1000 closest pairs of junction boxes and multiply the sizes
//!         of the three largest circuits.
//! Part 2: Continue connecting until all junction boxes are in one circuit,
//!         then multiply the X coordinates of the last two connected boxes.
//!
//! ## Solution Approach
//!
//! **Input Format**: 3D coordinates (X,Y,Z) one per line, representing junction
//! box positions.
//!
//! **Algorithm**: Kruskal's minimum spanning tree algorithm with Union-Find.
//!
//! **Part 1 Strategy**:
//! - Parse all 3D coordinates
//! - Generate all pairwise distances (n*(n-1)/2 edges)
//! - Sort edges by distance
//! - Use Union-Find to track connected components (circuits)
//! - Process closest pairs (skip if already connected, but still count them)
//! - For the puzzle: make 1000 connection attempts (pairs to try connecting)
//! - Example has 20 boxes and makes 10 connection attempts
//! - Count component sizes and multiply three largest
//!
//! **Part 2 Strategy**:
//! - Continue processing remaining edges until all boxes in one circuit
//! - Track the last edge that successfully connects two circuits
//! - Return the product of X coordinates of that edge's endpoints
//!
//! **Complexity**:
//! - Part 1: O(n^2 * log(n^2)) for sorting all pairwise distances
//! - Part 2: O(n^2 * alpha(n)) for continued processing
//! - Space: O(n^2) for storing all edges
//!
//! With n=1000, n^2 = 1,000,000 edges which is manageable.

use std::collections::HashMap;

/// 3D point representing a junction box position
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: u64,
    y: u64,
    z: u64,
}

/// Edge representing a possible connection between two junction boxes
#[derive(Debug, Clone, Copy)]
struct Edge {
    from: usize,
    to: usize,
    dist_sq: u64,
}

/// Union-Find (Disjoint Set Union) data structure
struct UnionFind {
    parent: Vec<usize>,
    rank: Vec<usize>,
    size: Vec<usize>,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        let parent: Vec<usize> = (0..n).collect();
        let rank = vec![0; n];
        let size = vec![1; n];
        UnionFind { parent, rank, size }
    }

    fn find(&mut self, mut x: usize) -> usize {
        let mut root = x;
        // Find the root
        while self.parent[root] != root {
            root = self.parent[root];
        }
        // Path compression
        while self.parent[x] != x {
            let next = self.parent[x];
            self.parent[x] = root;
            x = next;
        }
        root
    }

    /// Returns true if union was performed (different components),
    /// false if already in same component
    fn union(&mut self, x: usize, y: usize) -> bool {
        let x_root = self.find(x);
        let y_root = self.find(y);

        if x_root == y_root {
            return false;
        }

        // Union by rank
        if self.rank[x_root] < self.rank[y_root] {
            self.parent[x_root] = y_root;
            self.size[y_root] += self.size[x_root];
        } else if self.rank[x_root] > self.rank[y_root] {
            self.parent[y_root] = x_root;
            self.size[x_root] += self.size[y_root];
        } else {
            self.parent[y_root] = x_root;
            self.rank[x_root] += 1;
            self.size[x_root] += self.size[y_root];
        }
        true
    }

    /// Count the number of connected components
    fn component_count(&mut self) -> usize {
        let mut roots = std::collections::HashSet::new();
        for i in 0..self.parent.len() {
            roots.insert(self.find(i));
        }
        roots.len()
    }

    /// Get the size of each component
    fn component_sizes(&mut self) -> Vec<usize> {
        let mut sizes: HashMap<usize, usize> = HashMap::new();
        for i in 0..self.parent.len() {
            let root = self.find(i);
            *sizes.entry(root).or_insert(0) += 1;
        }
        let mut result: Vec<usize> = sizes.values().copied().collect();
        result.sort_by(|a, b| b.cmp(a)); // Descending order
        result
    }
}

/// Parse input into a list of 3D points
fn parse_input(input: &str) -> Vec<Point> {
    input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            let parts: Vec<&str> = line.trim().split(',').collect();
            let x: u64 = parts[0].parse().unwrap();
            let y: u64 = parts[1].parse().unwrap();
            let z: u64 = parts[2].parse().unwrap();
            Point { x, y, z }
        })
        .collect()
}

/// Compute squared Euclidean distance between two 3D points
fn distance_squared(p1: Point, p2: Point) -> u64 {
    let dx = p1.x as i64 - p2.x as i64;
    let dy = p1.y as i64 - p2.y as i64;
    let dz = p1.z as i64 - p2.z as i64;
    (dx * dx + dy * dy + dz * dz) as u64
}

/// Process connections to find circuit information
fn process_circuits(
    points: &[Point],
    target_attempts: usize,
    find_last_edge: bool,
) -> (Vec<usize>, Option<(u64, u64)>) {
    let n = points.len();

    // Generate all pairwise edges
    let mut edges: Vec<Edge> = Vec::with_capacity(n * n / 2);
    for i in 0..n {
        for j in (i + 1)..n {
            let dist_sq = distance_squared(points[i], points[j]);
            edges.push(Edge {
                from: i,
                to: j,
                dist_sq,
            });
        }
    }

    // Sort by distance (ascending)
    edges.sort_by_key(|e| e.dist_sq);

    let mut uf = UnionFind::new(n);
    let mut attempts = 0;
    let mut last_edge_x_product: Option<(u64, u64)> = None;

    for edge in &edges {
        attempts += 1;

        // Try to connect - even if already connected, we count the attempt
        if uf.union(edge.from, edge.to) {
            // Check if this is the connection that makes everything one circuit
            let components = uf.component_count();
            if components == 1 {
                if find_last_edge {
                    last_edge_x_product =
                        Some((points[edge.from].x, points[edge.to].x));
                }
                break;
            }
        }

        // Count attempts (pairs we try to connect), not successful connections
        if attempts >= target_attempts {
            let sizes = uf.component_sizes();
            return (sizes, last_edge_x_product);
        }
    }

    let sizes = uf.component_sizes();
    (sizes, last_edge_x_product)
}

/// Part 1: Multiply sizes of three largest circuits after making connection attempts
pub fn part_one(input: &str) -> u64 {
    let points = parse_input(input);
    let n = points.len();

    // The puzzle specifies 1000 connection attempts for real input (1000 boxes)
    // Example has 20 boxes and uses 10 connection attempts
    let target_attempts = if n >= 1000 { 1000 } else { 10 };

    let (sizes, _) = process_circuits(&points, target_attempts, false);

    // Multiply three largest circuit sizes
    sizes.iter().take(3).product::<usize>() as u64
}

/// Part 2: Find the X coordinates of the last two boxes that get connected
pub fn part_two(input: &str) -> u64 {
    let points = parse_input(input);
    let (_, last_edge) = process_circuits(&points, usize::MAX, true);

    if let Some((x1, x2)) = last_edge {
        x1 * x2
    } else {
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::read_example;

    #[test]
    fn test_part_one() {
        let input = read_example(8);
        assert_eq!(part_one(&input), 40);
    }

    #[test]
    fn test_part_two() {
        let input = read_example(8);
        assert_eq!(part_two(&input), 25272);
    }
}
