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
- **Code Formatter**: rustfmt (max_width = 78)
- **Linter**: Clippy
- **Testing Framework**: Built-in Rust test framework
- **Shell Scripts**: Bash for automation (fetching puzzles, downloading inputs, submitting answers)
- **External API**: Advent of Code website (adventofcode.com)

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
- **Registry Pattern**: Main executable maintains a vector of puzzle solvers in `src/main.rs`
- **I/O Abstraction**: Helper functions in `src/lib.rs` for reading inputs
- **Flexibility**: Support for both example and real inputs via command-line flags
- **Timing**: Optional execution timing with `--time` flag

### Testing Strategy
- **Test Location**: Tests embedded in each day module with `#[cfg(test)]`
- **Test Data**: Uses example inputs from `inputs/XX-example.txt`
- **Validation**: Tests assert against known correct answers from puzzle examples
- **Execution**: Run all tests with `cargo test` or specific day with `cargo test day05`
- **Coverage**: Each day's solution must pass its example tests before submission

### Git Workflow
- **Branching**: Direct commits to main branch (simple workflow for personal project)
- **Commit Style**: Clear, descriptive commit messages
- **Examples**:
  - "Initial commit"
  - "add a script to fetch puzzle"
- **Hooks**: No pre-commit hooks configured
- **PRs**: Main branch used for pull requests

## Domain Context

**Advent of Code**: An annual event releasing daily programming puzzles from December 1-25. Each puzzle has:
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
  - Puzzle descriptions: HTML format
  - Input download: Text format (requires session cookie)
  - Answer submission: Form POST with validation response
- **Session Management**:
  - Token can be set via `AOC_SESSION` environment variable
  - Or stored in `.aoc-session` file (project or home directory)
  - Obtained from browser cookies after GitHub authentication
- **No External Libraries**: Project uses only Rust standard library (no external crates)
