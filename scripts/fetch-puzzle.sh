#!/bin/bash
# Fetch Advent of Code puzzle description
# Usage: ./scripts/fetch-puzzle.sh <day>
#
# Requires AOC_SESSION environment variable or .aoc-session file
# (Session is needed to access Part 2 after completing Part 1)

set -e

YEAR=2025
DAY=${1:-$(date +%-d)}

# Pad day to 2 digits
DAY_PADDED=$(printf "%02d" "$DAY")

# Get session token
if [ -n "$AOC_SESSION" ]; then
  SESSION="$AOC_SESSION"
elif [ -f ".aoc-session" ]; then
  SESSION=$(cat .aoc-session)
elif [ -f "$HOME/.aoc-session" ]; then
  SESSION=$(cat "$HOME/.aoc-session")
else
  echo "Error: No session token found."
  echo "Set AOC_SESSION env var or create .aoc-session file"
  exit 1
fi

OUTPUT="inputs/${DAY_PADDED}-puzzle.html"
URL="https://adventofcode.com/${YEAR}/day/${DAY}"

echo "Fetching Day ${DAY} puzzle..."
curl -s -b "session=${SESSION}" "$URL" -o "$OUTPUT"

if grep -q "Please log in" "$OUTPUT" 2>/dev/null; then
  echo "Error: Invalid session token"
  rm "$OUTPUT"
  exit 1
fi

if grep -q "404 Not Found" "$OUTPUT" 2>/dev/null; then
  echo "Error: Puzzle not yet available"
  rm "$OUTPUT"
  exit 1
fi

echo "Saved to ${OUTPUT}"

# Check if Part 2 is available
if grep -q "Part Two" "$OUTPUT"; then
  echo "✓ Part 2 is available"
else
  echo "○ Part 2 not yet unlocked (complete Part 1 first)"
fi
