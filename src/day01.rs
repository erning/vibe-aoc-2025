//! Day 1: Secret Entrance
//!
//! ## Puzzle Overview
//!
//! The Elves need to access the North Pole by opening a safe. The safe has a dial with
//! numbers 0-99. The dial starts at position 50. A sequence of rotations tells us how
//! to move the dial - each rotation is either "L" (left toward lower numbers) or "R"
//! (right toward higher numbers) followed by a distance.
//!
//! The dial is circular, so rotating left from 0 goes to 99, and rotating right from
//! 99 goes to 0.
//!
//! ## Part 1 Strategy
//!
//! We need to count how many times the dial points at 0 **after each rotation completes**.
//! This is straightforward: simulate each rotation and check if the final position is 0.
//!
//! **Algorithm**:
//! 1. Parse each rotation into direction and distance
//! 2. Track the current position (starting at 50)
//! 3. For each rotation:
//!    - Update position based on direction and distance (with wrap-around)
//!    - Check if position is 0 and increment counter
//! 4. Return the counter
//!
//! **Complexity**: O(n) where n is the number of rotations
//!
//! ## Part 2 Strategy
//!
//! We need to count how many times the dial points at 0 **at any point during a rotation**,
//! not just at the end. This includes intermediate positions when crossing 0.
//!
//! **Algorithm**:
//! 1. Parse each rotation into direction and distance
//! 2. Track the current position (starting at 50)
//! 3. For each rotation:
//!    - Calculate how many times we cross 0 during this rotation
//!    - Update position based on direction and distance (with wrap-around)
//!    - Check if final position is 0 and increment counter
//! 4. Return the total counter
//!
//! **Complexity**: O(n) where n is the number of rotations
//!
//! The key insight for Part 2 is that rotating by distance d crosses 0 exactly
//! `distance / 100` times (integer division), plus an additional time if we wrap around.

#[derive(Debug, Clone, Copy)]
enum Direction {
    Left,
    Right,
}

fn parse_input(input: &str) -> Vec<(Direction, usize)> {
    input
        .trim()
        .lines()
        .map(|line| {
            let direction = match line.chars().next().unwrap() {
                'L' => Direction::Left,
                'R' => Direction::Right,
                _ => panic!("Invalid direction: {}", line),
            };
            let distance = line[1..].parse().unwrap();
            (direction, distance)
        })
        .collect()
}

pub fn part_one(input: &str) -> u32 {
    let rotations = parse_input(input);
    let mut position = 50;
    let mut zero_count = 0;

    for (direction, distance) in rotations {
        match direction {
            Direction::Left => {
                position = (position + 100 - distance % 100) % 100;
            }
            Direction::Right => {
                position = (position + distance) % 100;
            }
        }
        if position == 0 {
            zero_count += 1;
        }
    }

    zero_count
}

pub fn part_two(input: &str) -> u32 {
    let rotations = parse_input(input);
    let mut position = 50;
    let mut zero_count = 0;

    for (direction, distance) in rotations {
        let full_cycles = distance / 100;
        let remainder = distance % 100;

        // Add zeros from full cycles
        zero_count += full_cycles as u32;

        // Count zeros from the partial rotation
        // Check if starting position is 0
        if position == 0 {
            zero_count += 1;
        }

        // Move the partial distance
        let end_pos = match direction {
            Direction::Left => (position + 100 - remainder) % 100,
            Direction::Right => (position + remainder) % 100,
        };

        // Check if ending position is 0
        if end_pos == 0 {
            zero_count += 1;
        }

        // Update position for next rotation
        position = end_pos;
    }

    zero_count
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
