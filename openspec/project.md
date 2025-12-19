# Project Context

## Purpose
This is a **Rust implementation of Advent of Code 2025 puzzles**. The project provides a clean, modular architecture for solving daily programming challenges with consistent patterns, automated tooling for puzzle management, and comprehensive testing against provided examples.

**Goals:**
- Solve Advent of Code 2025 puzzles efficiently in Rust
- Maintain consistent code patterns across all days
- Enable easy testing against example inputs
- Provide automation for fetching puzzles and submitting answers
- Document algorithms and complexity analysis for each solution

## Tech Stack
- **Language**: Rust (2021 edition)
- **Build Tool**: Cargo
- **Package Name**: `aoc-2025-in-rust` (library: `aoc`, binary: `aoc`)
- **Code Formatter**: rustfmt (max_width = 78, configured in `.rustfmt.toml`)
- **Linter**: Clippy
- **Testing Framework**: Built-in Rust test framework (`#[cfg(test)]` modules)
- **Shell Scripts**: Bash for automation (`scripts/` directory)
  - `fetch-puzzle.sh` - Download puzzle descriptions as HTML
  - `download-input.sh` - Download puzzle inputs (requires auth)
  - `submit-answer.sh` - Submit answers to AoC
- **External API**: Advent of Code website (adventofcode.com)
- **External Dependencies**: None (standard library only)

## Project Conventions

### Code Style
- **Formatting**: `max_width = 78` enforced via `.rustfmt.toml`
- **Naming**: snake_case for functions and variables, PascalCase for modules
- **Documentation**: Every day module includes:
  - Problem description in markdown format
  - Detailed solution approach with algorithm explanation
  - Complexity analysis (Big O notation)
  - Optimization notes where applicable
- **Error Handling**: `panic!()` for unexpected conditions (puzzle guarantees valid input)
- **Imports**: Grouped by std, external crates, and internal modules

### Architecture Patterns
- **Modular Design**: Each day is a separate module (`src/dayNN.rs`)
- **Consistent Interface**: All day modules implement:
  - Private `parse_input()` - converts string input to structured data
  - Public `part_one()` - solves part 1 of the puzzle
  - Public `part_two()` - solves part 2 of the puzzle
- **Registry Pattern**: Main executable maintains a vector of puzzle solvers using `puzzle!` macro in `src/main.rs`
- **I/O Abstraction**: Helper functions in `src/lib.rs`:
  - `read_input(day)` - reads `inputs/NN-input.txt`
  - `read_example(day)` - reads `inputs/NN-example.txt`
  - `read_as_string(day, filename)` - reads custom input file
- **Flexibility**: Support for both example and real inputs via command-line flags
  - `--example` - use example inputs instead of real inputs
  - `--time` - display execution timing for each part
  - Positional args - run specific days (e.g., `cargo run --release -- 1 5 10`)

### Testing Strategy
- **Test Location**: Tests embedded in each day module with `#[cfg(test)]`
- **Test Data**: Uses example inputs from `inputs/XX-example.txt`
- **Validation**: Tests assert against known correct answers from puzzle examples
- **Execution**: Run all tests with `cargo test` or specific day with `cargo test day05`
- **Coverage**: Each day's solution must pass its example tests before submission

### Git Workflow
- **Default Branch**: `master`
- **Feature Branches**: Used for development (e.g., `openspec`)
- **Commit Style**: Clear, lowercase, descriptive commit messages
- **Examples**:
  - "Initial commit"
  - "add a script to fetch puzzle"
  - "introduce openspec (claude, factory)"
- **Hooks**: No pre-commit hooks configured

## Domain Context

**Advent of Code**: An annual event releasing daily programming puzzles from December 1-25. Each puzzle has:

For AoC 2025, only the first 12 days have puzzles (December 1-12). Each puzzle has:
- Two parts (Part 2 unlocks after completing Part 1)
- Example input with known answers for validation
- Unique puzzle input per user (requires authentication)
- Text-based problem descriptions with variable complexity

**Puzzle Structure:**
- Inputs are text files (numbers, strings, grids, etc.)
- Solutions typically involve parsing, algorithms, and data structures
- Part 2 often extends Part 1 with additional complexity
- Performance matters for some puzzles (explicit complexity constraints)

**Workflow:**
1. Create new day module following template
2. Parse input according to puzzle specification
3. Implement solution for Part 1
4. Test against example input
5. Implement solution for Part 2
6. Run with real input and submit answer
7. Document approach and complexity

## Important Constraints
- **Authentication Required**: Session token needed to download puzzle inputs and submit answers
- **Rate Limiting**: Advent of Code API has rate limits (max 100 submissions per puzzle)
- **Input Handling**: All inputs guaranteed valid (no extensive error handling needed)
- **Performance**: Some puzzles have explicit Big O requirements
- **Timing**: Puzzles release at midnight EST daily in December
- **Part 2 Unlock**: Must complete Part 1 before accessing Part 2 description

## External Dependencies
- **Advent of Code API**:
  - Base URL: `https://adventofcode.com/2025/day/{day}`
  - Puzzle descriptions: HTML format (saved to `inputs/NN-puzzle.html`)
  - Input download: Text format (requires session cookie, saved to `inputs/NN-input.txt`)
  - Answer submission: Form POST with validation response
- **Session Management** (required for input download and submission):
  - `AOC_SESSION` environment variable, OR
  - `.aoc-session` file in project directory, OR
  - `~/.aoc-session` file in home directory
  - Obtained from browser cookies after GitHub authentication
- **Rust Crates**: None - project uses only Rust standard library
