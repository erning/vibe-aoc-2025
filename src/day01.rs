//! Day 1: Secret Entrance
//!
//! ## Problem Description
//!
//! Part 1: Track a dial that rotates left and right. The dial starts at 50 and has numbers 0-99.
//! Each rotation is given as L/R followed by a distance. We need to count how many times the dial
//! points at 0 after completing each rotation.
//!
//! Part 2: Count all times the dial points at 0, including during rotations (not just at the end).
//!
//! ## Solution Approach
//!
//! **Input Parsing**: Parse each line as a direction (L or R) and a distance value.
//!
//! **Part 1**: Simulate the dial, tracking the current position. After each rotation, check if
//! the position is 0 and increment a counter. The dial wraps around using modulo 100 arithmetic.
//!
//! **Part 2**: For each rotation, calculate how many times the dial passes through 0 during the
//! rotation itself. This happens when the rotation distance crosses the 0 boundary.
//!
//! ## Complexity Analysis
//!
//! - Time Complexity: O(N) where N is the number of rotations
//! - Space Complexity: O(1) - constant space for tracking position and count

/// Rotation instruction with direction and distance
#[derive(Debug, Clone, Copy)]
pub struct Rotation {
    direction: Direction,
    distance: u32,
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    Left,
    Right,
}

/// Parse the input string into a vector of rotations
pub fn parse_input(input: &str) -> Vec<Rotation> {
    input
        .lines()
        .filter_map(|line| {
            if line.trim().is_empty() {
                return None;
            }

            let Some(first) = line.chars().next() else {
                return None;
            };

            let direction = match first {
                'L' => Direction::Left,
                'R' => Direction::Right,
                _ => return None,
            };

            let Ok(distance) = line[1..].trim().parse::<u32>() else {
                return None;
            };

            Some(Rotation {
                direction,
                distance,
            })
        })
        .collect()
}

/// Part 1: Count times dial points at 0 after each rotation
pub fn part_one(input: &str) -> impl std::fmt::Display {
    let rotations = parse_input(input);
    let mut position = 50; // Dial starts at 50
    let mut zero_count = 0;

    for rotation in rotations {
        match rotation.direction {
            Direction::Left => {
                // Moving left means decreasing position
                position = (position as i32 - rotation.distance as i32)
                    .rem_euclid(100) as u32;
            }
            Direction::Right => {
                // Moving right means increasing position
                position = (position + rotation.distance) % 100;
            }
        }

        if position == 0 {
            zero_count += 1;
        }
    }

    zero_count
}

/// Part 2: Count all times dial points at 0, including during rotations
pub fn part_two(input: &str) -> impl std::fmt::Display {
    let rotations = parse_input(input);
    let mut position = 50; // Dial starts at 50
    let mut zero_count = 0;

    for rotation in rotations {
        match rotation.direction {
            Direction::Left => {
                for _ in 0..rotation.distance {
                    position = if position == 0 { 99 } else { position - 1 };
                    if position == 0 {
                        zero_count += 1;
                    }
                }
            }
            Direction::Right => {
                for _ in 0..rotation.distance {
                    position = (position + 1) % 100;
                    if position == 0 {
                        zero_count += 1;
                    }
                }
            }
        }
    }

    zero_count
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::read_example;

    #[test]
    fn example() {
        let input = read_example(1);
        assert_eq!(part_one(&input).to_string(), "3");
        assert_eq!(part_two(&input).to_string(), "6");
    }
}
