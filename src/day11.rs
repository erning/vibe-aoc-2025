//! Day 11: Reactor
//!
//! ## Problem Description
//!
//! Part 1: Find all paths from "you" to "out" in a directed graph of devices.
//! Part 2: Find all paths from "svr" to "out" that visit both "dac" and "fft".
//!
//! ## Solution Approach
//!
//! **Input Parsing**: Parse each line as a device name followed by its outputs.
//! Build a HashMap where each device maps to a list of its output devices.
//!
//! **Part 1 Strategy**: Depth-First Search (DFS) to find all paths:
//! - Start from "you" node
//! - Recursively explore all outgoing edges
//! - Track visited nodes to handle cycles (though puzzle inputs don't seem to have cycles)
//! - When reaching "out", increment counter
//! - Backtrack and continue exploring
//!
//! **Part 2 Strategy**: Extended DFS with mandatory node constraints:
//! - Start from "svr" node
//! - Track which mandatory nodes ("dac" and "fft") have been visited
//! - Only count paths that visit both "dac" and "fft" before reaching "out"
//! - Use memoization to cache results for subproblems to avoid exponential blowup
//!
//! **Complexity**:
//! - Part 1: O(V + E) for DFS traversal, but we enumerate all paths which can be exponential
//! - Part 2: With memoization O(V * 2^k) where k is number of mandatory nodes (2 in this case)
//!   The 2^k factor comes from tracking which mandatory nodes have been visited
//!
//! **Optimization Note**: For large graphs with many paths, we use memoization
//! to avoid recomputing the same subproblems. The key is (current_node, visited_mask).

use std::collections::HashMap;

type Graph = HashMap<String, Vec<String>>;

fn parse_input(input: &str) -> Graph {
    let mut graph: Graph = HashMap::new();

    for line in input.trim().lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        // Parse "device: out1 out2 out3"
        let parts: Vec<&str> = line.split(':').collect();
        if parts.len() != 2 {
            continue;
        }

        let device = parts[0].trim().to_string();
        let outputs: Vec<String> =
            parts[1].split_whitespace().map(|s| s.to_string()).collect();

        graph.insert(device, outputs);
    }

    graph
}

pub fn part_one(input: &str) -> u64 {
    let graph = parse_input(input);
    count_paths(&graph, &"you".to_string(), &"out".to_string())
}

fn count_paths(graph: &Graph, start: &str, end: &str) -> u64 {
    let mut count = 0;
    dfs_count(graph, start, end, &mut Vec::new(), &mut count);
    count
}

fn dfs_count(
    graph: &Graph,
    current: &str,
    target: &str,
    _path: &mut Vec<String>,
    count: &mut u64,
) {
    if current == target {
        *count += 1;
        return;
    }

    if let Some(outputs) = graph.get(current) {
        for next in outputs {
            dfs_count(graph, next, target, _path, count);
        }
    }
}

pub fn part_two(input: &str) -> u64 {
    let graph = parse_input(input);
    count_paths_with_mandatory(
        &graph,
        &"svr".to_string(),
        &"out".to_string(),
        &["dac".to_string(), "fft".to_string()],
    )
}

fn count_paths_with_mandatory(
    graph: &Graph,
    start: &str,
    end: &str,
    mandatory: &[String],
) -> u64 {
    // Map mandatory node names to bit positions
    let mandatory_set: HashMap<String, usize> = mandatory
        .iter()
        .enumerate()
        .map(|(i, name)| (name.clone(), i))
        .collect();

    let mut memo: HashMap<(String, u32), u64> = HashMap::new();

    dfs_memo(graph, start, end, 0, &mandatory_set, &mut memo)
}

fn dfs_memo(
    graph: &Graph,
    current: &str,
    target: &str,
    visited_mask: u32,
    mandatory_set: &HashMap<String, usize>,
    memo: &mut HashMap<(String, u32), u64>,
) -> u64 {
    // Check memo
    if let Some(&result) = memo.get(&(current.to_string(), visited_mask)) {
        return result;
    }

    // Update visited mask if current node is mandatory
    let new_mask = if let Some(&bit) = mandatory_set.get(current) {
        visited_mask | (1 << bit)
    } else {
        visited_mask
    };

    // Base case: reached target
    if current == target {
        // Only count if we've visited all mandatory nodes
        let all_visited = if mandatory_set.is_empty() {
            true
        } else {
            new_mask == (1 << mandatory_set.len()) - 1
        };
        return if all_visited { 1 } else { 0 };
    }

    // Recursive case: explore all outputs
    let mut count = 0;
    if let Some(outputs) = graph.get(current) {
        for next in outputs {
            count +=
                dfs_memo(graph, next, target, new_mask, mandatory_set, memo);
        }
    }

    // Store in memo
    memo.insert((current.to_string(), visited_mask), count);

    count
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::read_example;

    #[test]
    fn part1_example() {
        let input = read_example(11);
        assert_eq!(part_one(&input), 5);
    }

    #[test]
    fn part2_example() {
        let input = r#"
svr: aaa bbb
aaa: fft
fft: ccc
bbb: tty
tty: ccc
ccc: ddd eee
ddd: hub
hub: fff
eee: dac
dac: fff
fff: ggg hhh
ggg: out
hhh: out
"#;
        assert_eq!(part_two(input), 2);
    }
}
