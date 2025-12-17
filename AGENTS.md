<!-- OPENSPEC:START -->
# OpenSpec Instructions

These instructions are for AI assistants working in this project.

Always open `@/openspec/AGENTS.md` when the request:
- Mentions planning or proposals (words like proposal, spec, change, plan)
- Introduces new capabilities, breaking changes, architecture shifts, or big performance/security work
- Sounds ambiguous and you need the authoritative spec before coding

Use `@/openspec/AGENTS.md` to learn:
- How to create and apply change proposals
- Spec format and conventions
- Project structure and guidelines

Keep this managed block so 'openspec update' can refresh the instructions.

<!-- OPENSPEC:END -->

# AGENTS.md

## Project Overview


This is a template project for [Advent of Code 2025](https://adventofcode.com/2025/) implementation in Rust, featuring solutions with consistent architecture and testing patterns.
Note: This project is configured for a 12-day event structure (Day 1-12).

- Puzzle page: `https://adventofcode.com/2025/day/${N}`
- Input download: `https://adventofcode.com/2025/day/${N}/input` (requires authentication)

## Architecture
- **Entry Point**: `src/main.rs` - Main executable with option to run all days or specific ones
- **Library**: `src/lib.rs` - Re-exports day modules and provides I/O utilities
- **Day Modules**: `src/day{NN}.rs` - Individual solutions following standardized patterns
- **Inputs**: `inputs/` - Puzzle inputs organized by day
  - `{NN}-example.txt` - Example input from puzzle description
  - `{NN}-input.txt` - Actual puzzle input (requires authentication to download)

## Core Patterns
Each day module implements:
- Private `parse_input()` function for input parsing
- Public `part_one()` and `part_two()` functions returning results
- Tests using `read_example()` against provided examples

## Commands

### Build & Run
```bash
cargo build --release
cargo run --release --           # Run all days
cargo run --release -- 1 5 10    # Run specific days
cargo run --release -- --example # Use example inputs
```

### Testing
```bash
cargo test                           # All tests
cargo test day05 -- --nocapture      # Specific day tests
```

### Development
```bash
cargo check    # Quick syntax/type validation
cargo clippy   # Linting suggestions
cargo fmt      # Code formatting
```

### Input Utilities
- `aoc::read_input(day)` - Real puzzle input
- `aoc::read_example(day)` - Example input
- `aoc::read_as_string(day, filename)` - Custom input file

## Scripts

### Set up AoC session

choose one method:
```bash
# Set up authentication (choose one method):
# Method 1: Environment variable
export AOC_SESSION="your_session_cookie_here"

# Method 2: Local file
echo "your_session_cookie_here" > .aoc-session

# Method 3: Global file
echo "your_session_cookie_here" > ~/.aoc-session
```

To get your session cookie:
1. Login to https://adventofcode.com via GitHub
2. Open browser DevTools (F12) → Application → Cookies
3. Copy the value of 'session' cookie

### Fetch Puzzle Description

```bash
# Fetch puzzle description for a specific day
./scripts/fetch-puzzle.sh 5

# Fetch today's puzzle
./scripts/fetch-puzzle.sh

# Saves to inputs/05-puzzle.html
# Note: Part 2 description is only available after completing Part 1
```

### Download Puzzle Input
```bash
# Download input for a specific day (requires authentication)
./scripts/download-input.sh 5

# Download today's input
./scripts/download-input.sh
```

### Submit Answer
```bash
# Submit answer for a specific day and part
./scripts/submit-answer.sh 5 1 12345

# The script will indicate if the answer is correct or not
# and provide any available hints (too high/low, rate limits, etc.)
```

## Extension Workflow
1. Create `src/dayXX.rs` following established patterns
2. Add module declaration to `src/lib.rs`
3. Register in `src/main.rs` puzzles array
4. Add corresponding input and test files

## Quality Standards
- All code passes `cargo clippy` checks
- Follows `.rustfmt.toml` formatting rules
- Includes algorithm explanation comments
- Tests validate against provided examples
