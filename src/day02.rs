//! Day 2: Gift Shop
//!
//! ## Problem Description
//!
//! Part 1: Find all invalid product IDs in given ranges. An invalid ID is
//! one made only of a sequence of digits repeated exactly twice.
//! For example: 55 (5 twice), 6464 (64 twice), 123123 (123 twice).
//!
//! Part 2: An invalid ID is now one made of a sequence of digits repeated
//! at least twice. For example: 12341234 (1234 twice), 123123123 (123 three
//! times), 1212121212 (12 five times), 1111111 (1 seven times).
//!
//! ## Solution Approach
//!
//! **Input Parsing**: Parse comma-separated ranges in format "start-end".
//!
//! **Helper Functions**:
//! - `is_invalid_part1()`: Check if ID is made of a pattern repeated exactly
//!   twice
//! - `is_invalid_part2()`: Check if ID is made of a pattern repeated at least
//!   twice
//!
//! **Strategy**:
//! - For each range, iterate through all IDs
//! - For each ID, check if it's invalid using the appropriate function
//! - Sum all invalid IDs found
//!
//! For efficiency, check pattern lengths from 1 to half the ID length.
//! An ID of length n can have patterns of length 1..n/2 at most.
//!
//! **Complexity**: O(r * d * n²) where r is number of ranges, d is max
//! distance per range, and n is max ID length. For typical inputs, this
//! is acceptable since we only check valid pattern lengths.

fn parse_input(input: &str) -> Vec<(u64, u64)> {
    input
        .trim()
        .split(',')
        .map(|range| {
            let parts: Vec<&str> = range.split('-').collect();
            let start = parts[0].parse().unwrap();
            let end = parts[1].parse().unwrap();
            (start, end)
        })
        .collect()
}

fn is_invalid_part1(id: u64) -> bool {
    let s = id.to_string();
    let len = s.len();

    // A valid repeated pattern for part 1 must be exactly half the
    // length
    if !len.is_multiple_of(2) {
        return false;
    }

    let half = len / 2;
    let (first, second) = s.split_at(half);
    first == second
}

fn is_invalid_part2(id: u64) -> bool {
    let s = id.to_string();
    let len = s.len();

    // Check all possible pattern lengths from 1 to len/2
    for pattern_len in 1..=len / 2 {
        if !len.is_multiple_of(pattern_len) {
            continue;
        }

        let pattern = &s[0..pattern_len];
        let mut is_repeating = true;

        // Check if the entire string is made of this pattern repeated
        for i in 0..len {
            if s.chars().nth(i).unwrap()
                != pattern.chars().nth(i % pattern_len).unwrap()
            {
                is_repeating = false;
                break;
            }
        }

        if is_repeating {
            return true;
        }
    }

    false
}

pub fn part_one(input: &str) -> u64 {
    let ranges = parse_input(input);
    let mut sum = 0;

    for (start, end) in ranges {
        for id in start..=end {
            if is_invalid_part1(id) {
                sum += id;
            }
        }
    }

    sum
}

pub fn part_two(input: &str) -> u64 {
    let ranges = parse_input(input);
    let mut sum = 0;

    for (start, end) in ranges {
        for id in start..=end {
            if is_invalid_part2(id) {
                sum += id;
            }
        }
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::read_example;

    #[test]
    fn example_part_one() {
        let input = read_example(2);
        assert_eq!(part_one(&input), 1227775554);
    }

    #[test]
    fn example_part_two() {
        let input = read_example(2);
        assert_eq!(part_two(&input), 4174379265);
    }
}
