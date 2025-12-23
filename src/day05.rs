//! Day 5: Cafeteria
//!
//! Part 1: Count how many ingredient IDs fall within any fresh range.
//! Part 2: Compute the total size of merged fresh ranges (union).
//!
//! ## Solution Approach
//!
//! **Input Parsing**: Two sections separated by blank line:
//! - First section: fresh ingredient ID ranges (start-end format)
//! - Second section: available ingredient IDs to check (one per line)
//!
//! **Part 1 Strategy**:
//! - Parse all ranges into (start, end) pairs
//! - For each available ID, check if it falls within any range
//! - Count matches
//!
//! **Part 2 Strategy**:
//! - Parse all ranges (large numbers requiring u128)
//! - Sort ranges by start value
//! - Merge overlapping/adjacent ranges
//! - Sum up the sizes (end - start + 1 for inclusive ranges)
//!
//! **Complexity**: O(n log n) for sorting ranges in Part 2, where n is
//! the number of ranges. Part 1 is O(m * n) where m is number of IDs.
//!
//! **Optimization Note**: For Part 1, could use binary search on sorted
//! ranges for O(m log n) complexity, but the straightforward approach
//! is sufficient for the given input sizes.

#[derive(Debug, Clone, PartialEq, Eq)]
struct Range {
    start: u128,
    end: u128,
}

fn parse_input(input: &str) -> (Vec<Range>, Vec<u128>) {
    let parts: Vec<&str> = input.split("\n\n").collect();
    let range_input = parts[0];
    let ids_input = if parts.len() > 1 { parts[1] } else { "" };

    let mut ranges = Vec::new();
    for line in range_input.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        let parts: Vec<&str> = line.split('-').collect();
        if parts.len() == 2 {
            let start: u128 = parts[0].parse().unwrap();
            let end: u128 = parts[1].parse().unwrap();
            ranges.push(Range { start, end });
        }
    }

    let mut ids = Vec::new();
    for line in ids_input.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        if let Ok(id) = line.parse::<u128>() {
            ids.push(id);
        }
    }

    (ranges, ids)
}

pub fn part_one(input: &str) -> usize {
    let (ranges, ids) = parse_input(input);

    ids.iter()
        .filter(|&&id| ranges.iter().any(|r| id >= r.start && id <= r.end))
        .count()
}

pub fn part_two(input: &str) -> u128 {
    let (ranges, _ids) = parse_input(input);

    if ranges.is_empty() {
        return 0;
    }

    // Sort ranges by start value
    let mut sorted = ranges.clone();
    sorted.sort_by_key(|r| r.start);

    // Merge overlapping ranges
    let mut merged: Vec<Range> = Vec::new();
    for range in sorted {
        if let Some(last) = merged.last_mut() {
            // Check if ranges overlap or are adjacent
            if range.start <= last.end + 1 {
                // Merge: extend the end if needed
                last.end = last.end.max(range.end);
            } else {
                merged.push(range);
            }
        } else {
            merged.push(range);
        }
    }

    // Calculate total size
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
