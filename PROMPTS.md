# Role
You are an expert Rust developer and automation engineer tasked with setting up and solving the "Advent of Code 2025" (AoC 2025) using a specific project template.

# Context
- **Event URL:** https://adventofcode.com/2025
- **Scope:** Days 1 through 12.
- **Tools:** Use the scripts located in the `scripts/` directory for automation.

# Workflow Instructions

Please execute the following steps in order. Stop and ask for clarification only if a critical error occurs that you cannot resolve.

## Phase 1: Setup & Initialization
1.  **Read Documentation:** Read `AGENTS.md` to understand the project structure and available tools.
2.  **Update Configuration:** Find occurrences of the placeholder `xxxx` in the project files and replace them with `2025`.
3.  **Git Configuration:** Ensure `.aoc-session` is added to `.gitignore`. If it is missing, add it immediately.
4.  **Reset Day 1:** The current Day 1 files (`src/day01.rs`, `inputs/01-example.txt`, `inputs/01-inputs.txt`) are placeholders. Please use the scripts in `scripts/` to re-download the problem description and generate the correct files for Day 1.
5. You should include `.aoc-session` in the request cookie to fetch problem description of part two.

## Phase 2: Solving Loop (Day 1 to Day 12)
Iterate through Day 1 to Day 12 sequentially. For each day, perform the following:

1.  **Fetch Data:**
    - Read the problem description from the website using the `scripts/`.
    - Extract/Download `inputs/NN-example.txt` and `inputs/NN-input.txt`.
2.  **Implement Part One:**
    - Write the solution in `src/dayNN.rs`.
    - **Verification:** Verify your logic against the provided example data.
    - **Git Commit:** Commit the changes with message `feat: solve day NN part one`. Including example/input files.
3.  **Implement Part Two:**
    - Write the solution for Part Two in the same file.
    - **Verification:** Verify your logic against the provided example data.
    - **Git Commit:** Commit the changes with message `feat: solve day NN part two`. Including example/input files.
4.  **Proceed:** Move to the next day immediately after the second commit.

# Constraints & Guidelines
- **Git Commits:** strictly **DO NOT** include "Co-Author: Claude Code" or any similar attribution in the commit messages. Keep the history clean.
- **Submission:** Do not perform any HTTP POST requests to submit answers to the AoC server. The answers have been previously solved; your goal is to implement the code and verify it passes the local tests/examples correctly.
- **Code Style:** Write clean, idiomatic Rust code.

