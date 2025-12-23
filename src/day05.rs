//! Day 5: Cafeteria
//!
//! ## Problem Description
//!
//! The database consists of:
//! - Fresh ingredient ID ranges (inclusive, can overlap)
//! - A blank line
//! - Available ingredient IDs to check
//!
//! **Part 1:** Count how many available IDs fall into at least one fresh range.
//!
//! **Part 2:** Count total unique IDs covered by all fresh ranges (merge overlapping ranges).

use std::ops::RangeInclusive;

/// Parse input into (fresh_ranges, available_ids)
fn parse_input(input: &str) -> (Vec<RangeInclusive<u64>>, Vec<u64>) {
    let parts: Vec<&str> = input.trim().split("\n\n").collect();
    let ranges_str = parts[0];
    let ids_str = parts[1];

    let fresh_ranges: Vec<RangeInclusive<u64>> = ranges_str
        .lines()
        .map(|line| {
            let nums: Vec<u64> =
                line.split('-').map(|n| n.parse().unwrap()).collect();
            nums[0]..=nums[1]
        })
        .collect();

    let available_ids: Vec<u64> =
        ids_str.lines().map(|l| l.parse().unwrap()).collect();

    (fresh_ranges, available_ids)
}

/// Check if an ID is covered by any range
fn is_fresh(id: u64, ranges: &[RangeInclusive<u64>]) -> bool {
    ranges.iter().any(|r| r.contains(&id))
}

/// Merge overlapping or adjacent ranges
fn merge_ranges(
    ranges: &mut [RangeInclusive<u64>],
) -> Vec<RangeInclusive<u64>> {
    if ranges.is_empty() {
        return vec![];
    }

    // Sort by start
    ranges.sort_by_key(|r| *r.start());

    let mut merged = Vec::new();
    let mut current = ranges[0].clone();

    for range in ranges.iter().skip(1) {
        if *range.start() <= *current.end() + 1 {
            // Overlap or adjacent - merge
            current = *current.start()..=(*current.end().max(range.end()));
        } else {
            // No overlap - push current and start new
            merged.push(current);
            current = range.clone();
        }
    }
    merged.push(current);

    merged
}

pub fn part_one(input: &str) -> usize {
    let (ranges, ids) = parse_input(input);
    ids.iter().filter(|&&id| is_fresh(id, &ranges)).count()
}

pub fn part_two(input: &str) -> u64 {
    let (mut ranges, _) = parse_input(input);
    let merged = merge_ranges(&mut ranges);
    merged.iter().map(|r| r.end() - r.start() + 1).sum()
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
