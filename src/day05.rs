//! Day 05: Cafeteria
//!
//! ## Problem Description
//!
//! The Elves have a database with fresh ingredient ID ranges and available ingredient IDs.
//!
//! **Part 1**: Count how many of the available ingredient IDs are fresh (fall into any range).
//! **Part 2**: Count the total number of ingredient IDs that are considered fresh by the ranges.
//!
//! ## Solution Approach
//!
//! **Input Parsing**: Parse ranges and available IDs from input.
//! Part 1: Check each available ID against all ranges.
//! Part 2: Merge overlapping ranges and count total IDs covered.
//!
//! **Complexity**: O(N*M) for Part 1 where N is available IDs and M is ranges,
//! O(R*log(R)) for Part 2 where R is number of ranges.
//!
//! ## Algorithm Explanation
//!
//! Part 1: For each available ID, check if it falls within any fresh range.
//! Part 2: Normalize intervals by merging overlapping ranges, then count total unique IDs.

#[derive(Debug, Clone, Copy)]
struct Range {
    start: i64,
    end: i64,
}

fn parse_input(input: &str) -> (Vec<Range>, Vec<i64>) {
    let sections: Vec<&str> = input.split("\n\n").collect();
    let ranges = parse_ranges(sections[0]);
    let available_ids = parse_available_ids(sections[1]);

    (ranges, available_ids)
}

fn parse_ranges(ranges_str: &str) -> Vec<Range> {
    ranges_str
        .lines()
        .map(|line| {
            let line = line.trim();
            if line.is_empty() {
                return Range { start: 0, end: -1 }; // Return invalid range for empty lines
            }
            let parts: Vec<&str> = line.split('-').collect();
            let start = parts[0].parse::<i64>().unwrap();
            let end = parts[1].parse::<i64>().unwrap();
            Range { start, end }
        })
        .filter(|r| r.start <= r.end) // Filter out invalid ranges
        .collect()
}

fn parse_available_ids(ids_str: &str) -> Vec<i64> {
    ids_str
        .lines()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty())
        .map(|line| line.parse::<i64>().unwrap())
        .collect()
}

pub fn part_one(input: &str) -> u32 {
    let (ranges, available_ids) = parse_input(input);
    let mut count = 0;

    for id in available_ids {
        // Check if id falls within any range
        for range in &ranges {
            if id >= range.start && id <= range.end {
                count += 1;
                break; // Found, no need to check other ranges
            }
        }
    }

    count
}

pub fn part_two(input: &str) -> u64 {
    let (ranges, _) = parse_input(input);

    // Normalize the ranges by merging overlapping ones
    let merged_ranges = merge_overlapping_ranges(ranges);

    // Count the total number of IDs covered by all ranges
    let mut total = 0u64;
    for range in merged_ranges {
        total += range.end as u64 - range.start as u64 + 1;
    }

    total
}

fn merge_overlapping_ranges(mut ranges: Vec<Range>) -> Vec<Range> {
    if ranges.is_empty() {
        return ranges;
    }

    // Sort ranges by start value
    ranges.sort_by(|a, b| a.start.cmp(&b.start));

    let mut result = Vec::new();
    let mut current = ranges[0];

    for &range in &ranges[1..] {
        if range.start <= current.end + 1 {
            // Overlapping or adjacent ranges, merge them
            current.end = current.end.max(range.end);
        } else {
            // Non-overlapping range, add current to result and start new
            result.push(current);
            current = range;
        }
    }

    // Add the last range
    result.push(current);

    result
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
