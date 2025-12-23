//! Day 1: Secret Entrance
//!
//! ## Problem Description
//!
//! Part 1: Count how many times the dial points at 0 after completing each rotation.
//! Part 2: Count how many times the dial points at 0 during OR after each rotation.
//!
//! ## Solution Approach
//!
//! **Input Parsing**: Each line contains a direction (L or R) and a distance value.
//! Parse into a list of (direction, distance) pairs where direction is -1 for L and +1 for R.
//!
//! **Part 1 Strategy**: Simulate the dial rotation step by step:
//! - Start with dial at position 50
//! - For each rotation, calculate new position: `(current + direction * distance) % 100`
//! - Use modulo arithmetic that handles negative numbers correctly
//! - After each rotation, check if position == 0 and count it
//!
//! **Part 2 Strategy**: Extended simulation to count zero crossings during rotation:
//! - For each rotation, iterate through each individual click
//! - After each click, check if position == 0 and count it
//! - This counts zeros that occur mid-rotation, not just at the end
//!
//! **Complexity**: O(n) for part 1 where n is number of rotations.
//! O(n * d) for part 2 where d is average distance per rotation.
//! Since positions are only 0-99, we can optimize part 2 with math but iteration
//! is straightforward and fast enough for given inputs.
//!
//! **Optimization Note**: For part 2, instead of iterating through each click,
//! we could compute zero crossings directly using division, but the simple
//! approach is readable and efficient for the input sizes involved.

#[derive(Clone, Copy)]
enum Direction {
    Left = -1,
    Right = 1,
}

struct Rotation {
    direction: Direction,
    distance: u32,
}

fn parse_input(input: &str) -> Vec<Rotation> {
    input
        .trim()
        .lines()
        .map(|line| {
            let dir = if line.starts_with('L') {
                Direction::Left
            } else {
                Direction::Right
            };
            let dist = line[1..].parse().unwrap();
            Rotation {
                direction: dir,
                distance: dist,
            }
        })
        .collect()
}

pub fn part_one(input: &str) -> u32 {
    let rotations = parse_input(input);
    let mut position: i32 = 50;
    let mut zero_count = 0;

    for rot in rotations {
        let delta = (rot.direction as i32) * (rot.distance as i32);
        position = ((position + delta) % 100 + 100) % 100;
        if position == 0 {
            zero_count += 1;
        }
    }

    zero_count
}

pub fn part_two(input: &str) -> u32 {
    let rotations = parse_input(input);
    let mut position: i32 = 50;
    let mut zero_count = 0;

    for rot in rotations {
        for _ in 0..rot.distance {
            let delta = rot.direction as i32;
            position = ((position + delta) % 100 + 100) % 100;
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
