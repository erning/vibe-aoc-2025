# Advent of Code 2025 - Multi-Branch Analysis

This repository contains multiple branches exploring different AI models solving Advent of Code 2025 puzzles. Each branch represents a different AI model's approach to solving the daily challenges.

## Branch Comparison Summary

| Branch | Days Completed | Total Commits | Approach | Status |
|--------|----------------|---------------|----------|--------|
| **[droid-opus-4.5](../../tree/droid-opus-4.5)** | 1-12 (ALL) | 14 | Direct implementation | ✅ Complete |
| **[openspec-opus-4.5](../../tree/openspec-opus-4.5)** | 1-12 (ALL) | 17 | OpenSpec framework (archived) | ✅ Complete |
| **[openspec-glm-4.7](../../tree/openspec-glm-4.7)** | 1-12 (ALL) | 20 | OpenSpec with iteration | ✅ Complete |
| **[openspec-haiku-4.5](../../tree/openspec-haiku-4.5)** | 1-12 (partial) | 33 | OpenSpec with extensive fixes | ⚠️ Partial (Day 10+ needs optimization) |
| **[openspec-minimax-m2](../../tree/openspec-minimax-m2)** | 1-8 | 16 | OpenSpec + .opencode | 🔄 In Progress |
| **[openspec-sonnet-4.5](../../tree/openspec-sonnet-4.5)** | 1-8 | 13 | OpenSpec framework | 🔄 In Progress |
| **[openspec-glm-4.6](../../tree/openspec-glm-4.6)** | 1-5 | 11 | OpenSpec framework | 🔄 In Progress |
| **[openspec-minimax-m2.1](../../tree/openspec-minimax-m2.1)** | 1-5 | 10 | OpenSpec + .opencode | 🔄 In Progress |
| **[openspec-kimi-for-coding](../../tree/openspec-kimi-for-coding)** | 1-5 | 13 | OpenSpec + .opencode | 🔄 In Progress |
| **[openspec-qwen](../../tree/openspec-qwen)** | 1-5 | 9 | OpenSpec framework | 🔄 In Progress |
| **[openspec-amp-free](../../tree/openspec-amp-free)** | 1-2 | 8 | OpenSpec framework | 🔄 Early Stage |
| **[openspec-gemini](../../tree/openspec-gemini)** | 1-2 | 7 | OpenSpec + Gemini CLI | 🔄 Early Stage |
| **[openspec](../../tree/openspec)** | 0 | 5 | Framework only | 📋 Setup |

## Detailed Branch Analysis

### 1. droid-opus-4.5 (Claude Opus 4.5 - Direct)
- **Completion**: Days 1-12 (100%)
- **Commits**: 14 total
- **Approach**: Direct implementation without OpenSpec framework
- **Notable**:
  - Clean, straightforward implementation
  - Includes `PROMPTS.md` and comprehensive `README.md`
  - No openspec scaffolding
  - All "part two" solutions implemented sequentially

**Commit Pattern**:
```
feat: solve day XX part two (x12)
chore: update project for AoC 2025
docs: update README with all solutions and add PROMPTS.md
```

---

### 2. openspec-opus-4.5 (Claude Opus 4.5 - OpenSpec)
- **Completion**: Days 1-12 (100%)
- **Commits**: 17 total
- **Approach**: Full OpenSpec framework with archived completed changes
- **Notable**:
  - Uses OpenSpec change management system
  - All changes properly archived after completion
  - Includes performance constraints for days 5-12
  - Comprehensive spec files for each day

**Commit Pattern**:
```
add day XX solution: [puzzle name]
archive all completed openspec changes
update README with all day 1-12 solutions
```

**Structure**:
- `openspec/changes/archive/` - All completed changes
- `openspec/specs/` - Specification files for each day
- `.claude/` and `.factory/` command configurations

---

### 3. openspec-glm-4.7 (GLM 4.7)
- **Completion**: Days 1-12 (100%)
- **Commits**: 20 total
- **Approach**: OpenSpec with iterative fixes and performance optimization
- **Notable**:
  - Most complete GLM variant
  - Multiple fix commits (day 6, 8, 9)
  - Day 10 part 2 required elimination approach optimization
  - Task completion tracking via tasks.md updates

**Commit Pattern**:
```
add dayXX solution: [puzzle name]
update dayXX tasks: mark implementation complete
fix dayXX: [specific issue]
wip: dayXX - [progress notes]
```

**Issues Encountered**:
- Day 6: Separator-based problem boundaries needed
- Day 10: Part 2 elimination approach (performance issues)

---

### 4. openspec-haiku-4.5 (Claude Haiku 4.5)
- **Completion**: Days 1-12 (partial - many fixes/revisions)
- **Commits**: 33 total (most commits of any branch)
- **Approach**: OpenSpec with extensive iteration and debugging
- **Notable**:
  - Multiple fix commits for same days (day 7, 8, 9)
  - WIP (Work In Progress) markers for days 10, 11, 12
  - Many "fix" and "optimize" commits
  - Shows iterative problem-solving approach

**Commit Pattern**:
```
add day XX solution: [puzzle name] (partial/WIP)
fix day XX solution: [issue description]
optimize day XX solution: [optimization details]
```

**Issues Encountered**:
- Day 7: Revised algorithm (column-based beam model)
- Day 8: Minimum spanning tree approach needed fixes
- Day 9: Cell limit issues, needed intelligent sampling
- Day 10: Part 2 optimization still needed
- Day 11: Partial solution

---

### 5. openspec-minimax-m2 (Minimax M2)
- **Completion**: Days 1-8
- **Commits**: 16 total
- **Approach**: OpenSpec with .opencode command templates
- **Notable**:
  - Includes `.opencode/` command templates
  - Multiple fix commits for days 6 and 7
  - Detailed commit messages describing algorithms
  - Clippy warnings addressed

**Commit Pattern**:
```
implement day XX: [puzzle name] solution
fix day XX: [specific issue]
fix clippy warnings
```

**Unique Features**:
- Tachyon beam splitting with quantum timelines (Day 7)
- Junction box circuit analysis (Day 8)
- Specific algorithm descriptions in commits

---

### 6. openspec-sonnet-4.5 (Claude Sonnet 4.5)
- **Completion**: Days 1-8
- **Commits**: 13 total
- **Approach**: OpenSpec framework with task completion markers
- **Notable**:
  - Clean commit history
  - Explicit task completion commits
  - Organized progression through days

**Commit Pattern**:
```
add day XX solution: [puzzle name]
mark day XX tasks as completed
mark all day X, Y, Z tasks as completed
```

---

### 7. openspec-glm-4.6 (GLM 4.6)
- **Completion**: Days 1-5
- **Commits**: 11 total
- **Approach**: OpenSpec with post-implementation fixes
- **Notable**:
  - Batch implementation followed by fixes
  - Clippy warnings addressed
  - Example and test fixes needed

**Commit Pattern**:
```
add day XX solution: [puzzle name]
fix examples and tests for days 1-5
update proposals and fix clippy warnings
```

---

### 8. openspec-minimax-m2.1 (Minimax M2.1)
- **Completion**: Days 1-5
- **Commits**: 10 total
- **Approach**: OpenSpec + .opencode with clean commits
- **Notable**:
  - Simpler commit style than m2 variant
  - Straightforward implementation
  - Clean progression through days 1-5

**Commit Pattern**:
```
add dayXX solution: [puzzle name]
init: configure project for AoC 2025
```

---

### 9. openspec-kimi-for-coding (Kimi)
- **Completion**: Days 1-5
- **Commits**: 13 total
- **Approach**: OpenSpec + .opencode with detailed commit prefixes
- **Notable**:
  - Uses structured commit prefixes (add-dayXX-solution:)
  - Explicit file tracking (01-input.txt)
  - Task status updates in proposal files
  - Includes `.opencode/` command templates

**Commit Pattern**:
```
add-dayXX-solution: implement Day X - [Puzzle Name]
fix: update Day X task status in proposal file
```

---

### 10. openspec-qwen (Qwen)
- **Completion**: Days 1-5
- **Commits**: 9 total
- **Approach**: OpenSpec framework with batch implementation
- **Notable**:
  - Days 2 and 3 implemented together
  - Clean, minimal commit history
  - Straightforward progression

**Commit Pattern**:
```
Add Day X solution: [Puzzle Name]
add day 02 and day 03 implementations
```

---

### 11. openspec-amp-free (AMP)
- **Completion**: Days 1-2
- **Commits**: 8 total
- **Approach**: OpenSpec framework with separated part one/two commits
- **Notable**:
  - Each day's parts committed separately
  - Detailed openspec structure
  - Early-stage development

**Commit Pattern**:
```
add dayXX part one solution
add dayXX part two solution
```

---

### 12. openspec-gemini (Gemini)
- **Completion**: Days 1-2
- **Commits**: 7 total
- **Approach**: OpenSpec + Gemini CLI configuration
- **Notable**:
  - Includes Gemini-specific CLI command configs
  - Conventional commit messages (feat:)
  - Structured initialization

**Commit Pattern**:
```
feat: Initialize Advent of Code 2025
feat(dayXX): implement part one of day XX solution
feat: Add Gemini CLI openspec command configurations
```

---

### 13. openspec (Framework Only)
- **Completion**: 0 days
- **Commits**: 5 total
- **Approach**: Framework initialization without solutions
- **Notable**:
  - Contains openspec change proposals for AoC 2025
  - Project-specific details in project.md
  - Base framework for other branches

## Key Observations

### Completion Rate Ranking (by days completed)

**Tier 1 - Complete (Days 1-12)**:
1. droid-opus-4.5 ✅ (14 commits, most efficient)
2. openspec-opus-4.5 ✅ (17 commits)
3. openspec-glm-4.7 ✅ (20 commits)

**Tier 2 - Nearly Complete (Days 1-12 with issues)**:
4. openspec-haiku-4.5 ⚠️ (33 commits, extensive iteration)

**Tier 3 - Strong Progress (Days 1-8)**:
5. openspec-minimax-m2 🔄 (16 commits)
6. openspec-sonnet-4.5 🔄 (13 commits)

**Tier 4 - Moderate Progress (Days 1-5)**:
7. openspec-minimax-m2.1 🔄 (10 commits)
8. openspec-glm-4.6 🔄 (11 commits)
9. openspec-kimi-for-coding 🔄 (13 commits)
10. openspec-qwen 🔄 (9 commits)

**Tier 5 - Early Stage (Days 1-2)**:
11. openspec-amp-free 🔄 (8 commits)
12. openspec-gemini 🔄 (7 commits)

**Tier 6 - Framework Only**:
13. openspec 📋 (5 commits)

### Approach Differences

#### Without OpenSpec Framework
- **droid-opus-4.5**: Direct implementation, cleaner commit history, most efficient

#### With OpenSpec Framework
Most branches use the OpenSpec framework which provides:
- Structured change proposals (`openspec/changes/`)
- Specification files (`openspec/specs/`)
- Task tracking (`tasks.md` files)
- Command configurations (`.claude/`, `.factory/`, `.opencode/`)

#### Additional Tooling
- **Gemini**: Gemini CLI command configs
- **Kimi & Minimax**: `.opencode/` command templates
- **Claude models**: `.claude/` and `.factory/` configurations

### Quality Indicators

**Highest Quality** (based on completion and efficiency):
1. **droid-opus-4.5**: Complete with fewest commits (14), no framework overhead
2. **openspec-opus-4.5**: Complete, archived, organized (17 commits)
3. **openspec-glm-4.7**: Complete with iterative improvements (20 commits)

**Most Persistent** (most fixes/revisions):
1. **openspec-haiku-4.5**: 33 commits with many fixes and WIP markers
2. **openspec-minimax-m2**: Multiple algorithm revisions
3. **openspec-glm-4.7**: Several fix commits

### Commit Message Styles

- **Conventional Commits**: openspec-gemini (feat:, feat():)
- **Descriptive**: openspec-opus-4.5, droid-opus-4.5
- **Prefixed**: openspec-kimi-for-coding (add-dayXX-solution:)
- **Task-oriented**: openspec-sonnet-4.5, openspec-glm-4.7 (mark tasks as completed)
- **Implementation-first**: openspec-minimax-m2 (implement day XX:)

## Common Base

All branches diverge from commit `05e7fa1` (Initial commit) and share these common ancestor commits:
- `05e7fa1` - Initial commit
- `16dbccd` - add a script to fetch puzzle
- `fe5aff1` - introduce openspec (claude, factory)
- `a91cf35` - update openspec/project.md with project-specific details
- `cb60557` - add openspec change proposals for aoc 2025

## Repository Statistics

- **Total Branches**: 13 remote branches (excluding master and ooenspec)
- **Total Unique Commits**: ~200+ commits across all branches
- **Common Framework**: OpenSpec used in 12/13 branches
- **Average Commits per Branch**: ~15 commits
- **Most Active Branch**: openspec-haiku-4.5 (33 commits)
- **Least Active Branch**: openspec (5 commits - framework only)
- **Fully Complete Branches**: 3 (droid-opus-4.5, openspec-opus-4.5, openspec-glm-4.7)

## Conclusions

1. **Best Overall Performance**: Claude Opus 4.5 (both direct and OpenSpec approaches complete all 12 days)
2. **Most Efficient**: droid-opus-4.5 (14 commits for full completion - no framework overhead)
3. **Best Non-Claude Performance**: GLM 4.7 (first non-Claude model to complete all 12 days)
4. **Most Persistent**: Haiku 4.5 (extensive iteration - 33 commits with many fixes)
5. **Most Organized**: OpenSpec branches with proper archiving (Claude Opus 4.5)

The OpenSpec framework provides good structure but adds overhead. The direct implementation (droid-opus-4.5) achieved full completion with the fewest commits. GLM 4.7 demonstrates that non-Claude models can also achieve full completion with the OpenSpec framework.
