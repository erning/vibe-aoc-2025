//! Day 2: Gift Shop
//!
//! ## Problem Description
//!
//! Part 1: Find invalid product IDs that are made only of some sequence of digits repeated twice.
//! Part 2: Find invalid product IDs that are made only of some sequence of digits repeated at least twice.
//!
//! ## Solution Approach
//!
//! **Input Parsing**: Parse comma-separated ranges in format "start-end".
//!
//! **Part 1 Strategy**:
//! - For each range, iterate through all numbers
//! - Check if number consists of a digit sequence repeated exactly twice
//! - Sum all invalid IDs
//!
//! **Part 2 Strategy**:
//! - Same as Part 1, but allow sequences repeated 2 or more times
//! - This catches patterns like "123123123" (3 repetitions)
//!
//! **Complexity**: O(n * m) where n is total numbers in ranges and m is number length.

use std::ops::RangeInclusive;

fn parse_input(input: &str) -> Vec<RangeInclusive<u64>> {
    input
        .trim()
        .split(',')
        .map(|range_str| {
            let parts: Vec<&str> = range_str.split('-').collect();
            let start = parts[0].parse::<u64>().unwrap();
            let end = parts[1].parse::<u64>().unwrap();
            start..=end
        })
        .collect()
}

fn is_invalid_id_part1(id: u64) -> bool {
    let id_str = id.to_string();
    let len = id_str.len();

    if !len.is_multiple_of(2) || len == 0 {
        return false;
    }

    let half_len = len / 2;
    let first_half = &id_str[0..half_len];
    let second_half = &id_str[half_len..];

    first_half == second_half
}

fn is_invalid_id_part2(id: u64) -> bool {
    let id_str = id.to_string();
    let len = id_str.len();

    if len < 2 {
        return false;
    }

    // Check all possible pattern lengths that could repeat at least twice
    for pattern_len in 1..=len / 2 {
        if !len.is_multiple_of(pattern_len) {
            continue;
        }

        let pattern = &id_str[0..pattern_len];
        let mut is_valid_pattern = true;

        // Check if the entire string consists of this pattern repeated
        for i in (pattern_len..len).step_by(pattern_len) {
            let chunk = &id_str[i..i + pattern_len];
            if chunk != pattern {
                is_valid_pattern = false;
                break;
            }
        }

        if is_valid_pattern {
            return true;
        }
    }

    false
}

pub fn part_one(input: &str) -> u64 {
    let ranges = parse_input(input);
    let mut invalid_sum = 0u64;

    for range in ranges {
        for id in range {
            if is_invalid_id_part1(id) {
                invalid_sum += id;
            }
        }
    }

    invalid_sum
}

pub fn part_two(input: &str) -> u64 {
    let ranges = parse_input(input);
    let mut invalid_sum = 0u64;

    for range in ranges {
        for id in range {
            if is_invalid_id_part2(id) {
                invalid_sum += id;
            }
        }
    }

    invalid_sum
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
