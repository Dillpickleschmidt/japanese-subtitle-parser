#!/usr/bin/env python3
"""
Get Matcher Code
================

Extracts the exact code for a pattern's matcher function.
Returns the file path, line range, and the actual code.

Usage:
    python get_matcher_code.py --progress cleanup_progress.json --project-dir .
    python get_matcher_code.py --pattern "だ" --project-dir .
"""

import argparse
import json
import re
import subprocess
from pathlib import Path


def get_next_pattern(progress_path: Path) -> dict | None:
    """Get the next incomplete pattern from progress."""
    progress = json.loads(progress_path.read_text())
    for pattern in progress["patterns"]:
        if pattern["status"] in ("pending", "in_progress"):
            return pattern
    return None


def find_function_range(file_path: Path, function_name: str) -> tuple[int, int] | None:
    """
    Find the start and end line numbers for a Rust function.

    Returns (start_line, end_line) or None if not found.
    """
    content = file_path.read_text()
    lines = content.split('\n')

    # Find function start: pub fn function_name() or pub fn function_name<...>()
    pattern = rf'^pub fn {re.escape(function_name)}\s*[<(]'
    start_line = None

    for i, line in enumerate(lines, 1):
        if re.match(pattern, line):
            start_line = i
            break

    if start_line is None:
        return None

    # Find function end: closing brace at column 0
    brace_count = 0
    in_function = False
    end_line = None

    for i, line in enumerate(lines[start_line - 1:], start_line):
        # Count braces
        for char in line:
            if char == '{':
                brace_count += 1
                in_function = True
            elif char == '}':
                brace_count -= 1

        # Function ends when brace count returns to 0
        if in_function and brace_count == 0:
            end_line = i
            break

    if end_line is None:
        # Fallback: use a reasonable chunk
        end_line = min(start_line + 100, len(lines))

    return start_line, end_line


def main():
    parser = argparse.ArgumentParser(description="Extract matcher code for a pattern")
    parser.add_argument("--progress", type=Path, help="Path to cleanup_progress.json")
    parser.add_argument("--pattern", type=str, help="Specific pattern name to look up")
    parser.add_argument("--project-dir", type=Path, required=True, help="Project directory")
    parser.add_argument("--grammar-data", type=Path, help="Path to grammar_points_data.json")
    parser.add_argument("--code", action="store_true", help="Also output the actual code")
    args = parser.parse_args()

    # Get pattern info
    if args.pattern:
        # Look up specific pattern from progress
        progress_path = args.progress or (args.project_dir / "cleanup_progress.json")
        if not progress_path.exists():
            print(f"Error: Progress file not found: {progress_path}")
            return 1
        progress = json.loads(progress_path.read_text())
        pattern = None
        for p in progress["patterns"]:
            if p["name"] == args.pattern:
                pattern = p
                break
        if not pattern:
            print(f"Error: Pattern '{args.pattern}' not found")
            return 1
    elif args.progress:
        pattern = get_next_pattern(args.progress)
        if not pattern:
            print("All patterns have been reviewed!")
            return 0
    else:
        print("Error: Either --progress or --pattern is required")
        return 1

    # Find the matcher file and function
    file_path = args.project_dir / "grammar-lib" / "src" / "matchers" / pattern["file"]
    if not file_path.exists():
        print(f"Error: File not found: {file_path}")
        return 1

    function_name = pattern["function"]
    line_range = find_function_range(file_path, function_name)

    if not line_range:
        print(f"Error: Function '{function_name}' not found in {file_path}")
        return 1

    start_line, end_line = line_range

    # Output
    print(f"============================================================")
    print(f"  PATTERN: {pattern['name']} ({pattern['jlpt'].upper()})")
    print(f"============================================================")
    print(f"")
    print(f"File: grammar-lib/src/matchers/{pattern['file']}")
    print(f"Function: {function_name}()")
    print(f"Lines: {start_line}-{end_line} ({end_line - start_line + 1} lines)")
    print(f"")
    print(f"--- Commands (copy-paste these exactly) ---")
    print(f"Mark in progress:")
    print(f"  python mark_pattern_done.py --progress cleanup_progress.json --pattern \"{pattern['name']}\" --status in_progress")
    print(f"Mark done:")
    print(f"  python mark_pattern_done.py --progress cleanup_progress.json --pattern \"{pattern['name']}\" --status done")

    # Look up grammar definition if available
    grammar_data_path = args.grammar_data or (args.project_dir / "grammar_points_data.json")
    if grammar_data_path.exists():
        grammar_data = json.loads(grammar_data_path.read_text())
        if pattern["name"] in grammar_data:
            defn = grammar_data[pattern["name"]]
            print(f"")
            print(f"--- Grammar Definition ---")
            structure = defn.get("structure", {})
            if isinstance(structure, dict):
                print("Structure:")
                for key, forms in structure.items():
                    if forms:
                        for form in forms[:2]:
                            print(f"  - {form}")
            about = defn.get("about", "")
            if about and len(about) > 200:
                about = about[:200] + "..."
            if about:
                print(f"About: {about}")

    if args.code:
        print(f"")
        print(f"--- Code ---")
        lines = file_path.read_text().split('\n')
        for i in range(start_line - 1, end_line):
            print(f"{i + 1:4d}│ {lines[i]}")

    # Show progress stats
    if args.progress and args.progress.exists():
        progress = json.loads(args.progress.read_text())
        stats = progress["stats"]
        print(f"")
        print(f"--- Progress ---")
        print(f"Total: {stats['done']}/{stats['total']} patterns done")

    return 0


if __name__ == "__main__":
    exit(main())
