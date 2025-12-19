//! Day 00: Template Module
//!
//! ## Problem Description
//!
//! This is a template/reference module for Advent of Code solutions.
//! Part 1 and Part 2 are placeholder implementations.
//!
//! ## Solution Approach
//!
//! **Input Parsing**: Converts the multiline string input into a vector of integers.
//!
//! **Template Structure**: This module serves as a template for new day implementations:
//! - Private `parse_input()` function for input parsing (follow the established pattern)
//! - Public `part_one()` function returning the solution for part one
//! - Public `part_two()` function returning the solution for part two
//! - Tests using `read_example()` against provided examples
//!
//! **Complexity**: Varies depending on the specific day's requirements.
//! **Note**: This is a template/reference implementation from a previous year (AoC 2020).

fn parse_input(input: &str) -> Vec<i32> {
    input.trim().lines().map(|s| s.parse().unwrap()).collect()
}

pub fn part_one(input: &str) -> i32 {
    let numbers = parse_input(input);
    let n = numbers.len();
    for (i, a) in numbers.iter().take(n - 1).enumerate() {
        for b in numbers.iter().skip(i) {
            if a + b == 2020 {
                return a * b;
            }
        }
    }
    panic!()
}

pub fn part_two(input: &str) -> i32 {
    let numbers = parse_input(input);
    let n = numbers.len();
    for (i, a) in numbers.iter().enumerate().take(n - 2) {
        for (j, b) in numbers.iter().enumerate().take(n - 1).skip(i) {
            for c in numbers.iter().skip(j) {
                if a + b + c == 2020 {
                    return a * b * c;
                }
            }
        }
    }
    panic!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::read_example;

    #[test]
    fn example() {
        // This is a template module, so example tests are from AoC 2020 Day 1
        // Replace with appropriate tests for the actual day when implementing
        let input = read_example(0);
        assert_eq!(part_one(&input), 514579);
        assert_eq!(part_two(&input), 241861950);
    }
}
