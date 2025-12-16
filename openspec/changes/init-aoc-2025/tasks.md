# Tasks: Initialize Advent of Code 2025

## 1. Update Year Placeholders
- [ ] 1.1 Update `Cargo.toml` package name from `aoc-xxxx-in-rust` to `aoc-2025-in-rust`
- [ ] 1.2 Update `README.md` title
- [ ] 1.3 Update `AGENTS.md` URLs and references
- [ ] 1.4 Update `flake.nix` description
- [ ] 1.5 Update `openspec/project.md` package name reference

## 2. Update Scripts
- [ ] 2.1 Update `scripts/download-input.sh` YEAR variable to 2025
- [ ] 2.2 Update `scripts/submit-answer.sh` YEAR variable to 2025

## 3. Rename Day01 to Day00 (Template Reference)
- [ ] 3.1 Rename `src/day01.rs` to `src/day00.rs`
- [ ] 3.2 Update `src/day00.rs` comments to indicate it's a template/reference
- [ ] 3.3 Update `src/lib.rs` module declaration from `day01` to `day00`
- [ ] 3.4 Update `src/main.rs` puzzle registration (comment out day00)
- [ ] 3.5 Rename `inputs/01-example.txt` to `inputs/00-example.txt`
- [ ] 3.6 Rename `inputs/01-input.txt` to `inputs/00-input.txt`

## 4. Update Documentation
- [ ] 4.1 Update `AGENTS.md` to reflect 12-day puzzle structure (Day 1-12)
- [ ] 4.2 Regenerate `Cargo.lock` with new package name

## 5. Validation
- [ ] 5.1 Run `cargo check` to verify compilation
- [ ] 5.2 Run `cargo test` to verify tests pass
- [ ] 5.3 Run `cargo clippy` to verify no lint warnings
