//! Day 0: Template/Reference Module
//!
//! ## Template for New Day Solutions
//!
//! This module serves as a reference template for creating new day solutions.
//! Copy this file to `src/dayXX.rs` and modify the implementation for your specific puzzle.
//!
//! ## Template Structure
//!
//! **Input Parsing**: Converts the input string into appropriate data structures.
//!
//! **Part 1 Strategy**: Implement your algorithm here.
//! - Document your approach clearly
//! - Include complexity analysis
//! - Add optimization notes if applicable
//!
//! **Part 2 Strategy**: Implement the second part of the puzzle.
//! - Often extends Part 1 with additional complexity
//! - May require different data structures or algorithms
//!
//! **Testing**: Each solution should include tests using example inputs.
//! - Example input from puzzle description
//! - Known correct answers for validation
//!
//! ## Usage
//! 1. Copy this file: `cp src/day00.rs src/dayXX.rs`
//! 2. Update module name and implement solution
//! 3. Add module to `src/lib.rs`: `pub mod dayXX;`
//! 4. Register in `src/main.rs` puzzles array
//! 5. Create input files: `inputs/XX-example.txt` and `inputs/XX-input.txt`

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
        let input = read_example(0);
        assert_eq!(part_one(&input), 514579);
        assert_eq!(part_two(&input), 241861950);
    }
}
