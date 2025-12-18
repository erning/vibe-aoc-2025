//! Day 2: Gift Shop
//!
//! ## Puzzle Overview
//!
//! The Elves need to identify invalid product IDs in gift shop database ranges.
//! An ID is invalid if it's made of some sequence of digits repeated.
//!
//! ## Part 1 Strategy
//!
//! An ID is invalid if it consists of some sequence of digits repeated **exactly twice**.
//! For example: 55 (5 repeated twice), 6464 (64 repeated twice), 123123 (123 repeated twice).
//! No leading zeros are allowed.
//!
//! **Algorithm**:
//! 1. Parse ranges from input (format: "start-end,start-end,...")
//! 2. For each number in each range:
//!    - Check if the number string can be divided into equal halves
//!    - If both halves are identical, the number is invalid
//! 3. Sum all invalid numbers
//!
//! **Complexity**: O(n*m) where n is the number of ranges and m is the average range size
//!
//! ## Part 2 Strategy
//!
//! An ID is invalid if it consists of some sequence of digits repeated **at least twice**.
//! For example: 12341234 (1234 twice), 123123123 (123 three times), 1212121212 (12 five times),
//! 1111111 (1 seven times).
//!
//! **Algorithm**:
//! 1. Parse ranges from input (format: "start-end,start-end,...")
//! 2. For each number in each range:
//!    - Check if the number string can be divided into equal parts
//!    - Try all possible pattern lengths that divide the string evenly
//!    - If a pattern repeats at least twice, the number is invalid
//! 3. Sum all invalid numbers
//!
//! **Complexity**: O(n*m*k) where n is the number of ranges, m is the average range size,
//! and k is the number of divisors to check

/// Parse input string into a list of ranges (start, end)
fn parse_input(input: &str) -> Vec<(u64, u64)> {
    input
        .trim()
        .split(',')
        .map(|range| {
            let parts: Vec<&str> = range.split('-').collect();
            (parts[0].parse().unwrap(), parts[1].parse().unwrap())
        })
        .collect()
}

/// Check if a number is invalid (Part 1: exactly two repetitions)
fn is_invalid_part_one(num_str: &str) -> bool {
    let len = num_str.len();
    if !len.is_multiple_of(2) {
        return false;
    }
    let half = len / 2;
    num_str[..half] == num_str[half..]
}

/// Check if a number is invalid (Part 2: at least two repetitions)
fn is_invalid_part_two(num_str: &str) -> bool {
    let len = num_str.len();

    // Try all possible pattern lengths that divide the string evenly
    for pattern_len in 1..=len / 2 {
        if !len.is_multiple_of(pattern_len) {
            continue;
        }

        let repetitions = len / pattern_len;
        if repetitions < 2 {
            continue;
        }

        // Check if all repetitions are the same
        let pattern = &num_str[..pattern_len];
        let mut all_same = true;
        for i in 1..repetitions {
            if &num_str[i * pattern_len..(i + 1) * pattern_len] != pattern {
                all_same = false;
                break;
            }
        }

        if all_same {
            return true;
        }
    }

    false
}

pub fn part_one(input: &str) -> u64 {
    let ranges = parse_input(input);
    let mut sum = 0;

    for (start, end) in ranges {
        for num in start..=end {
            let num_str = num.to_string();
            if is_invalid_part_one(&num_str) {
                sum += num;
            }
        }
    }

    sum
}

pub fn part_two(input: &str) -> u64 {
    let ranges = parse_input(input);
    let mut sum = 0;

    for (start, end) in ranges {
        for num in start..=end {
            let num_str = num.to_string();
            if is_invalid_part_two(&num_str) {
                sum += num;
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
