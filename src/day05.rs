//! Day 5: Cafeteria
//!
//! Determine fresh ingredients from ID ranges in the kitchen inventory system.
//!
//! ## Solution Approach
//!
//! **Input Parsing**: Parse fresh ID ranges (start-end) and available ingredient
//! IDs separated by a blank line.
//!
//! **Part 1 Strategy**: Count available IDs that fall within any fresh range.
//! - For each available ID, check if it's in any range
//! - Complexity: O(n * m) where n = available IDs, m = ranges
//!
//! **Part 2 Strategy**: Count total unique fresh IDs across all ranges.
//! - Sort ranges by start, merge overlapping/adjacent ranges
//! - Sum the size of each merged range
//! - Complexity: O(m log m) for sorting + O(m) for merging

/// A range of fresh ingredient IDs (inclusive on both ends)
#[derive(Debug, Clone, Copy)]
struct Range {
    start: u64,
    end: u64,
}

impl Range {
    fn contains(&self, id: u64) -> bool {
        id >= self.start && id <= self.end
    }

    fn size(&self) -> u64 {
        self.end - self.start + 1
    }
}

fn parse_input(input: &str) -> (Vec<Range>, Vec<u64>) {
    let mut parts = input.split("\n\n");

    let ranges: Vec<Range> = parts
        .next()
        .unwrap()
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            let mut nums = line.split('-');
            let start: u64 = nums.next().unwrap().parse().unwrap();
            let end: u64 = nums.next().unwrap().parse().unwrap();
            Range { start, end }
        })
        .collect();

    let ids: Vec<u64> = parts
        .next()
        .unwrap()
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| line.parse().unwrap())
        .collect();

    (ranges, ids)
}

/// Check if an ID is fresh (falls within any range)
fn is_fresh(ranges: &[Range], id: u64) -> bool {
    ranges.iter().any(|r| r.contains(id))
}

/// Merge overlapping ranges and return the total count of unique IDs
fn count_unique_fresh_ids(ranges: &[Range]) -> u64 {
    if ranges.is_empty() {
        return 0;
    }

    // Sort ranges by start position
    let mut sorted: Vec<Range> = ranges.to_vec();
    sorted.sort_by_key(|r| r.start);

    // Merge overlapping/adjacent ranges
    let mut merged: Vec<Range> = Vec::new();
    let mut current = sorted[0];

    for range in sorted.iter().skip(1) {
        // Check if ranges overlap or are adjacent (end + 1 >= start)
        if range.start <= current.end + 1 {
            // Extend current range if needed
            current.end = current.end.max(range.end);
        } else {
            // No overlap, save current and start new
            merged.push(current);
            current = *range;
        }
    }
    merged.push(current);

    // Sum the sizes of all merged ranges
    merged.iter().map(|r| r.size()).sum()
}

pub fn part_one(input: &str) -> u64 {
    let (ranges, ids) = parse_input(input);
    ids.iter().filter(|&&id| is_fresh(&ranges, id)).count() as u64
}

pub fn part_two(input: &str) -> u64 {
    let (ranges, _) = parse_input(input);
    count_unique_fresh_ids(&ranges)
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

    #[test]
    fn test_range_contains() {
        let range = Range { start: 3, end: 5 };
        assert!(!range.contains(2));
        assert!(range.contains(3));
        assert!(range.contains(4));
        assert!(range.contains(5));
        assert!(!range.contains(6));
    }

    #[test]
    fn test_merge_overlapping() {
        // Ranges: 3-5, 10-14, 16-20, 12-18
        // After merge: 3-5, 10-20
        // Total: 3 + 11 = 14
        let ranges = vec![
            Range { start: 3, end: 5 },
            Range { start: 10, end: 14 },
            Range { start: 16, end: 20 },
            Range { start: 12, end: 18 },
        ];
        assert_eq!(count_unique_fresh_ids(&ranges), 14);
    }
}
