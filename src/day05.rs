//! Day 5: Cafeteria
//!
//! ## Problem Description
//!
//! Part 1: Given ranges of fresh ingredient IDs and a list of available IDs,
//! count how many available IDs fall within any of the fresh ranges.
//!
//! Part 2: Calculate the total count of all IDs covered by the fresh ranges,
//! accounting for overlaps between ranges.
//!
//! ## Solution Approach
//!
//! **Input Parsing**: Split input at the blank line. Parse ranges and available IDs.
//!
//! **Part 1**: For each available ID, check if it's within any range.
//! Since the number of available IDs is small, this is efficient.
//!
//! **Part 2**: Merge overlapping ranges and sum the total covered IDs.
//! Sort ranges by start, then iterate to merge overlaps.
//!
//! ## Complexity Analysis
//!
//! Let R be the number of ranges and A be the number of available IDs.
//! - Time Complexity: O(R log R + A * R) for Part 1, O(R log R) for Part 2
//! - Space Complexity: O(R) for storing ranges

/// Represents a range of ingredient IDs
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Range {
    start: u64,
    end: u64,
}

/// Parse the input into ranges and available IDs
pub fn parse_input(input: &str) -> (Vec<Range>, Vec<u64>) {
    let parts: Vec<&str> = input.split("\n\n").collect();

    let mut ranges = Vec::new();
    if let Some(range_section) = parts.get(0) {
        for line in range_section.lines() {
            if let Some((start_str, end_str)) = line.split_once('-') {
                if let (Ok(start), Ok(end)) = (start_str.parse::<u64>(), end_str.parse::<u64>()) {
                    ranges.push(Range { start, end });
                }
            }
        }
    }

    let mut available_ids = Vec::new();
    if parts.len() > 1 {
        if let Some(id_section) = parts.get(1) {
            for line in id_section.lines() {
                if let Ok(id) = line.trim().parse::<u64>() {
                    available_ids.push(id);
                }
            }
        }
    }

    (ranges, available_ids)
}

/// Check if an ID is within any of the ranges
fn is_fresh(ranges: &[Range], id: u64) -> bool {
    for range in ranges {
        if id >= range.start && id <= range.end {
            return true;
        }
    }
    false
}

/// Part 1: Count available IDs that are fresh
pub fn part_one(input: &str) -> impl std::fmt::Display {
    let (ranges, available_ids) = parse_input(input);

    let mut fresh_count = 0;
    for id in available_ids {
        if is_fresh(&ranges, id) {
            fresh_count += 1;
        }
    }

    fresh_count
}

/// Merge overlapping ranges and return the total count of covered IDs
fn count_total_fresh(ranges: &[Range]) -> u64 {
    if ranges.is_empty() {
        return 0;
    }

    // Sort ranges by start
    let mut sorted_ranges = ranges.to_vec();
    sorted_ranges.sort_by_key(|r| r.start);

    // Merge overlapping ranges
    let mut merged = Vec::new();
    let mut current = sorted_ranges[0];

    for &range in &sorted_ranges[1..] {
        if range.start <= current.end + 1 {
            // Overlapping or adjacent, merge them
            current.end = current.end.max(range.end);
        } else {
            merged.push(current);
            current = range;
        }
    }
    merged.push(current);

    // Calculate total covered IDs
    let mut total = 0u64;
    for range in merged {
        // +1 because ranges are inclusive
        total += range.end - range.start + 1;
    }

    total
}

/// Part 2: Count total IDs considered fresh by all ranges
pub fn part_two(input: &str) -> impl std::fmt::Display {
    let (ranges, _) = parse_input(input);
    count_total_fresh(&ranges)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_input() {
        let input = "3-5
10-14
16-20

1
5
8
11
17
32";

        let (ranges, ids) = parse_input(input);
        assert_eq!(ranges.len(), 3);
        assert_eq!(ids.len(), 6);
        assert_eq!(ranges[0], Range { start: 3, end: 5 });
    }

    #[test]
    fn test_is_fresh() {
        let ranges = vec![
            Range { start: 3, end: 5 },
            Range { start: 10, end: 14 },
            Range { start: 16, end: 20 },
        ];

        assert!(is_fresh(&ranges, 5));
        assert!(is_fresh(&ranges, 11));
        assert!(is_fresh(&ranges, 17));
        assert!(!is_fresh(&ranges, 1));
        assert!(!is_fresh(&ranges, 8));
    }

    #[test]
    fn test_count_total_fresh() {
        let ranges = vec![
            Range { start: 3, end: 5 },
            Range { start: 10, end: 14 },
            Range { start: 16, end: 20 },
            Range { start: 12, end: 18 },
        ];

        assert_eq!(count_total_fresh(&ranges), 14);
    }

    #[test]
    fn test_merge_overlapping() {
        let ranges = vec![
            Range { start: 1, end: 5 },
            Range { start: 3, end: 7 },
            Range { start: 10, end: 12 },
        ];

        assert_eq!(count_total_fresh(&ranges), 10); // 1-7 (7 IDs) + 10-12 (3 IDs)
    }

    #[test]
    fn example() {
        let input = "3-5
10-14
16-20
12-18

1
5
8
11
17
32";

        assert_eq!(part_one(input).to_string(), "3");
        assert_eq!(part_two(input).to_string(), "14");
    }
}