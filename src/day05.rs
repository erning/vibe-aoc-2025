//! Day 5: Cafeteria
//!
//! ## Puzzle Overview
//!
//! The Elves have a new inventory management system that uses ingredient ID ranges
//! to determine which ingredients are fresh. We need to process ranges and available
//! IDs to determine freshness.
//!
//! ## Part 1 Strategy
//!
//! Count how many available ingredient IDs fall into any of the fresh ranges.
//! An ingredient ID is fresh if it appears in any of the inclusive ranges.
//!
//! **Algorithm**:
//! 1. Parse input into two sections: ranges and available IDs (separated by blank line)
//! 2. Convert ranges to (start, end) tuples
//! 3. For each available ID, check if it falls into any range
//! 4. Count IDs that fall into at least one range
//!
//! **Complexity**: O(n*m) where n is number of available IDs and m is number of ranges
//!
//! ## Part 2 Strategy
//!
//! Count the total number of unique ingredient IDs covered by all fresh ranges,
//! considering overlapping ranges only once.
//!
//! **Algorithm**:
//! 1. Parse ranges from input
//! 2. Sort ranges by start position
//! 3. Merge overlapping ranges
//! 4. Sum the lengths of merged ranges
//!
//! **Complexity**: O(n log n) for sorting, O(n) for merging

fn parse_input(input: &str) -> (Vec<(u64, u64)>, Vec<u64>) {
    let sections: Vec<&str> = input.trim().split("\n\n").collect();
    let ranges_section = sections[0];
    let available_section = sections[1];

    let ranges: Vec<(u64, u64)> = ranges_section
        .lines()
        .map(|line| {
            let parts: Vec<&str> = line.split('-').collect();
            (parts[0].parse().unwrap(), parts[1].parse().unwrap())
        })
        .collect();

    let available_ids: Vec<u64> = available_section
        .lines()
        .map(|line| line.parse().unwrap())
        .collect();

    (ranges, available_ids)
}

/// Check if an ID falls into any of the ranges
fn is_fresh(id: u64, ranges: &[(u64, u64)]) -> bool {
    for &(start, end) in ranges {
        if id >= start && id <= end {
            return true;
        }
    }
    false
}

pub fn part_one(input: &str) -> u64 {
    let (ranges, available_ids) = parse_input(input);
    let mut fresh_count = 0;

    for &id in &available_ids {
        if is_fresh(id, &ranges) {
            fresh_count += 1;
        }
    }

    fresh_count
}

/// Merge overlapping ranges
fn merge_ranges(mut ranges: Vec<(u64, u64)>) -> Vec<(u64, u64)> {
    if ranges.is_empty() {
        return ranges;
    }

    // Sort by start position
    ranges.sort_by_key(|&(start, _)| start);

    let mut merged = Vec::new();
    let mut current_start = ranges[0].0;
    let mut current_end = ranges[0].1;

    for &(start, end) in &ranges[1..] {
        if start <= current_end + 1 {
            // Overlapping or adjacent ranges
            current_end = current_end.max(end);
        } else {
            // Non-overlapping range
            merged.push((current_start, current_end));
            current_start = start;
            current_end = end;
        }
    }
    merged.push((current_start, current_end));

    merged
}

pub fn part_two(input: &str) -> u64 {
    let (ranges, _) = parse_input(input);
    let merged = merge_ranges(ranges);

    // Sum the lengths of merged ranges
    let mut total_fresh = 0u64;
    for &(start, end) in &merged {
        total_fresh += end - start + 1;
    }

    total_fresh
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
