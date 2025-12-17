# Tasks: Add Day 1 Solution

## 1. Setup Input Files
- [x] 1.1 Fetch puzzle description from https://adventofcode.com/2025/day/1
- [x] 1.2 Extract example data from puzzle and save to `inputs/01-example.txt`
- [x] 1.3 Download real input via `./scripts/download-input.sh 1` to `inputs/01-input.txt`

## 2. Implement Part One
- [x] 2.1 Create `src/day01.rs` with module documentation following project conventions
- [x] 2.2 Implement `parse_input()` function based on puzzle input format
- [x] 2.3 Implement `part_one()` function based on puzzle requirements
- [x] 2.4 Add test for Part 1 using example data and expected answer from puzzle
- [x] 2.5 Update `src/lib.rs` to declare `day01` module (if needed)
- [x] 2.6 Update `src/main.rs` to register day01 puzzle (if needed)
- [x] 2.7 Run `cargo test day01` to verify example passes
- [x] 2.8 Run with real input: `cargo run --release -- 1`
- [x] 2.9 Submit Part 1 answer via `./scripts/submit-answer.sh 1 1 <answer>` (if unknown)
- [x] 2.10 Verify answer is correct; modify implementation if needed
- [x] 2.11 Run `cargo clippy` and `cargo fmt` to ensure code quality
- [x] 2.12 Commit Part 1 implementation

## 3. Implement Part Two
- [x] 3.1 Fetch Part 2 description (available after Part 1 is completed)
- [x] 3.2 Implement `part_two()` function based on Part 2 requirements
- [x] 3.3 Add test for Part 2 using example data and expected answer (if provided)
- [x] 3.4 Run `cargo test day01` to verify tests pass
- [x] 3.5 Run with real input: `cargo run --release -- 1`
- [x] 3.6 Submit Part 2 answer via `./scripts/submit-answer.sh 1 2 <answer>` (if unknown)
- [x] 3.7 Verify answer is correct; modify implementation if needed
- [x] 3.8 Run `cargo clippy` and `cargo fmt` to ensure code quality
- [x] 3.9 Commit Part 2 implementation

## 4. Final Validation
- [x] 4.1 Run `cargo test` to verify all tests pass
- [x] 4.2 Run `cargo clippy` to verify no lint warnings
- [x] 4.3 Run `cargo fmt --check` to verify formatting