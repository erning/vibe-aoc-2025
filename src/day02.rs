//! Day 02: Gift Shop (AoC 2025)
//!
//! ## Problem Description
//!
//! Find invalid product IDs within given ranges. An ID is invalid if it consists
//! of a repeating digit pattern.
//!
//! **Part 1**: An ID is invalid if it's made of exactly 2 repetitions of a pattern
//! (e.g., 55, 6464, 123123).
//!
//! **Part 2**: An ID is invalid if it's made of at least 2 repetitions of a pattern
//! (e.g., 11, 111, 1111, 12341234, 123123123).
//!
//! ## Solution Approach
//!
//! **Input Parsing**: Parse comma-separated ranges in format "start-end".
//!
//! **Validation Functions**:
//! - For Part 1: Check if ID can be split into exactly 2 equal halves
//! - For Part 2: Check all possible pattern lengths (1 to len/2) for repetition
//!
//! **Part 1 Strategy**:
//! - For each range, iterate through all IDs
//! - Split ID string in half and compare
//! - Sum all invalid IDs
//!
//! **Part 2 Strategy**:
//! - For each range, iterate through all IDs
//! - Try all pattern lengths k from 1 to len/2
//! - Check if pattern repeats at least twice covering the full ID
//! - Sum all invalid IDs
//!
//! **Complexity**: O(n*m*k) where n is number of ranges, m is avg range size, k is ID length

fn is_invalid_part1(id: u64) -> bool {
    let s = id.to_string();
    if !s.len().is_multiple_of(2) {
        return false;
    }
    let half = s.len() / 2;
    s[..half] == s[half..]
}

fn is_invalid_part2(id: u64) -> bool {
    let s = id.to_string();
    let len = s.len();

    // Try all possible pattern lengths from 1 to len/2
    for pattern_len in 1..=len / 2 {
        // Check if the ID is made of this pattern repeated at least twice
        let pattern = &s[..pattern_len];

        // Pattern must repeat at least twice to be invalid
        let min_repeats = (len as f64 / pattern_len as f64).ceil() as usize;
        if min_repeats < 2 {
            continue;
        }

        // Verify that the string consists of repetitions of this pattern
        let mut matches = true;
        for i in 0..len {
            let pattern_idx = i % pattern_len;
            if s.chars().nth(i) != pattern.chars().nth(pattern_idx) {
                matches = false;
                break;
            }
        }

        if matches {
            return true;
        }
    }

    false
}

fn parse_input(input: &str) -> Vec<(u64, u64)> {
    input
        .trim()
        .split(',')
        .map(|range| {
            let mut parts = range.split('-');
            let start = parts.next().unwrap().parse().unwrap();
            let end = parts.next().unwrap().parse().unwrap();
            (start, end)
        })
        .collect()
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
    fn example() {
        let input = read_example(2);
        assert_eq!(part_one(&input), 1227775554);
        assert_eq!(part_two(&input), 4174379265);
    }
}
