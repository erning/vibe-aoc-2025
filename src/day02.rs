//! Day 2: Gift Shop
//!
//! ## Problem Description
//!
//! Part 1: Find invalid product IDs in ranges. An ID is invalid if it consists of some
//! sequence of digits repeated exactly twice (e.g., 11, 6464, 123123).
//!
//! Part 2: An ID is invalid if it consists of some sequence of digits repeated
//! at least twice (e.g., 12341234, 123123123, 1111111).
//!
//! ## Solution Approach
//!
//! **Input Parsing**: Parse the comma-separated ranges into (start, end) pairs.
//!
//! **Part 1**: For each ID in each range, check if it's made of a pattern repeated
//! exactly twice. We do this by checking all possible pattern lengths from 1 to
//! half the ID length.
//!
//! **Part 2**: Similar to Part 1, but check if the pattern repeats at least twice.
//! This means we check if the ID length is a multiple of the pattern length and
//! the pattern repeats throughout the entire ID.
//!
//! ## Complexity Analysis
//!
//! Let N be the number of IDs to check and L be the average length of IDs.
//! - Time Complexity: O(N * L^2) in the worst case (checking all pattern lengths)
//! - Space Complexity: O(1) - constant space for pattern checking

/// Represents a range of product IDs
#[derive(Debug, Clone, Copy)]
pub struct IdRange {
    start: u64,
    end: u64,
}

/// Parse the input string into a vector of ID ranges
pub fn parse_input(input: &str) -> Vec<IdRange> {
    input
        .trim()
        .split(',')
        .filter_map(|range_str| {
            let mut parts = range_str.trim().split('-');
            let start: u64 = parts.next()?.parse().ok()?;
            let end: u64 = parts.next()?.parse().ok()?;

            if start <= end {
                Some(IdRange { start, end })
            } else {
                None
            }
        })
        .collect()
}

/// Check if a number consists of a pattern repeated exactly twice
fn is_invalid_part_one(mut n: u64) -> bool {
    if n < 10 {
        return false; // Single digit can't be repeated
    }

    let digits: Vec<u8> = {
        let mut d = Vec::new();
        if n == 0 {
            d.push(0);
        } else {
            while n > 0 {
                d.push((n % 10) as u8);
                n /= 10;
            }
            d.reverse();
        }
        d
    };

    let len = digits.len();
    if !len.is_multiple_of(2) {
        return false; // Must have even length for exact repetition
    }

    let half_len = len / 2;
    for i in 0..half_len {
        if digits[i] != digits[half_len + i] {
            return false;
        }
    }

    true
}

/// Check if a number consists of a pattern repeated at least twice
fn is_invalid_part_two(mut n: u64) -> bool {
    if n < 10 {
        return false; // Single digit can't be repeated
    }

    let digits: Vec<u8> = {
        let mut d = Vec::new();
        if n == 0 {
            d.push(0);
        } else {
            while n > 0 {
                d.push((n % 10) as u8);
                n /= 10;
            }
            d.reverse();
        }
        d
    };

    let len = digits.len();

    // Try all possible pattern lengths from 1 to len/2
    for pattern_len in 1..=len / 2 {
        if !len.is_multiple_of(pattern_len) {
            continue; // Pattern must divide the length
        }

        let repeats = len / pattern_len;
        if repeats < 2 {
            continue; // Need at least 2 repeats
        }

        let mut is_valid_pattern = true;
        for i in 0..pattern_len {
            let pattern_digit = digits[i];
            for j in 1..repeats {
                if digits[j * pattern_len + i] != pattern_digit {
                    is_valid_pattern = false;
                    break;
                }
            }
            if !is_valid_pattern {
                break;
            }
        }

        if is_valid_pattern {
            return true;
        }
    }

    false
}

/// Part 1: Sum all invalid IDs (pattern repeated exactly twice)
pub fn part_one(input: &str) -> impl std::fmt::Display {
    let ranges = parse_input(input);
    let mut sum = 0u64;

    for range in ranges {
        for id in range.start..=range.end {
            if is_invalid_part_one(id) {
                sum += id;
            }
        }
    }

    sum
}

/// Part 2: Sum all invalid IDs (pattern repeated at least twice)
pub fn part_two(input: &str) -> impl std::fmt::Display {
    let ranges = parse_input(input);
    let mut sum = 0u64;

    for range in ranges {
        for id in range.start..=range.end {
            if is_invalid_part_two(id) {
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
        assert_eq!(part_one(&input).to_string(), "1227775554");
        assert_eq!(part_two(&input).to_string(), "4174379265");
    }
}
