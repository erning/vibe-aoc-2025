//! Day 5: Cafeteria
//!
//! ## Problem Description
//!
//! Part 1: Given fresh ingredient ID ranges and available ingredient IDs,
//! count how many available IDs are fresh (fall within any range).
//!
//! Part 2: Given fresh ingredient ID ranges, count the total number of
//! ingredient IDs considered fresh by the ranges (union of all ranges).
//!
//! ## Solution Approach
//!
//! **Input Parsing**: Parse two sections: fresh ranges and available IDs.
//!
//! **Part 1 Strategy**:
//! - For each available ID, check if it falls within any fresh range
//! - Count matches
//! - Time: O(m * n) where m = available IDs, n = ranges
//!
//! **Part 2 Strategy**:
//! - Merge overlapping ranges to get disjoint ranges
//! - Sum the sizes of all disjoint ranges
//! - Time: O(n log n) for merging, then O(n) for summing
//!
//! **Complexity**: Part 1 is O(m * n), Part 2 is O(n log n).

use std::cmp::max;

#[derive(Clone)]
struct Range {
    start: u64,
    end: u64,
}

fn parse_input(input: &str) -> (Vec<Range>, Vec<u64>) {
    let parts: Vec<&str> = input.trim().split("\n\n").collect();

    let ranges: Vec<Range> = parts[0]
        .lines()
        .map(|line| {
            let nums: Vec<u64> =
                line.split('-').map(|s| s.parse().unwrap()).collect();
            Range {
                start: nums[0],
                end: nums[1],
            }
        })
        .collect();

    let ids: Vec<u64> =
        parts[1].lines().map(|line| line.parse().unwrap()).collect();

    (ranges, ids)
}

fn is_fresh(id: u64, ranges: &[Range]) -> bool {
    ranges.iter().any(|r| id >= r.start && id <= r.end)
}

fn merge_ranges(mut ranges: Vec<Range>) -> Vec<Range> {
    if ranges.is_empty() {
        return Vec::new();
    }

    // Sort ranges by start position
    ranges.sort_by_key(|r| r.start);

    let mut merged = vec![ranges[0].clone()];

    for range in ranges.iter().skip(1) {
        let last = merged.last_mut().unwrap();

        // If current range overlaps with last merged range, merge them
        if range.start <= last.end + 1 {
            last.end = max(last.end, range.end);
        } else {
            // Otherwise, add as new range
            merged.push(range.clone());
        }
    }

    merged
}

pub fn part_one(input: &str) -> usize {
    let (ranges, ids) = parse_input(input);
    ids.iter().filter(|&&id| is_fresh(id, &ranges)).count()
}

pub fn part_two(input: &str) -> u64 {
    let (ranges, _) = parse_input(input);
    let merged = merge_ranges(ranges);

    merged.iter().map(|r| r.end - r.start + 1).sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::read_example;

    #[test]
    fn example_part_one() {
        let input = read_example(5);
        assert_eq!(part_one(&input), 3);
    }

    #[test]
    fn example_part_two() {
        let input = read_example(5);
        assert_eq!(part_two(&input), 14);
    }
}
