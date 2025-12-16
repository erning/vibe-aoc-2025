//! Day 2: Gift Shop
//!
//! Find invalid product IDs in ranges. An invalid ID is a number whose
//! digit string is a sequence repeated multiple times.
//!
//! ## Solution Approach
//!
//! **Input Parsing**: Parse comma-separated ranges in format "start-end".
//!
//! **Part 1 Strategy**: Find IDs where digits are exactly a sequence repeated
//! twice (e.g., `55` = "5"+"5", `1234` = "12"+"12").
//! - For each ID in range, check if length is even
//! - Check if first half equals second half
//! - Sum all invalid IDs
//! - Complexity: O(R * D) where R is total range size, D is max digits
//!
//! **Part 2 Strategy**: Find IDs where digits are a sequence repeated 2+
//! times (e.g., `111` = "1"*3, `121212` = "12"*3).
//! - For each ID, check all possible repeat lengths that evenly divide the
//!   digit count
//! - Check if the ID consists of that pattern repeated
//! - Sum all invalid IDs
//! - Complexity: O(R * D * sqrt(D)) for divisor checks

fn parse_input(input: &str) -> Vec<(u64, u64)> {
    input
        .trim()
        .split(',')
        .map(|range| {
            let parts: Vec<&str> = range.trim().split('-').collect();
            let start: u64 = parts[0].parse().unwrap();
            let end: u64 = parts[1].parse().unwrap();
            (start, end)
        })
        .collect()
}

/// Check if a number is invalid for Part 1:
/// digits form a sequence repeated exactly twice
fn is_invalid_part1(n: u64) -> bool {
    let s = n.to_string();
    let len = s.len();

    // Must have even length for exact two repetitions
    if !len.is_multiple_of(2) {
        return false;
    }

    let half = len / 2;
    s[..half] == s[half..]
}

/// Check if a number is invalid for Part 2:
/// digits form a sequence repeated 2 or more times
fn is_invalid_part2(n: u64) -> bool {
    let s = n.to_string();
    let len = s.len();

    // Try all possible pattern lengths that divide evenly
    // and result in 2+ repetitions
    for pattern_len in 1..=len / 2 {
        if !len.is_multiple_of(pattern_len) {
            continue;
        }

        let pattern = &s[..pattern_len];
        let repetitions = len / pattern_len;

        // Need at least 2 repetitions
        if repetitions >= 2 {
            let mut valid = true;
            for i in 1..repetitions {
                let start = i * pattern_len;
                if &s[start..start + pattern_len] != pattern {
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
    let mut sum = 0;

    for (start, end) in ranges {
        for n in start..=end {
            if is_invalid_part1(n) {
                sum += n;
            }
        }
    }
    sum
}

pub fn part_two(input: &str) -> u64 {
    let ranges = parse_input(input);
    let mut sum = 0;

    for (start, end) in ranges {
        for n in start..=end {
            if is_invalid_part2(n) {
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
