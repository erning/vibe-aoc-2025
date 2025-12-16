//! Day 1: Secret Entrance
//!
//! A safe dial goes from 0-99 in a circle, starting at 50.
//! Input is a sequence of rotations: L (left/lower) or R (right/higher)
//! followed by a distance.
//!
//! ## Solution Approach
//!
//! **Input Parsing**: Each line is parsed into a direction (L/R) and distance.
//!
//! **Part 1 Strategy**: Simulate rotations using modular arithmetic.
//! - Track current position, apply each rotation
//! - Count how many times the dial ends at 0 after a rotation
//! - Complexity: O(n) where n is number of rotations
//!
//! **Part 2 Strategy**: Count all times dial passes through or ends at 0.
//! - For each rotation, calculate how many times 0 is crossed
//! - Going left from pos with distance d: crosses 0 if (pos + 100 - d..=pos)
//!   wraps around 0
//! - Going right from pos with distance d: crosses 0 if (pos..pos + d) wraps
//!   around 100
//! - Complexity: O(n) where n is number of rotations

struct Rotation {
    direction: char,
    distance: i32,
}

fn parse_input(input: &str) -> Vec<Rotation> {
    input
        .trim()
        .lines()
        .map(|line| {
            let direction = line.chars().next().unwrap();
            let distance: i32 = line[1..].parse().unwrap();
            Rotation {
                direction,
                distance,
            }
        })
        .collect()
}

pub fn part_one(input: &str) -> i32 {
    let rotations = parse_input(input);
    let mut position: i32 = 50;
    let mut count = 0;

    for r in rotations {
        match r.direction {
            'L' => position = (position - r.distance).rem_euclid(100),
            'R' => position = (position + r.distance).rem_euclid(100),
            _ => panic!("Unknown direction"),
        }
        if position == 0 {
            count += 1;
        }
    }
    count
}

pub fn part_two(input: &str) -> i32 {
    let rotations = parse_input(input);
    let mut position: i32 = 50;
    let mut count = 0;

    for r in rotations {
        // Count how many times we hit 0 during this rotation (not counting
        // start position)
        let crosses = match r.direction {
            'L' => {
                // Going left (decreasing), we hit 0 at steps: pos, pos+100,
                // pos+200, etc. But if pos=0, we start there (don't count),
                // so first hit is at step 100.
                if position > 0 {
                    if r.distance >= position {
                        (r.distance - position) / 100 + 1
                    } else {
                        0
                    }
                } else {
                    r.distance / 100
                }
            }
            'R' => {
                // Going right (increasing), we hit 0 at steps: 100-pos,
                // 200-pos, etc. This is equivalent to floor((pos+dist)/100)
                // since we start at pos < 100.
                (position + r.distance) / 100
            }
            _ => panic!("Unknown direction"),
        };

        count += crosses;

        // Update position
        match r.direction {
            'L' => position = (position - r.distance).rem_euclid(100),
            'R' => position = (position + r.distance).rem_euclid(100),
            _ => panic!("Unknown direction"),
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
