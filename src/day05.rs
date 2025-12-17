//! Day 5: Cafeteria
//!
//! ## Problem Description
//!
//! Part 1: Count how many available ingredient IDs are fresh (fall within any range).
//! Part 2: Count total unique ingredient IDs that ranges consider fresh.
//!
//! ## Solution Approach
//!
//! **Input Parsing**: Parse ranges section and available IDs section, separated by blank line.
//!
//! **Part 1 Strategy**:
//! - Parse all ranges and build set of fresh IDs (only for reasonable ranges)
//! - Check each available ID against fresh set
//! - Count matches
//!
//! **Part 2 Strategy**:
//! - Calculate total span of all ranges without iterating individual IDs
//! - Handle overlapping ranges efficiently
//!
//! **Complexity**: O(n + m) where n is number of ranges and m is available IDs.

fn parse_input(input: &str) -> (Vec<(u64, u64)>, Vec<u64>) {
    let sections: Vec<&str> = input.split("\n\n").collect();

    // Parse ranges section
    let ranges: Vec<(u64, u64)> = sections[0]
        .lines()
        .map(|line| {
            let parts: Vec<&str> = line.split('-').collect();
            let start = parts[0].parse::<u64>().unwrap();
            let end = parts[1].parse::<u64>().unwrap();
            (start, end)
        })
        .collect();

    // Parse available IDs section
    let available: Vec<u64> = sections[1]
        .lines()
        .map(|line| line.parse::<u64>().unwrap())
        .collect();

    (ranges, available)
}

fn is_fresh(id: u64, ranges: &[(u64, u64)]) -> bool {
    for (start, end) in ranges {
        if id >= *start && id <= *end {
            return true;
        }
    }
    false
}

pub fn part_one(input: &str) -> usize {
    let (ranges, available) = parse_input(input);

    let mut fresh_count = 0;
    for id in available {
        if is_fresh(id, &ranges) {
            fresh_count += 1;
        }
    }

    fresh_count
}

fn merge_overlapping_ranges(ranges: &[(u64, u64)]) -> Vec<(u64, u64)> {
    if ranges.is_empty() {
        return Vec::new();
    }

    let mut sorted_ranges = ranges.to_vec();
    sorted_ranges.sort_by_key(|(start, _)| *start);

    let mut merged = Vec::new();
    let mut current_start = sorted_ranges[0].0;
    let mut current_end = sorted_ranges[0].1;

    for (start, end) in sorted_ranges.iter().skip(1) {
        if *start <= current_end + 1 {
            // Overlapping or adjacent ranges, merge them
            current_start = current_start.min(*start);
            current_end = current_end.max(*end);
        } else {
            // Non-overlapping, save current range and start new one
            merged.push((current_start, current_end));
            current_start = *start;
            current_end = *end;
        }
    }
    merged.push((current_start, current_end));

    merged
}

pub fn part_two(input: &str) -> u64 {
    let (ranges, _) = parse_input(input);
    let merged_ranges = merge_overlapping_ranges(&ranges);

    let mut total_count = 0u64;
    for (start, end) in merged_ranges {
        total_count += end - start + 1;
    }

    total_count
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::read_example;

    #[test]
    fn example() {
        let input = read_example(5);
        assert_eq!(part_one(&input), 3);
        assert_eq!(part_two(&input), 14);
    }
}
