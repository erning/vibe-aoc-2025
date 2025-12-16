# Day 11 Solution

## ADDED Requirements

### Requirement: Puzzle Input Parsing
The system SHALL implement a `parse_input()` function that parses the Day 11 puzzle input according to the format specified in the puzzle description.

#### Scenario: Parse input correctly
- **WHEN** puzzle input is provided
- **THEN** parse it into appropriate data structures for solving

### Requirement: Part One Solution
The system SHALL implement a `part_one()` function that solves Part 1 of Day 11 according to the puzzle requirements.

#### Scenario: Example input produces expected answer
- **WHEN** example input from puzzle is provided
- **THEN** return the expected answer specified in the puzzle

#### Scenario: Real input produces correct answer
- **WHEN** real puzzle input is provided
- **THEN** return the answer accepted by Advent of Code

### Requirement: Part Two Solution
The system SHALL implement a `part_two()` function that solves Part 2 of Day 11 according to the puzzle requirements (available after completing Part 1).

#### Scenario: Example input produces expected answer
- **WHEN** example input from puzzle is provided
- **THEN** return the expected answer specified in the puzzle (if provided)

#### Scenario: Real input produces correct answer
- **WHEN** real puzzle input is provided
- **THEN** return the answer accepted by Advent of Code

### Requirement: Module Structure
The day11 module SHALL follow project conventions with `parse_input()`, `part_one()`, `part_two()` functions and embedded tests using `#[cfg(test)]`.

#### Scenario: Module follows conventions
- **WHEN** examining src/day11.rs
- **THEN** it contains parse_input, part_one, part_two functions and test module

### Requirement: Test Coverage
The day11 module SHALL include tests that validate solutions against example data from the puzzle description.

#### Scenario: Tests use example data
- **WHEN** running `cargo test day11`
- **THEN** tests pass using inputs/11-example.txt and expected answers
