//! Day 2: Gift Shop
//!
//! ## Problem Description
//!
//! Part 1: Find all invalid IDs that are exactly a sequence of digits repeated twice.
//! Part 2: Find all invalid IDs that are a sequence of digits repeated at least twice.
//!
//! ## Solution Approach
//!
//! **Input Parsing**: Parse a comma-separated list of ranges (e.g., "11-22,95-115").
//! Each range has a start and end ID.
//!
//! **Part 1 Strategy**: For each range, check each number to see if it's a repeated pattern:
//! - A number is invalid if it consists of some sequence of digits repeated exactly twice
//! - Examples: 11 (5 twice), 6464 (64 twice), 123123 (123 twice)
//! - To check: try all possible pattern lengths that divide the number's length in half
//! - For a number with 2n digits, try pattern lengths 1, 2, ..., n
//! - For each pattern length, check if first half equals second half
//!
//! **Part 2 Strategy**: Extended to check for patterns repeated at least twice:
//! - Try all pattern lengths from 1 to half the digit count
//! - Check if the entire number is composed of the pattern repeated
//! - This handles 123123123 (123 repeated 3 times), 1111111 (1 repeated 7 times), etc.
//!
//! **Complexity**: O(R * M * D) where R is number of ranges, M is max range size,
//! D is number of digits. For each number, we try up to D/2 pattern lengths.
//!
//! **Optimization Note**: Could optimize by pre-checking if number ends in the same
//! digit sequence it starts with, but the straightforward approach is fast enough.

#[derive(Clone, Copy)]
struct Range {
    start: u64,
    end: u64,
}

fn parse_input(input: &str) -> Vec<Range> {
    input
        .trim()
        .trim_end_matches(',')
        .split(',')
        .map(|part| {
            let mut parts = part.split('-');
            let start: u64 = parts.next().unwrap().parse().unwrap();
            let end: u64 = parts.next().unwrap().parse().unwrap();
            Range { start, end }
        })
        .collect()
}

fn is_repeated_twice(n: u64) -> bool {
    let s = n.to_string();
    let len = s.len();

    // Only even-length numbers can be repeated twice
    if !len.is_multiple_of(2) {
        return false;
    }

    let half = len / 2;
    s[..half] == s[half..]
}

fn is_repeated_at_least_twice(n: u64) -> bool {
    let s = n.to_string();
    let len = s.len();

    // Try all possible pattern lengths from 1 to half the string length
    for pattern_len in 1..=len / 2 {
        // Only check if the total length is divisible by pattern length
        if !len.is_multiple_of(pattern_len) {
            continue;
        }

        let pattern = &s[..pattern_len];
        let repeats = len / pattern_len;

        // Check if the entire string is the pattern repeated
        if s.chars()
            .enumerate()
            .all(|(i, c)| c == pattern.chars().nth(i % pattern_len).unwrap())
            && repeats >= 2
        {
            return true;
        }
    }

    false
}

pub fn part_one(input: &str) -> u64 {
    let ranges = parse_input(input);
    let mut sum = 0u64;

    for range in ranges {
        for n in range.start..=range.end {
            if is_repeated_twice(n) {
                sum += n;
            }
        }
    }

    sum
}

pub fn part_two(input: &str) -> u64 {
    let ranges = parse_input(input);
    let mut sum = 0u64;

    for range in ranges {
        for n in range.start..=range.end {
            if is_repeated_at_least_twice(n) {
                sum += n;
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
