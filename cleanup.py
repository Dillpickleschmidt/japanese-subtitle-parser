#!/usr/bin/env python3
"""
Pattern Cleanup CLI
===================

Unified CLI for pattern cleanup workflow.

Usage:
    python cleanup.py next              # Get next pattern (auto-marks in_progress)
    python cleanup.py skip              # Pattern is clean, move to next
    python cleanup.py done              # Pattern edited, mark complete
    python cleanup.py search "pattern"  # Search in matchers directory
    python cleanup.py status            # Show progress summary
"""

import argparse
import json
import re
import subprocess
import sys
from pathlib import Path
from datetime import datetime


# Auto-detect paths
SCRIPT_DIR = Path(__file__).parent
PROJECT_DIR = Path.cwd()  # Assume running from project directory
PROGRESS_FILE = PROJECT_DIR / "cleanup_progress.json"
GRAMMAR_DATA = PROJECT_DIR / "grammar_points_data.json"
MATCHERS_DIR = PROJECT_DIR / "grammar-lib" / "src" / "matchers"
MOD_RS = MATCHERS_DIR / "mod.rs"


def load_progress() -> dict | None:
    """Load cleanup progress."""
    if PROGRESS_FILE.exists():
        return json.loads(PROGRESS_FILE.read_text())
    return None


def save_progress(progress: dict) -> None:
    """Save cleanup progress."""
    PROGRESS_FILE.write_text(json.dumps(progress, indent=2, ensure_ascii=False))


def get_current_pattern(progress: dict) -> dict | None:
    """Get the current (next incomplete) pattern."""
    for pattern in progress["patterns"]:
        if pattern["status"] in ("pending", "in_progress"):
            return pattern
    return None


def extract_helpers_from_mod_rs() -> list[str]:
    """Extract public helper function signatures from mod.rs."""
    if not MOD_RS.exists():
        return []

    content = MOD_RS.read_text()
    helpers = []

    # Match pub fn declarations
    for match in re.finditer(r'^pub fn (\w+)[<(].*?(?:-> [^{]+)?', content, re.MULTILINE):
        line = match.group(0).strip()
        # Clean up multi-line signatures
        line = ' '.join(line.split())
        if len(line) > 80:
            line = line[:77] + "..."
        helpers.append(line)

    return helpers


def git_commit(message: str) -> bool:
    """Run git add and commit. Returns True on success."""
    try:
        # Stage all changes
        result = subprocess.run(
            ["git", "add", "-A"],
            cwd=PROJECT_DIR,
            capture_output=True,
            text=True
        )
        if result.returncode != 0:
            print(f"git add failed: {result.stderr}")
            return False

        # Check if there are changes to commit
        result = subprocess.run(
            ["git", "diff", "--cached", "--quiet"],
            cwd=PROJECT_DIR,
            capture_output=True
        )
        if result.returncode == 0:
            print("No changes to commit.")
            return True

        # Commit
        result = subprocess.run(
            ["git", "commit", "-m", message],
            cwd=PROJECT_DIR,
            capture_output=True,
            text=True
        )
        if result.returncode != 0:
            print(f"git commit failed: {result.stderr}")
            return False

        print(f"Committed: {message}")
        return True
    except Exception as e:
        print(f"Git error: {e}")
        return False


def run_tests(jlpt_level: str) -> tuple[bool, str]:
    """Run tests for JLPT level. Returns (success, output)."""
    cmd = ["cargo", "test", f"{jlpt_level}_patterns", "-p", "grammar-lib", "--quiet"]
    try:
        result = subprocess.run(
            cmd,
            capture_output=True,
            text=True,
            cwd=PROJECT_DIR,
            timeout=120  # 2 minute timeout
        )
        return result.returncode == 0, result.stdout + result.stderr
    except subprocess.TimeoutExpired:
        return False, "Tests timed out after 120 seconds"
    except Exception as e:
        return False, f"Failed to run tests: {e}"


def find_function_range(file_path: Path, function_name: str) -> tuple[int, int] | None:
    """Find start and end line numbers for a Rust function."""
    content = file_path.read_text()
    lines = content.split('\n')

    pattern = rf'^pub fn {re.escape(function_name)}\s*[<(]'
    start_line = None

    for i, line in enumerate(lines, 1):
        if re.match(pattern, line):
            start_line = i
            break

    if start_line is None:
        return None

    brace_count = 0
    in_function = False
    end_line = None

    for i, line in enumerate(lines[start_line - 1:], start_line):
        for char in line:
            if char == '{':
                brace_count += 1
                in_function = True
            elif char == '}':
                brace_count -= 1

        if in_function and brace_count == 0:
            end_line = i
            break

    if end_line is None:
        end_line = min(start_line + 100, len(lines))

    return start_line, end_line


# ============================================================
# Subcommands
# ============================================================

def cmd_next(args):
    """Get next pattern with full code. Auto-marks as in_progress."""
    progress = load_progress()
    if not progress:
        print("Error: No cleanup_progress.json found. Run from project directory.")
        return 1

    pattern = get_current_pattern(progress)
    if not pattern:
        print("All patterns have been reviewed!")
        return 0

    # STRICT STATE ENFORCEMENT: Refuse if already in_progress
    if pattern["status"] == "in_progress":
        print("=" * 60)
        print("  ERROR: Pattern already in progress!")
        print("=" * 60)
        print()
        print(f"Current pattern: {pattern['name']}")
        print()
        print("You MUST run one of these commands first:")
        print("  python cleanup.py skip   (if pattern is already clean)")
        print("  python cleanup.py done   (after editing and testing)")
        print()
        print("DO NOT run 'next' again!")
        print("=" * 60)
        return 1

    # Mark as in_progress (only reached if status was "pending")
    pattern["status"] = "in_progress"
    progress["stats"]["pending"] = max(0, progress["stats"].get("pending", 1) - 1)
    progress["stats"]["in_progress"] = progress["stats"].get("in_progress", 0) + 1
    save_progress(progress)

    # Find function in file
    file_path = MATCHERS_DIR / pattern["file"]
    if not file_path.exists():
        print(f"Error: File not found: {file_path}")
        return 1

    function_name = pattern["function"]
    line_range = find_function_range(file_path, function_name)

    if not line_range:
        print(f"Error: Function '{function_name}' not found in {file_path}")
        return 1

    start_line, end_line = line_range
    num_lines = end_line - start_line + 1

    # Output
    print(f"{'='*60}")
    print(f"  PATTERN: {pattern['name']} ({pattern['jlpt'].upper()})")
    print(f"{'='*60}")
    print()
    print(f"File: grammar-lib/src/matchers/{pattern['file']}")
    print(f"Function: {function_name}()")
    print(f"Lines: {start_line}-{end_line} ({num_lines} lines)")
    print()

    # Grammar definition
    if GRAMMAR_DATA.exists():
        grammar_data = json.loads(GRAMMAR_DATA.read_text())
        if pattern["name"] in grammar_data:
            defn = grammar_data[pattern["name"]]
            print("--- Grammar Definition ---")
            structure = defn.get("structure", {})
            if isinstance(structure, dict):
                for key, forms in structure.items():
                    if forms:
                        for form in forms[:2]:
                            print(f"  {form}")
            about = defn.get("about", "")
            if about:
                if len(about) > 200:
                    about = about[:200] + "..."
                print(f"About: {about}")
            print()

    # Available helpers from mod.rs
    helpers = extract_helpers_from_mod_rs()
    if helpers:
        print("--- Available Helpers (from mod.rs) ---")
        for helper in helpers:
            print(f"  {helper}")
        print()

    # Code
    print("--- Code ---")
    lines = file_path.read_text().split('\n')
    for i in range(start_line - 1, end_line):
        print(f"{i + 1:4d}| {lines[i]}")

    # Progress
    print()
    print("--- Progress ---")
    stats = progress["stats"]
    print(f"Total: {stats['done']}/{stats['total']} done")

    # IMPORTANT: Force commitment warning
    print()
    print("=" * 60)
    print("  YOU MUST NOW DO ONE OF:")
    print("  - python cleanup.py skip   (if already clean)")
    print("  - Edit code, test, then: python cleanup.py done")
    print()
    print("  DO NOT run 'next' again until you skip or done!")
    print("=" * 60)

    return 0


def cmd_start(args):
    """Mark current pattern as in_progress."""
    progress = load_progress()
    if not progress:
        print("Error: No cleanup_progress.json found.")
        return 1

    pattern = get_current_pattern(progress)
    if not pattern:
        print("No pattern to start - all done!")
        return 0

    if pattern["status"] == "in_progress":
        print(f"Already in progress: {pattern['name']}")
        return 0

    pattern["status"] = "in_progress"
    progress["stats"]["pending"] = max(0, progress["stats"].get("pending", 1) - 1)
    progress["stats"]["in_progress"] = progress["stats"].get("in_progress", 0) + 1

    save_progress(progress)
    print(f"Started: {pattern['name']}")
    return 0


def cmd_show(args):
    """Show current in-progress pattern (read-only, no state change)."""
    progress = load_progress()
    if not progress:
        print("Error: No cleanup_progress.json found.")
        return 1

    pattern = None
    for p in progress["patterns"]:
        if p["status"] == "in_progress":
            pattern = p
            break

    if not pattern:
        print("No pattern in progress. Run 'next' first.")
        return 1

    # Find function in file
    file_path = MATCHERS_DIR / pattern["file"]
    if not file_path.exists():
        print(f"Error: File not found: {file_path}")
        return 1

    function_name = pattern["function"]
    line_range = find_function_range(file_path, function_name)

    if not line_range:
        print(f"Error: Function '{function_name}' not found in {file_path}")
        return 1

    start_line, end_line = line_range
    num_lines = end_line - start_line + 1

    # Output
    print(f"{'='*60}")
    print(f"  CURRENT PATTERN: {pattern['name']} ({pattern['jlpt'].upper()})")
    print(f"{'='*60}")
    print()
    print(f"File: grammar-lib/src/matchers/{pattern['file']}")
    print(f"Function: {function_name}()")
    print(f"Lines: {start_line}-{end_line} ({num_lines} lines)")
    print()

    # Available helpers
    helpers = extract_helpers_from_mod_rs()
    if helpers:
        print("--- Available Helpers ---")
        for helper in helpers:
            print(f"  {helper}")
        print()

    # Code
    print("--- Code ---")
    lines = file_path.read_text().split('\n')
    for i in range(start_line - 1, end_line):
        print(f"{i + 1:4d}| {lines[i]}")

    print()
    print("=" * 60)
    print("  Run 'skip' or 'done' to complete this pattern")
    print("=" * 60)

    return 0


def cmd_skip(args):
    """Mark current pattern as done (already clean, no changes needed)."""
    progress = load_progress()
    if not progress:
        print("Error: No cleanup_progress.json found.")
        return 1

    pattern = None
    for p in progress["patterns"]:
        if p["status"] == "in_progress":
            pattern = p
            break

    if not pattern:
        print("No pattern to skip - run 'next' first!")
        return 1

    pattern["status"] = "done"
    pattern["completed_at"] = datetime.now().isoformat()
    pattern["notes"] = "Already clean - skipped"

    # Update stats
    progress["stats"]["in_progress"] = max(0, progress["stats"].get("in_progress", 1) - 1)
    progress["stats"]["done"] = progress["stats"].get("done", 0) + 1

    # Update by_jlpt
    jlpt = pattern["jlpt"]
    if jlpt in progress["stats"].get("by_jlpt", {}):
        progress["stats"]["by_jlpt"][jlpt]["done"] += 1

    save_progress(progress)
    print(f"Skipped (already clean): {pattern['name']}")
    print(f"Progress: {progress['stats']['done']}/{progress['stats']['total']} done")

    # Auto-commit
    print()
    git_commit(f"Cleanup: {pattern['name']} (skipped - already clean)")

    print()
    print("Now run: python cleanup.py next")
    return 0


def cmd_done(args):
    """Mark current pattern as done (after editing). Runs tests first."""
    progress = load_progress()
    if not progress:
        print("Error: No cleanup_progress.json found.")
        return 1

    pattern = None
    for p in progress["patterns"]:
        if p["status"] == "in_progress":
            pattern = p
            break

    if not pattern:
        print("No pattern to mark done - run 'next' first!")
        return 1

    # Run tests BEFORE marking as done
    jlpt = pattern["jlpt"]
    print(f"Running {jlpt}_patterns tests...")
    success, output = run_tests(jlpt)

    if not success:
        print()
        print("=" * 60)
        print("  TESTS FAILED! Pattern NOT marked as done.")
        print("=" * 60)
        print()
        if output.strip():
            # Limit output to avoid flooding
            lines = output.strip().split('\n')
            for line in lines[:30]:
                print(line)
            if len(lines) > 30:
                print(f"... ({len(lines) - 30} more lines)")
        print()
        print("Options:")
        print("  1. Fix the code and run 'done' again")
        print("  2. Revert with: git checkout grammar-lib/")
        print("  3. Skip with: python cleanup.py skip")
        return 1

    print("Tests passed!")
    print()

    # Mark as done
    pattern["status"] = "done"
    pattern["completed_at"] = datetime.now().isoformat()

    # Update stats
    progress["stats"]["in_progress"] = max(0, progress["stats"].get("in_progress", 1) - 1)
    progress["stats"]["done"] = progress["stats"].get("done", 0) + 1

    # Update by_jlpt
    if jlpt in progress["stats"].get("by_jlpt", {}):
        progress["stats"]["by_jlpt"][jlpt]["done"] += 1

    save_progress(progress)
    print(f"Completed: {pattern['name']}")
    print(f"Progress: {progress['stats']['done']}/{progress['stats']['total']} done")

    # Auto-commit
    print()
    git_commit(f"Cleanup: {pattern['name']}")

    print()
    print("Now run: python cleanup.py next")
    return 0


def cmd_search(args):
    """Search in matchers directory using ripgrep."""
    if not MATCHERS_DIR.exists():
        print(f"Error: Matchers directory not found: {MATCHERS_DIR}")
        return 1

    pattern = args.pattern

    # Use ripgrep for fast searching
    cmd = ["rg", "-n", "--color=never", pattern, str(MATCHERS_DIR)]

    try:
        result = subprocess.run(cmd, capture_output=True, text=True)
        if result.returncode == 0:
            # Format output nicely
            lines = result.stdout.strip().split('\n')
            print(f"Found {len(lines)} matches for '{pattern}':")
            print()
            for line in lines[:50]:  # Limit output
                # Convert absolute path to relative
                line = line.replace(str(PROJECT_DIR) + "/", "")
                print(line)
            if len(lines) > 50:
                print(f"... and {len(lines) - 50} more")
        else:
            print(f"No matches found for '{pattern}'")
        return 0
    except FileNotFoundError:
        # Fallback to grep if rg not available
        cmd = ["grep", "-rn", pattern, str(MATCHERS_DIR)]
        result = subprocess.run(cmd, capture_output=True, text=True)
        if result.returncode == 0:
            print(result.stdout)
        else:
            print(f"No matches found for '{pattern}'")
        return 0


def cmd_status(args):
    """Show progress summary."""
    progress = load_progress()
    if not progress:
        print("Error: No cleanup_progress.json found.")
        return 1

    stats = progress["stats"]

    print()
    print(f"{'='*40}")
    print(f"  CLEANUP PROGRESS")
    print(f"{'='*40}")
    print()
    print(f"Total: {stats['done']}/{stats['total']} patterns done ({100*stats['done']//stats['total']}%)")
    print(f"  Done:        {stats['done']}")
    print(f"  In Progress: {stats.get('in_progress', 0)}")
    print(f"  Pending:     {stats.get('pending', 0)}")
    print()
    print("By JLPT:")
    for jlpt in sorted(stats.get("by_jlpt", {}).keys()):
        jlpt_stats = stats["by_jlpt"][jlpt]
        pct = 100 * jlpt_stats['done'] // jlpt_stats['total'] if jlpt_stats['total'] > 0 else 0
        print(f"  {jlpt.upper()}: {jlpt_stats['done']}/{jlpt_stats['total']} ({pct}%)")

    # Show current pattern
    pattern = get_current_pattern(progress)
    if pattern:
        print()
        print(f"Current: {pattern['name']} ({pattern['jlpt'].upper()})")

    return 0


def main():
    parser = argparse.ArgumentParser(
        description="Pattern cleanup CLI",
        formatter_class=argparse.RawDescriptionHelpFormatter,
        epilog="""
Commands:
  next      Get next pattern with full code
  start     Mark current pattern as in_progress
  done      Mark current pattern as done
  search    Search in matchers directory
  status    Show progress summary

Examples:
  python cleanup.py next
  python cleanup.py start
  python cleanup.py search "NNoMatcher"
  python cleanup.py done
"""
    )

    subparsers = parser.add_subparsers(dest="command", help="Command to run")

    # next
    subparsers.add_parser("next", help="Get next pattern (auto-marks in_progress)")

    # show
    subparsers.add_parser("show", help="Re-display current in-progress pattern")

    # skip
    subparsers.add_parser("skip", help="Pattern is clean, move to next")

    # done
    subparsers.add_parser("done", help="Pattern edited, mark complete")

    # search
    search_parser = subparsers.add_parser("search", help="Search in matchers directory")
    search_parser.add_argument("pattern", help="Pattern to search for")

    # status
    subparsers.add_parser("status", help="Show progress summary")

    args = parser.parse_args()

    if not args.command:
        parser.print_help()
        return 1

    commands = {
        "next": cmd_next,
        "show": cmd_show,
        "skip": cmd_skip,
        "done": cmd_done,
        "search": cmd_search,
        "status": cmd_status,
    }

    return commands[args.command](args)


if __name__ == "__main__":
    sys.exit(main())
