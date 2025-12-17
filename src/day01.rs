//! Day 01: Secret Entrance (AoC 2025)
//!
//! ## Problem Description
//!
//! A safe with a circular dial (0-99) needs to be opened. The dial starts at 50.
//! We receive a sequence of rotations (L/R direction + distance) that move the dial.
//!
//! **Part 1**: Count how many times the dial points at 0 after completing each
//! rotation in the sequence.
//!
//! **Part 2**: Count how many times the dial points at 0 during the entire
//! rotation process, including intermediate positions.
//!
//! ## Solution Approach
//!
//! **Input Parsing**: Parse each line into a direction (L/R) and a distance value.
//!
//! **Part 1 Strategy**:
//! - Simulate dial rotations, tracking position modulo 100
//! - After each rotation, check if position is 0
//! - Count occurrences
//!
//! **Part 2 Strategy**:
//! - For each rotation, count all intermediate positions that pass through 0
//! - For a rotation from position P by distance D:
//!   - If moving left: count how many times we cross 0
//!   - If moving right: count how many times we cross 0
//! - Also check final position
//!
//! **Complexity**: O(n*d) where n is number of rotations and d is max distance.
//! With careful modular arithmetic, effectively O(n).

fn parse_input(input: &str) -> Vec<(char, i32)> {
    input
        .trim()
        .lines()
        .map(|line| {
            let (dir, dist) = line.split_at(1);
            (dir.chars().next().unwrap(), dist.parse().unwrap())
        })
        .collect()
}

pub fn part_one(input: &str) -> u32 {
    let rotations = parse_input(input);
    let mut position = 50;
    let mut count = 0;

    for (direction, distance) in rotations {
        position = match direction {
            'L' => (position - distance).rem_euclid(100),
            'R' => (position + distance).rem_euclid(100),
            _ => panic!("Invalid direction"),
        };

        if position == 0 {
            count += 1;
        }
    }

    count
}

pub fn part_two(input: &str) -> u32 {
    let rotations = parse_input(input);
    let mut position: i32 = 50;
    let mut count = 0;

    for (direction, distance) in rotations {
        for _ in 0..distance {
            position = match direction {
                'L' => (position - 1).rem_euclid(100),
                'R' => (position + 1).rem_euclid(100),
                _ => panic!("Invalid direction"),
            };

            if position == 0 {
                count += 1;
            }
        }
    }

    count
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::read_example;

    #[test]
    fn example() {
        let input = read_example(1);
        assert_eq!(part_one(&input), 3);
        assert_eq!(part_two(&input), 6);
    }
}
