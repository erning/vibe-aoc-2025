//! Day 11: Reactor
//!
//! ## Problem Description
//!
//! Count paths through a device graph.
//! - Part 1: All paths from "you" to "out"
//! - Part 2: All paths from "svr" to "out" passing through "dac" and "fft"

use std::collections::HashMap;

fn parse_input(input: &str) -> HashMap<String, Vec<String>> {
    let mut graph = HashMap::new();

    for line in input.lines() {
        let parts: Vec<&str> = line.split(": ").collect();
        if parts.len() == 2 {
            let from = parts[0].to_string();
            let outputs =
                parts[1].split_whitespace().map(|s| s.to_string()).collect();
            graph.insert(from, outputs);
        }
    }

    graph
}

fn count_paths(
    graph: &HashMap<String, Vec<String>>,
    start: &str,
    end: &str,
    memo: &mut HashMap<String, u64>,
) -> u64 {
    if start == end {
        return 1;
    }

    if let Some(&count) = memo.get(start) {
        return count;
    }

    let mut total = 0u64;

    if let Some(neighbors) = graph.get(start) {
        for neighbor in neighbors {
            total += count_paths(graph, neighbor, end, memo);
        }
    }

    memo.insert(start.to_string(), total);
    total
}

fn count_paths_with_constraints(
    graph: &HashMap<String, Vec<String>>,
    current: &str,
    end: &str,
    must_visit: &[&str],
    visited: &mut Vec<String>,
    memo: &mut HashMap<(String, Vec<String>), u64>,
) -> u64 {
    if current == end {
        // Check if we've visited all required nodes
        return if must_visit
            .iter()
            .all(|req| visited.contains(&req.to_string()))
        {
            1
        } else {
            0
        };
    }

    let visited_sorted = {
        let mut v = visited.clone();
        v.sort();
        v
    };
    let key = (current.to_string(), visited_sorted);

    if let Some(&count) = memo.get(&key) {
        return count;
    }

    let mut total = 0u64;

    if let Some(neighbors) = graph.get(current) {
        for neighbor in neighbors {
            let mut new_visited = visited.clone();
            if !new_visited.contains(&neighbor.clone()) {
                new_visited.push(neighbor.clone());
            }
            total += count_paths_with_constraints(
                graph,
                neighbor,
                end,
                must_visit,
                &mut new_visited,
                memo,
            );
        }
    }

    memo.insert((current.to_string(), key.1), total);
    total
}

pub fn part_one(input: &str) -> u64 {
    let graph = parse_input(input);
    let mut memo = HashMap::new();
    count_paths(&graph, "you", "out", &mut memo)
}

pub fn part_two(input: &str) -> u64 {
    let graph = parse_input(input);
    let mut memo = HashMap::new();
    let mut visited = vec!["svr".to_string()];
    count_paths_with_constraints(
        &graph,
        "svr",
        "out",
        &["dac", "fft"],
        &mut visited,
        &mut memo,
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::read_example;

    #[test]
    fn example_part_one() {
        let input = read_example(11);
        assert_eq!(part_one(&input), 5);
    }
}
