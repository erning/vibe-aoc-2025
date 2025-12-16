//! Day 11: Reactor
//!
//! ## Problem Description
//!
//! Part 1: Count all paths from `you` to `out` in a directed graph.
//! Each device has outputs leading to other devices. We need to count
//! distinct paths through the graph.
//!
//! Part 2: Count paths from `svr` to `out` that pass through both `dac` AND `fft`.
//!
//! ## Solution Approach
//!
//! **Part 1**: Use memoized DFS to count paths. For each node, cache the
//! number of paths from that node to `out`.
//!
//! **Part 2**: Use state-based DP where state = (node, visited_dac, visited_fft).
//! Count paths that reach `out` with both flags set.

use std::collections::HashMap;

fn parse_input(input: &str) -> HashMap<String, Vec<String>> {
    let mut graph: HashMap<String, Vec<String>> = HashMap::new();

    for line in input.trim().lines() {
        let parts: Vec<&str> = line.split(':').collect();
        if parts.len() != 2 {
            continue;
        }
        let node = parts[0].trim().to_string();
        let outputs: Vec<String> =
            parts[1].split_whitespace().map(|s| s.to_string()).collect();
        graph.insert(node, outputs);
    }

    graph
}

/// Count paths from `start` to `out` using memoization
fn count_paths(
    graph: &HashMap<String, Vec<String>>,
    node: &str,
    memo: &mut HashMap<String, u64>,
) -> u64 {
    if node == "out" {
        return 1;
    }

    if let Some(&cached) = memo.get(node) {
        return cached;
    }

    let result = match graph.get(node) {
        Some(outputs) => outputs
            .iter()
            .map(|next| count_paths(graph, next, memo))
            .sum(),
        None => 0,
    };

    memo.insert(node.to_string(), result);
    result
}

/// Count paths from `start` to `out` that visit both `dac` and `fft`
/// State: (node, visited_dac, visited_fft)
fn count_paths_with_required(
    graph: &HashMap<String, Vec<String>>,
    node: &str,
    visited_dac: bool,
    visited_fft: bool,
    memo: &mut HashMap<(String, bool, bool), u64>,
) -> u64 {
    // Update visited flags
    let new_dac = visited_dac || node == "dac";
    let new_fft = visited_fft || node == "fft";

    if node == "out" {
        return if new_dac && new_fft { 1 } else { 0 };
    }

    let key = (node.to_string(), new_dac, new_fft);
    if let Some(&cached) = memo.get(&key) {
        return cached;
    }

    let result = match graph.get(node) {
        Some(outputs) => outputs
            .iter()
            .map(|next| {
                count_paths_with_required(graph, next, new_dac, new_fft, memo)
            })
            .sum(),
        None => 0,
    };

    memo.insert(key, result);
    result
}

pub fn part_one(input: &str) -> u64 {
    let graph = parse_input(input);
    let mut memo = HashMap::new();
    count_paths(&graph, "you", &mut memo)
}

pub fn part_two(input: &str) -> u64 {
    let graph = parse_input(input);
    let mut memo = HashMap::new();
    count_paths_with_required(&graph, "svr", false, false, &mut memo)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::read_example;

    #[test]
    fn example() {
        let input = read_example(11);
        assert_eq!(part_one(&input), 5);
        // Note: Part 2 example uses different input, so we don't test it here
    }
}
