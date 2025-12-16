# Change: Initialize Advent of Code 2025

## Why
The project is currently a generic template with placeholder year (`xxxx`) and contains day01 as a template example from a previous year (AoC 2020). We need to initialize it for AoC 2025 which has only 12 puzzles (Day 1-12) and rename day01 files to serve as reference templates.

## What Changes
- Replace all `xxxx` placeholders with `2025` across the codebase
- Update URLs to point to `https://adventofcode.com/2025`
- Rename `day01` module and related files to `day00` to serve as template reference
- Rename input files `01-example.txt` and `01-input.txt` to `00-example.txt` and `00-input.txt`
- Update documentation to reflect 12-day puzzle structure (Day 1-12 only)
- Update day00 module comments to indicate it's a template/reference

## Impact
- Affected files:
  - `Cargo.toml` - package name
  - `Cargo.lock` - package name
  - `README.md` - title
  - `AGENTS.md` - URLs and documentation
  - `flake.nix` - description
  - `openspec/project.md` - package name reference
  - `scripts/download-input.sh` - YEAR variable
  - `scripts/submit-answer.sh` - YEAR variable
  - `src/lib.rs` - module declaration
  - `src/main.rs` - puzzle registration
  - `src/day01.rs` → `src/day00.rs` - template module
  - `inputs/01-example.txt` → `inputs/00-example.txt`
  - `inputs/01-input.txt` → `inputs/00-input.txt`
