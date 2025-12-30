#!/usr/bin/env python3
"""
Get Next Pattern
================

Helper script to query the next incomplete pattern from cleanup progress.
Includes grammar point definition from grammar_points_data.json.

Usage:
    python get_next_pattern.py --progress cleanup_progress.json
    python get_next_pattern.py --progress cleanup_progress.json --grammar-data grammar_points_data.json
"""

import argparse
import json
from pathlib import Path


def get_next_incomplete(progress: dict) -> dict | None:
    """Get the next incomplete pattern."""
    for pattern in progress["patterns"]:
        if pattern["status"] in ("pending", "in_progress"):
            return pattern
    return None


def lookup_grammar_point(name: str, grammar_data_path: Path) -> dict | None:
    """Look up a grammar point definition."""
    with open(grammar_data_path) as f:
        data = json.load(f)

    if name in data:
        return {
            "jlpt": data[name].get("jlpt", ""),
            "structure": data[name].get("structure", {}),
            "about": data[name].get("about", ""),
        }
    return None


def main():
    parser = argparse.ArgumentParser(description="Get next incomplete pattern")
    parser.add_argument(
        "--progress",
        type=Path,
        required=True,
        help="Path to cleanup_progress.json"
    )
    parser.add_argument(
        "--grammar-data",
        type=Path,
        default=None,
        help="Path to grammar_points_data.json"
    )
    parser.add_argument(
        "--json",
        action="store_true",
        help="Output as JSON"
    )
    args = parser.parse_args()

    if not args.progress.exists():
        print(f"Error: Progress file not found: {args.progress}")
        return 1

    progress = json.loads(args.progress.read_text())
    pattern = get_next_incomplete(progress)

    if pattern is None:
        print("All patterns have been reviewed!")
        return 0

    # Look up grammar definition
    grammar_def = None
    if args.grammar_data and args.grammar_data.exists():
        grammar_def = lookup_grammar_point(pattern["name"], args.grammar_data)

    if args.json:
        output = {**pattern, "grammar_definition": grammar_def}
        print(json.dumps(output, indent=2, ensure_ascii=False))
    else:
        print(f"\n{'='*60}")
        print(f"  NEXT PATTERN: {pattern['name']}")
        print(f"{'='*60}")
        print(f"\nJLPT: {pattern['jlpt'].upper()}")
        print(f"File: grammar-lib/src/matchers/{pattern['file']}")
        print(f"Function: {pattern['function']}")
        print(f"Status: {pattern['status']}")

        if grammar_def:
            print(f"\n--- Grammar Definition ---")
            structure = grammar_def.get("structure", {})
            if isinstance(structure, dict):
                print("Structure:")
                for key, forms in structure.items():
                    if forms:
                        print(f"  [{key}]")
                        for form in forms[:3]:
                            print(f"    - {form}")
                        if len(forms) > 3:
                            print(f"    ... and {len(forms) - 3} more")
            elif structure:
                print("Structure:")
                for s in structure[:3]:
                    print(f"  - {s}")

            about = grammar_def.get("about", "")
            if about:
                # Truncate long descriptions
                if len(about) > 300:
                    about = about[:300] + "..."
                print(f"\nAbout: {about}")

        # Show progress
        stats = progress["stats"]
        print(f"\n--- Progress ---")
        print(f"Total: {stats['done']}/{stats['total']} patterns done")

        jlpt = pattern["jlpt"]
        if jlpt in stats.get("by_jlpt", {}):
            jlpt_stats = stats["by_jlpt"][jlpt]
            print(f"{jlpt.upper()}: {jlpt_stats['done']}/{jlpt_stats['total']}")

    return 0


if __name__ == "__main__":
    exit(main())
