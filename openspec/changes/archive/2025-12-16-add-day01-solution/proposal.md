# Change: Add Day 1 Solution

## Why
Day 1 of Advent of Code 2025 needs to be implemented. The existing day01 files are templates from a previous year and should be replaced with the actual 2025 solution.

## What Changes
- Fetch puzzle description from https://adventofcode.com/2025/day/1
- Create `inputs/01-example.txt` with example data extracted from puzzle
- Create `inputs/01-input.txt` with real puzzle input (downloaded)
- Create `src/day01.rs` with the Day 1 solution (replacing template)
- Update `src/lib.rs` module declaration if needed
- Update `src/main.rs` puzzle registration if needed
- Implement Part 1 and Part 2 separately with individual commits

## Impact
- Affected files:
  - `src/day01.rs` - solution implementation
  - `src/lib.rs` - module declaration
  - `src/main.rs` - puzzle registration
  - `inputs/01-example.txt` - example data
  - `inputs/01-input.txt` - real input
