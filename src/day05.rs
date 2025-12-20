//! Day 5: Cafeteria
//!
//! ## Problem Description
//!
//! Part 1: Given ranges of fresh ingredient IDs and a list of available
//! IDs, count how many available IDs fall within any fresh range.
//!
//! Part 2: Count the total number of IDs covered by the fresh ranges
//! (after merging overlapping ranges).
//!
//! ## Solution Approach
//!
//! **Part 1**: For each available ID, check if it falls within any
//! fresh range.
//!
//! **Part 2**: Merge overlapping ranges, then sum the size of each
//! merged range.
//!
//! **Complexity**: O(n*m) for part 1, O(n log n) for part 2 where n is
//! the number of ranges and m is the number of IDs.

struct Range {
    start: i64,
    end: i64,
}

fn parse_input(input: &str) -> (Vec<Range>, Vec<i64>) {
    let parts: Vec<&str> = input.split("\n\n").collect();

    let ranges: Vec<Range> = parts[0]
        .trim()
        .lines()
        .map(|line| {
            let nums: Vec<i64> =
                line.split('-').map(|s| s.parse().unwrap()).collect();
            Range {
                start: nums[0],
                end: nums[1],
            }
        })
        .collect();

    let ids: Vec<i64> = parts[1]
        .trim()
        .lines()
        .map(|line| line.parse().unwrap())
        .collect();

    (ranges, ids)
}

fn merge_ranges(ranges: &[Range]) -> Vec<Range> {
    let mut sorted: Vec<(i64, i64)> =
        ranges.iter().map(|r| (r.start, r.end)).collect();
    sorted.sort();

    let mut merged = Vec::new();
    let (mut start, mut end) = sorted[0];

    for &(s, e) in &sorted[1..] {
        if s <= end + 1 {
            // Overlapping or adjacent
            end = end.max(e);
        } else {
            // Non-overlapping
            merged.push(Range { start, end });
            start = s;
            end = e;
        }
    }
    merged.push(Range { start, end });

    merged
}

pub fn part_one(input: &str) -> usize {
    let (ranges, ids) = parse_input(input);

    ids.iter()
        .filter(|&id| ranges.iter().any(|r| *id >= r.start && *id <= r.end))
        .count()
}

pub fn part_two(input: &str) -> i64 {
    let (ranges, _) = parse_input(input);
    let merged = merge_ranges(&ranges);

    merged.iter().map(|r| r.end - r.start + 1).sum()
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
