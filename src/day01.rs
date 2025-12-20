//! Day 1: Secret Entrance
//!
//! ## Problem Description
//!
//! Part 1: The dial starts at position 50 (0-99 range). Count how many
//! times the dial points at 0 after any rotation completes. Rotations are
//! specified as L/R followed by a distance.
//!
//! Part 2: Same as Part 1, but also count times the dial passes through 0
//! during a rotation, not just at the end.
//!
//! ## Solution Approach
//!
//! **Input Parsing**: Parse each line as a direction (L/R) and distance.
//!
//! **Part 1 Strategy**:
//! - Start at position 50 on a 0-99 dial (circular)
//! - For each rotation, calculate new position: (pos + distance) mod 100
//!   or (pos - distance) mod 100 depending on direction
//! - Count how many times we land exactly on position 0 after a rotation
//!
//! **Part 2 Strategy**:
//! - For each rotation, count all positions we pass through, not just the end
//! - For a rotation from pos to new_pos, check all intermediate positions
//! - Count each time we encounter 0 (including intermediate passes)
//!
//! **Complexity**: O(n * m) where n is number of rotations and m is max
//! distance value. For typical inputs, this is efficient enough.

fn parse_input(input: &str) -> Vec<(char, u32)> {
    input
        .trim()
        .lines()
        .map(|line| {
            let direction = line.chars().next().unwrap();
            let distance: u32 = line[1..].parse().unwrap();
            (direction, distance)
        })
        .collect()
}

pub fn part_one(input: &str) -> u32 {
    let rotations = parse_input(input);
    let mut position: i32 = 50;
    let mut count = 0;

    for (direction, distance) in rotations {
        match direction {
            'L' => {
                position -= distance as i32;
            }
            'R' => {
                position += distance as i32;
            }
            _ => panic!("Invalid direction: {}", direction),
        }

        // Normalize to 0-99 range
        position = position.rem_euclid(100);

        // Count if we landed on 0
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
        let start = position;
        match direction {
            'L' => {
                position -= distance as i32;
            }
            'R' => {
                position += distance as i32;
            }
            _ => panic!("Invalid direction: {}", direction),
        }

        // Normalize to 0-99 range
        position = position.rem_euclid(100);

        // Count how many times we pass through 0 during the rotation
        // We need to check if 0 is on the path from start to position
        if direction == 'L' {
            // Moving left (decreasing), possibly wrapping from 0 to 99
            let mut current = start;
            for _ in 0..distance {
                current = if current == 0 { 99 } else { current - 1 };
                if current == 0 {
                    count += 1;
                }
            }
        } else {
            // Moving right (increasing), possibly wrapping from 99 to 0
            let mut current = start;
            for _ in 0..distance {
                current = (current + 1) % 100;
                if current == 0 {
                    count += 1;
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
