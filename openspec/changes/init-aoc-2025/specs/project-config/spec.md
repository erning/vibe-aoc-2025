# Project Configuration

## ADDED Requirements

### Requirement: AoC 2025 Year Configuration
The project SHALL be configured for Advent of Code 2025 with the event URL `https://adventofcode.com/2025`.

#### Scenario: Package name reflects year 2025
- **WHEN** the project is built
- **THEN** the package name is `aoc-2025-in-rust`

#### Scenario: Scripts use year 2025
- **WHEN** download or submit scripts are executed
- **THEN** they target the 2025 event year

### Requirement: 12-Day Puzzle Structure
The project SHALL support puzzles for Day 1 through Day 12 only (AoC 2025 has 12 puzzles).

#### Scenario: Valid day range
- **WHEN** running puzzles
- **THEN** days 1-12 are valid puzzle days

### Requirement: Day00 Template Reference
The project SHALL include a `day00` module as a template/reference for creating new day solutions, containing example patterns for `parse_input()`, `part_one()`, `part_two()`, and tests.

#### Scenario: Day00 serves as template
- **WHEN** creating a new day module
- **THEN** day00.rs can be copied and modified as a starting point

#### Scenario: Day00 is not registered as active puzzle
- **WHEN** running all puzzles without arguments
- **THEN** day00 is not executed (it is commented out in main.rs)
