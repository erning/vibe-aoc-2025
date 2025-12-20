//! Day 8: Playground
//!
//! ## Problem Description
//!
//! Part 1: Connect the 1000 closest pairs of junction boxes in 3D space.
//! Calculate the product of the three largest circuit sizes.
//!
//! Part 2: Continue connecting junction boxes until all are in one circuit.
//! Find the last connection that unifies all boxes and return the product
//! of the X coordinates of those two junction boxes.
//!
//! ## Solution Approach
//!
//! **Part 1**: Use Kruskal's algorithm with Union-Find to build circuits:
//! 1. Parse all junction box positions (X,Y,Z coordinates)
//! 2. Calculate distances between all pairs of boxes
//! 3. Sort edges by distance
//! 4. Connect the 1000 shortest edges using Union-Find
//! 5. Calculate circuit sizes and multiply the three largest
//!
//! **Part 2**: Continue the MST algorithm until all boxes form one circuit:
//! 1. Continue from where Part 1 left off
//! 2. Keep connecting until union-find has only one component
//! 3. Return the product of X coordinates of the final connecting edge
//!
//! **Complexity**: O(n^2 log n) where n is the number of junction boxes.
//! We need to calculate all pairwise distances and sort them.

#[derive(Debug, Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
    z: i32,
}

impl Point {
    fn distance_squared(&self, other: &Point) -> i64 {
        let dx = (self.x - other.x) as i64;
        let dy = (self.y - other.y) as i64;
        let dz = (self.z - other.z) as i64;
        dx * dx + dy * dy + dz * dz
    }
}

#[derive(Debug)]
struct Edge {
    from: usize,
    to: usize,
    dist_sq: i64,
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
        let root_x = self.find(x);
        let root_y = self.find(y);

        if root_x == root_y {
            return false; // Already in same set
        }

        // Union by size
        if self.size[root_x] < self.size[root_y] {
            self.parent[root_x] = root_y;
            self.size[root_y] += self.size[root_x];
        } else {
            self.parent[root_y] = root_x;
            self.size[root_x] += self.size[root_y];
        }

        true
    }

    fn get_component_sizes(&mut self) -> Vec<usize> {
        let mut sizes = std::collections::HashMap::new();
        for i in 0..self.parent.len() {
            let root = self.find(i);
            *sizes.entry(root).or_insert(0) += 1;
        }
        sizes.values().copied().collect()
    }

    fn component_count(&mut self) -> usize {
        let mut roots = std::collections::HashSet::new();
        for i in 0..self.parent.len() {
            roots.insert(self.find(i));
        }
        roots.len()
    }
}

fn parse_input(input: &str) -> Vec<Point> {
    input
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| {
            let parts: Vec<i32> =
                line.split(',').map(|s| s.trim().parse().unwrap()).collect();
            Point {
                x: parts[0],
                y: parts[1],
                z: parts[2],
            }
        })
        .collect()
}

pub fn part_one(input: &str) -> i64 {
    let points = parse_input(input);
    let n = points.len();

    // Build all edges
    let mut edges = Vec::new();
    for i in 0..n {
        for j in (i + 1)..n {
            edges.push(Edge {
                from: i,
                to: j,
                dist_sq: points[i].distance_squared(&points[j]),
            });
        }
    }

    // Sort by distance
    edges.sort_by_key(|e| e.dist_sq);

    // Determine how many connections to attempt based on input size
    // Example has 20 boxes -> attempt 10 connections
    // Real input has many more -> attempt 1000 connections
    let target_attempts = if n <= 20 { 10 } else { 1000 };

    // Connect the shortest edges (attempt connections in order)
    let mut uf = UnionFind::new(n);

    for (idx, edge) in edges.iter().enumerate() {
        if idx >= target_attempts {
            break;
        }
        uf.union(edge.from, edge.to);
    }

    // Get circuit sizes and multiply the three largest
    let mut sizes = uf.get_component_sizes();
    sizes.sort_by(|a, b| b.cmp(a)); // Sort descending

    sizes[0] as i64 * sizes[1] as i64 * sizes[2] as i64
}

pub fn part_two(input: &str) -> i64 {
    let points = parse_input(input);
    let n = points.len();

    // Build all edges
    let mut edges = Vec::new();
    for i in 0..n {
        for j in (i + 1)..n {
            edges.push(Edge {
                from: i,
                to: j,
                dist_sq: points[i].distance_squared(&points[j]),
            });
        }
    }

    // Sort by distance
    edges.sort_by_key(|e| e.dist_sq);

    // Connect until all boxes are in one circuit
    let mut uf = UnionFind::new(n);

    for edge in &edges {
        if uf.union(edge.from, edge.to) {
            // Check if we now have only one component
            if uf.component_count() == 1 {
                // This is the last connection
                return points[edge.from].x as i64 * points[edge.to].x as i64;
            }
        }
    }

    0 // Should never reach here
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::read_example;

    #[test]
    fn example() {
        let input = read_example(8);
        assert_eq!(part_one(&input), 40);
        assert_eq!(part_two(&input), 25272);
    }
}
