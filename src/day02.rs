//! Day 02: Gift Shop
//!
//! ## Problem Description
//!
//! The Elves have invalid product IDs in their gift shop database.
//! These IDs are stored in ranges (e.g., "11-22,95-115").
//!
//! **Part 1**: Find IDs that are made only of some sequence of digits repeated exactly twice.
//! Examples: 55 (5 twice), 6464 (64 twice), 123123 (123 twice).
//!
//! **Part 2**: Find IDs that are made only of some sequence of digits repeated at least twice.
//! Examples: 12341234 (1234 two times), 123123123 (123 three times), 1111111 (1 seven times).
//!
//! ## Solution Approach
//!
//! **Input Parsing**: Parse ranges from comma-separated "start-end" format.
//! For Part 1: Check each number in range if it's a doubled sequence.
//! For Part 2: Check each number in range if it's made of some sequence repeated at least twice.
//!
//! **Complexity**: O(N*M) where N is the number of ranges and M is the average range size.
//!
//! ## Algorithm Explanation
//!
//! **Part 1**: For each number in each range, check if it consists of a sequence repeated exactly twice.
//!
//! **Part 2**: For each number in each range, check if it consists of any sequence repeated at least twice.

use std::str::FromStr;

#[derive(Debug)]
struct Range {
    start: u64,
    end: u64,
}

impl FromStr for Range {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some((start_str, end_str)) = s.split_once('-') {
            let start = start_str.parse::<u64>().map_err(|_| ())?;
            let end = end_str.parse::<u64>().map_err(|_| ())?;
            Ok(Range { start, end })
        } else {
            Err(())
        }
    }
}

fn parse_input(input: &str) -> Vec<Range> {
    let all_lines: String = input
        .lines()
        .map(|line| line.trim().to_string())
        .collect::<Vec<String>>()
        .join("");

    all_lines
        .split(',')
        .filter(|s| !s.is_empty())
        .map(|s| s.trim().parse::<Range>().unwrap())
        .collect()
}

/// Checks if a number is made of a sequence repeated exactly twice
fn is_doubled_number_part1(n: u64) -> bool {
    let s = n.to_string();

    // If the length is odd, it can't be a perfect double
    if s.len() % 2 != 0 {
        return false;
    }

    let mid = s.len() / 2;
    let first_half = &s[0..mid];
    let second_half = &s[mid..];

    // The number must be made of the same sequence repeated twice
    first_half == second_half
}

/// Checks if a number is made of a sequence repeated at least twice
fn is_repeated_number_part2(n: u64) -> bool {
    let s = n.to_string();
    let len = s.len();

    // If length is less than 2, it can't have a sequence repeated at least twice
    if len < 2 {
        return false;
    }

    // Try all possible sequence lengths (from 1 to len/2)
    for seq_len in 1..=(len / 2) {
        if len % seq_len == 0 {
            // Check if the string can be divided evenly by this sequence
            let sequence = &s[0..seq_len];
            let mut valid = true;

            // Check each chunk of size seq_len
            for i in (seq_len..len).step_by(seq_len) {
                if &s[i..i + seq_len] != sequence {
                    valid = false;
                    break;
                }
            }

            if valid {
                return true;
            }
        }
    }

    false
}

pub fn part_one(input: &str) -> u64 {
    let ranges = parse_input(input);
    let mut total = 0;

    for range in ranges {
        for num in range.start..=range.end {
            if is_doubled_number_part1(num) {
                total += num;
            }
        }
    }

    total
}

pub fn part_two(input: &str) -> u64 {
    let ranges = parse_input(input);
    let mut total = 0;

    for range in ranges {
        for num in range.start..=range.end {
            if is_repeated_number_part2(num) {
                total += num;
            }
        }
    }

    total
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
