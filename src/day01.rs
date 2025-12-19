//! Day 01: Secret Entrance
//!
//! ## Problem Description
//!
//! The Elves need to finish decorating the North Pole by December 12th.
//! At the secret entrance, there's a safe with a dial numbered 0-99.
//! The dial starts at 50, and rotations (L for left, R for right) move it by a certain distance.
//! The dial wraps around, so going past 99 goes to 0, and going before 0 goes to 99.
//!
//! **Part 1**: Count how many times the dial ends up pointing at 0 after each rotation.
//!
//! **Part 2**: Count how many times the dial points at 0 during or after each rotation (including intermediate clicks).
//!
//! ## Solution Approach
//!
//! **Input Parsing**: Parse each line as a rotation (direction + distance).
//! For Part 1: Follow rotations and count how many times we end on 0.
//! For Part 2: Follow rotations and count how many times the dial passes over 0 during movement.
//!
//! **Complexity**: O(N*M) where N is number of rotations and M is average rotation distance for Part 2,
//! and O(N) for Part 1 where N is the number of rotations.
//!
//! ## Algorithm Explanation
//!
//! **Part 1**: Simply follow each rotation starting from position 50, and count final positions at 0.
//!
//! **Part 2**: For each rotation, simulate every click and count when we pass over 0.

use std::str::FromStr;

#[derive(Debug, Clone, Copy)]
enum Direction {
    Left,
    Right,
}

#[derive(Debug)]
struct Rotation {
    direction: Direction,
    distance: usize,
}

impl FromStr for Rotation {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (dir_char, dist_str) = s.split_at(1);
        let direction = match dir_char.chars().next().unwrap() {
            'L' => Direction::Left,
            'R' => Direction::Right,
            _ => return Err(()),
        };
        let distance = dist_str.parse::<usize>().unwrap();

        Ok(Rotation {
            direction,
            distance,
        })
    }
}

fn parse_input(input: &str) -> Vec<Rotation> {
    input
        .lines()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty())
        .map(|line| line.parse::<Rotation>().unwrap())
        .collect()
}

pub fn part_one(input: &str) -> usize {
    let rotations = parse_input(input);
    let mut position = 50; // Starting position
    let mut count = 0;

    for rotation in rotations {
        match rotation.direction {
            Direction::Left => {
                // Moving left (counter-clockwise), decrease position with wrapping
                position = (position + 100 - rotation.distance % 100) % 100;
            }
            Direction::Right => {
                // Moving right (clockwise), increase position with wrapping
                position = (position + rotation.distance) % 100;
            }
        }

        // Check if we're at position 0 after this rotation
        if position == 0 {
            count += 1;
        }
    }

    count
}

pub fn part_two(input: &str) -> usize {
    let rotations = parse_input(input);
    let mut position = 50; // Starting position
    let mut count = 0;

    for rotation in rotations {
        match rotation.direction {
            Direction::Left => {
                // Moving left - simulate each step to count 0 crossings
                for _ in 0..rotation.distance {
                    // Move one step left
                    position = (position + 99) % 100; // equivalent to (position - 1 + 100) % 100

                    // Check if we're at position 0 after this step
                    if position == 0 {
                        count += 1;
                    }
                }
            }
            Direction::Right => {
                // Moving right - simulate each step to count 0 crossings
                for _ in 0..rotation.distance {
                    // Move one step right
                    position = (position + 1) % 100;

                    // Check if we're at position 0 after this step
                    if position == 0 {
                        count += 1;
                    }
                }
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
    fn example_part_one() {
        let input = read_example(1);
        assert_eq!(part_one(&input), 3);
    }

    #[test]
    fn example_part_two() {
        let input = read_example(1);
        assert_eq!(part_two(&input), 6);
    }
}
