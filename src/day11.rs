//! Day 11: Reactor
//!
//! Count paths in a DAG.
//!
//! Part 1: Count all paths from "you" to "out".
//! Part 2: Count paths from "svr" to "out" passing through both "dac" and "fft".

use std::collections::HashMap;

fn parse_input(input: &str) -> HashMap<String, Vec<String>> {
    let mut graph: HashMap<String, Vec<String>> = HashMap::new();

    for line in input.trim().lines() {
        let parts: Vec<&str> = line.split(": ").collect();
        let node = parts[0].to_string();
        let targets: Vec<String> =
            parts[1].split_whitespace().map(|s| s.to_string()).collect();
        graph.insert(node, targets);
    }

    graph
}

fn count_paths(
    graph: &HashMap<String, Vec<String>>,
    from: &str,
    to: &str,
    memo: &mut HashMap<String, u64>,
) -> u64 {
    if from == to {
        return 1;
    }

    if let Some(&cached) = memo.get(from) {
        return cached;
    }

    let count = if let Some(neighbors) = graph.get(from) {
        neighbors
            .iter()
            .map(|n| count_paths(graph, n, to, memo))
            .sum()
    } else {
        0
    };

    memo.insert(from.to_string(), count);
    count
}

pub fn part_one(input: &str) -> u64 {
    let graph = parse_input(input);
    let mut memo = HashMap::new();
    count_paths(&graph, "you", "out", &mut memo)
}

fn count_paths_via(
    graph: &HashMap<String, Vec<String>>,
    from: &str,
    via1: &str,
    via2: &str,
    to: &str,
    memo_via: &mut HashMap<(String, u8), u64>,
) -> u64 {
    // Count paths from `from` to `to` that visit both via1 and via2
    // State: (current node, bitmask of visited checkpoints)
    // 0 = none visited, 1 = via1 visited, 2 = via2 visited, 3 = both visited

    #[allow(clippy::too_many_arguments)]
    fn recurse(
        graph: &HashMap<String, Vec<String>>,
        node: &str,
        via1: &str,
        via2: &str,
        to: &str,
        visited_mask: u8,
        memo_via: &mut HashMap<(String, u8), u64>,
    ) -> u64 {
        // Update mask based on current node
        let mut mask = visited_mask;
        if node == via1 {
            mask |= 1;
        }
        if node == via2 {
            mask |= 2;
        }

        if node == to {
            return if mask == 3 { 1 } else { 0 };
        }

        let key = (node.to_string(), mask);
        if let Some(&cached) = memo_via.get(&key) {
            return cached;
        }

        let count = if let Some(neighbors) = graph.get(node) {
            neighbors
                .iter()
                .map(|n| recurse(graph, n, via1, via2, to, mask, memo_via))
                .sum()
        } else {
            0
        };

        memo_via.insert(key, count);
        count
    }

    recurse(graph, from, via1, via2, to, 0, memo_via)
}

pub fn part_two(input: &str) -> u64 {
    let graph = parse_input(input);
    let mut memo_via = HashMap::new();
    count_paths_via(&graph, "svr", "dac", "fft", "out", &mut memo_via)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::read_example;

    #[test]
    fn example() {
        let input = read_example(11);
        assert_eq!(part_one(&input), 5);
    }

    #[test]
    fn example_part2() {
        let input = crate::read_as_string(11, "example-2");
        assert_eq!(part_two(&input), 2);
    }
}
