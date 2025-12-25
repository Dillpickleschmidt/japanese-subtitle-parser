#!/usr/bin/env python3
"""
Pattern Lookup Helper
=====================

Looks up grammar pattern definitions from grammar_points_data.json.
Allows querying by pattern name without loading the entire file into Claude's context.

Usage:
    python lookup_pattern.py --name "ね"
    python lookup_pattern.py --name "ている" --grammar-data ./grammar_points_data.json
"""

import argparse
import json
from pathlib import Path


DEFAULT_GRAMMAR_DATA = Path(__file__).parent / "grammar_points_data.json"


def lookup_pattern(name: str, grammar_data_path: Path) -> dict | None:
    """
    Look up a pattern by name in grammar_points_data.json.

    Args:
        name: The pattern name to look up
        grammar_data_path: Path to grammar_points_data.json

    Returns:
        Pattern data dict or None if not found
    """
    with open(grammar_data_path) as f:
        data = json.load(f)

    # Grammar data is a dict keyed by pattern name
    if name in data:
        return {"name": name, **data[name]}

    return None


def main():
    parser = argparse.ArgumentParser(description="Look up grammar pattern definitions")
    parser.add_argument(
        "--name",
        type=str,
        required=True,
        help="Pattern name to look up"
    )
    parser.add_argument(
        "--grammar-data",
        type=Path,
        default=DEFAULT_GRAMMAR_DATA,
        help=f"Path to grammar_points_data.json (default: {DEFAULT_GRAMMAR_DATA})"
    )
    parser.add_argument(
        "--json",
        action="store_true",
        help="Output as JSON instead of formatted text"
    )
    args = parser.parse_args()

    if not args.grammar_data.exists():
        print(f"Error: Grammar data file not found: {args.grammar_data}")
        return

    pattern = lookup_pattern(args.name, args.grammar_data)

    if pattern is None:
        print(f"Pattern '{args.name}' not found in grammar_points_data.json")
        return

    if args.json:
        print(json.dumps(pattern, indent=2, ensure_ascii=False))
    else:
        print(f"\n{'='*60}")
        print(f"  Pattern: {pattern.get('name', 'N/A')}")
        print(f"{'='*60}")
        print(f"\nJLPT: {pattern.get('jlpt', 'N/A')}")
        print(f"\nStructure:")
        structure = pattern.get("structure", {})
        if isinstance(structure, dict):
            for key, forms in structure.items():
                if forms:
                    print(f"  [{key}]")
                    for form in forms:
                        print(f"    - {form}")
        else:
            for struct in structure:
                print(f"  - {struct}")
        print(f"\nAbout: {pattern.get('about', 'N/A')}")

        examples = pattern.get("examples", [])
        if examples:
            print(f"\nExamples ({len(examples)}):")
            for i, ex in enumerate(examples[:3], 1):  # Show first 3
                print(f"  {i}. {ex.get('ja', 'N/A')}")
                print(f"     {ex.get('en', 'N/A')}")
            if len(examples) > 3:
                print(f"  ... and {len(examples) - 3} more")
        print()


if __name__ == "__main__":
    main()
