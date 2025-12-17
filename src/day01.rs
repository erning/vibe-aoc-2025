//! Day 1: Secret Entrance
//!
//! ## Problem Description
//!
//! Part 1: The dial starts at 50. Count how many times the dial points at 0 after any rotation in the sequence.
//! Part 2: Count the number of times any click causes the dial to point at 0, regardless of whether it happens during a rotation or at the end of one.
//!
//! ## Solution Approach
//!
//! **Input Parsing**: Converts each line to a rotation command (direction and distance).
//!
//! **Part 1 Strategy**:
//! - Start at position 50
//! - Process each rotation command
//! - Track final position after each rotation
//! - Count how many times position equals 0 after a rotation
//!
//! **Part 2 Strategy**:
//! - Same as Part 1, but also count intermediate positions during rotations
//! - For each rotation, check every position passed through
//! - Count total occurrences of position 0
//!
//! **Complexity**: O(n * m) where n is number of rotations and m is average rotation distance.

#[derive(Debug, Clone, Copy)]
struct Rotation {
    direction: char, // 'L' or 'R'
    distance: i32,
}

fn parse_input(input: &str) -> Vec<Rotation> {
    input
        .trim()
        .lines()
        .map(|line| {
            let line = line.trim();
            let direction = line.chars().next().unwrap();
            let distance = line[1..].parse::<i32>().unwrap();
            Rotation {
                direction,
                distance,
            }
        })
        .collect()
}

pub fn part_one(input: &str) -> i32 {
    let rotations = parse_input(input);
    let mut position = 50; // Start at position 50
    let mut zero_count = 0;

    for rotation in rotations {
        // Apply the rotation
        if rotation.direction == 'L' {
            position = (position - rotation.distance).rem_euclid(100);
        } else {
            // 'R'
            position = (position + rotation.distance).rem_euclid(100);
        }

        // Count if we're at 0 after this rotation
        if position == 0 {
            zero_count += 1;
        }
    }

    zero_count
}

pub fn part_two(input: &str) -> i32 {
    let rotations = parse_input(input);
    let mut position: i32 = 50; // Start at position 50
    let mut zero_count = 0;

    for rotation in rotations {
        // For each rotation, check every position we pass through
        for _step in 1..=rotation.distance {
            if rotation.direction == 'L' {
                position = (position - 1).rem_euclid(100);
            } else {
                // 'R'
                position = (position + 1).rem_euclid(100);
            }

            // Count if we're at 0 during this step
            if position == 0 {
                zero_count += 1;
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
        assert_eq!(part_one(&input), 3);
        assert_eq!(part_two(&input), 6);
    }
}
