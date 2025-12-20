//! Day 2: Gift Shop
//!
//! ## Problem Description
//!
//! Part 1: Find invalid product IDs in ranges. An ID is invalid if it's
//! made of a digit sequence repeated exactly twice (e.g., 11, 6464,
//! 123123). Sum all invalid IDs.
//!
//! Part 2: An ID is invalid if it's made of a digit sequence repeated
//! at least twice (e.g., 111, 12341234, 123123123). Sum all invalid IDs.
//!
//! ## Solution Approach
//!
//! **Input Parsing**: Parse comma-separated ranges (e.g., "11-22")
//! into start-end pairs.
//!
//! **Part 1 Strategy**: For each range, iterate through IDs and check
//! if the ID string can be split into two equal halves. If yes, it's
//! invalid.
//!
//! **Part 2 Strategy**: For each ID, check all possible pattern lengths
//! (from 1 to len/2) to see if the ID is made of that pattern repeated
//! at least twice.
//!
//! **Complexity**: O(n*r*d) where n is the number of ranges, r is the
//! range size, and d is the number of digits in each ID.

struct Range {
    start: i64,
    end: i64,
}

fn parse_input(input: &str) -> Vec<Range> {
    input
        .trim()
        .split(',')
        .map(|range| {
            let parts: Vec<&str> = range.split('-').collect();
            Range {
                start: parts[0].parse().unwrap(),
                end: parts[1].parse().unwrap(),
            }
        })
        .collect()
}

fn is_invalid_part1(id: i64) -> bool {
    let s = id.to_string();
    let len = s.len();

    // Must have even length to be split into two equal parts
    if !len.is_multiple_of(2) {
        return false;
    }

    let half = len / 2;
    let left = &s[..half];
    let right = &s[half..];

    // Check if both halves are equal and no leading zeros
    left == right && !left.starts_with('0')
}

fn is_invalid_part2(id: i64) -> bool {
    let s = id.to_string();
    let len = s.len();

    // Try all possible pattern lengths (from 1 to len/2)
    for pattern_len in 1..=(len / 2) {
        if len.is_multiple_of(pattern_len) {
            let pattern = &s[..pattern_len];

            // Check if no leading zeros
            if pattern.starts_with('0') {
                continue;
            }

            // Check if the entire string is this pattern repeated
            let mut is_repeated = true;
            for i in (pattern_len..len).step_by(pattern_len) {
                if &s[i..i + pattern_len] != pattern {
                    is_repeated = false;
                    break;
                }
            }

            if is_repeated {
                return true;
            }
        }
    }

    false
}

pub fn part_one(input: &str) -> i64 {
    let ranges = parse_input(input);
    let mut sum = 0;

    for range in ranges {
        for id in range.start..=range.end {
            if is_invalid_part1(id) {
                sum += id;
            }
        }
    }

    sum
}

pub fn part_two(input: &str) -> i64 {
    let ranges = parse_input(input);
    let mut sum = 0;

    for range in ranges {
        for id in range.start..=range.end {
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
