use std::collections::HashMap;

#[derive(Clone, Copy, Debug)]
struct Point3D {
    x: i64,
    y: i64,
    z: i64,
}

impl Point3D {
    fn distance(&self, other: &Point3D) -> f64 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        let dz = self.z - other.z;
        ((dx * dx + dy * dy + dz * dz) as f64).sqrt()
    }
}

fn parse_input(input: &str) -> Vec<Point3D> {
    input
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| {
            let coords: Vec<i64> = line
                .split(',')
                .map(|s| s.trim().parse().unwrap())
                .collect();
            Point3D {
                x: coords[0],
                y: coords[1],
                z: coords[2],
            }
        })
        .collect()
}

#[derive(Debug)]
struct UnionFind {
    parent: Vec<usize>,
    size: Vec<usize>,
    components: usize,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        UnionFind {
            parent: (0..n).collect(),
            size: vec![1; n],
            components: n,
        }
    }

    fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            let root = self.find(self.parent[x]);
            self.parent[x] = root;
        }
        self.parent[x]
    }

    fn union(&mut self, a: usize, b: usize) -> bool {
        let mut root_a = self.find(a);
        let mut root_b = self.find(b);
        
        if root_a == root_b {
            return false;
        }

        if self.size[root_a] < self.size[root_b] {
            std::mem::swap(&mut root_a, &mut root_b);
        }

        self.parent[root_b] = root_a;
        self.size[root_a] += self.size[root_b];
        self.components -= 1;
        true
    }

    fn all_component_sizes(&mut self) -> Vec<usize> {
        let mut sizes = HashMap::new();
        for i in 0..self.parent.len() {
            let root = self.find(i);
            *sizes.entry(root).or_insert(0) += 1;
        }
        sizes.into_values().collect()
    }
}

fn calculate_distances(points: &[Point3D]) -> Vec<(f64, usize, usize)> {
    let mut distances = Vec::new();
    
    for i in 0..points.len() {
        for j in (i + 1)..points.len() {
            let dist = points[i].distance(&points[j]);
            distances.push((dist, i, j));
        }
    }
    
    distances.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
    distances
}

fn test_with_connections(input: &str, num_connections: usize) -> i64 {
    let points = parse_input(input);
    let distances = calculate_distances(&points);
    
    let mut uf = UnionFind::new(points.len());
    
    for (dist, i, j) in distances.iter().take(num_connections) {
        uf.union(*i, *j);
    }
    
    let sizes = uf.all_component_sizes();
    let mut sorted_sizes = sizes;
    sorted_sizes.sort_by(|a, b| b.cmp(a));
    
    if sorted_sizes.len() >= 3 {
        (sorted_sizes[0] * sorted_sizes[1] * sorted_sizes[2]) as i64
    } else {
        0
    }
}

fn main() {
    let input = "162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689";

    for num in 1..=20 {
        let result = test_with_connections(input, num);
        if result != 0 {
            println!("Connections: {}, Result: {}", num, result);
        }
    }
}
