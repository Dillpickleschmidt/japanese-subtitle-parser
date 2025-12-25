#!/usr/bin/env python3
"""
Get Next Subtitle
=================

Helper script to query the next incomplete subtitle from validation progress.
Optionally includes pattern lookup data from grammar_points_data.json.

Usage:
    python get_next_subtitle.py --progress validation_progress.json
    python get_next_subtitle.py --progress ./japanese-subtitle-parser/validation_progress.json
    python get_next_subtitle.py --progress validation_progress.json --with-grammar-data ../grammar_points_data.json
"""

import argparse
import json
from pathlib import Path


def get_next_incomplete(progress: dict) -> tuple[str, dict] | None:
    """Get the next incomplete subtitle."""
    sorted_ids = sorted(progress["subtitles"].keys(), key=lambda x: int(x))

    for subtitle_id in sorted_ids:
        subtitle = progress["subtitles"][subtitle_id]
        if subtitle["status"] in ("pending", "in_progress"):
            return subtitle_id, subtitle

    return None


def lookup_patterns(pattern_names: list[str], grammar_data_path: Path) -> dict:
    """
    Look up multiple patterns from grammar_points_data.json.

    Returns:
        Dict mapping pattern name to pattern data
    """
    with open(grammar_data_path) as f:
        data = json.load(f)

    # Build lookup by name (data is a dict keyed by pattern name)
    lookup = {}
    for name, pattern_data in data.items():
        if name in pattern_names:
            # Include only essential fields to keep output small
            lookup[name] = {
                "jlpt": pattern_data.get("jlpt", ""),
                "structure": pattern_data.get("structure", {}),
                "about": pattern_data.get("about", ""),
            }

    return lookup


def main():
    parser = argparse.ArgumentParser(description="Get next incomplete subtitle")
    parser.add_argument(
        "--progress",
        type=Path,
        required=True,
        help="Path to validation_progress.json"
    )
    parser.add_argument(
        "--with-grammar-data",
        type=Path,
        default=None,
        help="Path to grammar_points_data.json to include pattern definitions"
    )
    parser.add_argument(
        "--json",
        action="store_true",
        help="Output as JSON instead of formatted text"
    )
    args = parser.parse_args()

    if not args.progress.exists():
        print(f"Error: Progress file not found: {args.progress}")
        return

    progress = json.loads(args.progress.read_text())
    result = get_next_incomplete(progress)

    if result is None:
        print("All subtitles have been validated!")
        return

    subtitle_id, subtitle = result

    # Optionally look up pattern definitions
    pattern_defs = {}
    if args.with_grammar_data:
        if args.with_grammar_data.exists():
            pattern_names = [p["name"] for p in subtitle["patterns"]]
            pattern_defs = lookup_patterns(pattern_names, args.with_grammar_data)
        else:
            print(f"Warning: Grammar data file not found: {args.with_grammar_data}")

    if args.json:
        output = {
            "subtitle_id": subtitle_id,
            **subtitle,
            "pattern_definitions": pattern_defs
        }
        print(json.dumps(output, indent=2, ensure_ascii=False))
    else:
        print(f"\n{'='*60}")
        print(f"  NEXT SUBTITLE: #{subtitle_id}")
        print(f"{'='*60}")
        print(f"\nText: {subtitle['text']}")
        print(f"Status: {subtitle['status']}")
        print(f"\nPatterns ({len(subtitle['patterns'])}):")
        for i, pattern in enumerate(subtitle['patterns'], 1):
            status = pattern.get('review_status', 'pending')
            name = pattern['name']
            print(f"\n  {i}. [{status}] {name}: \"{pattern['matched_text']}\" ({pattern['jlpt']})")

            # Show pattern definition if available
            if name in pattern_defs:
                defn = pattern_defs[name]
                structure = defn['structure']
                if isinstance(structure, dict):
                    # Flatten structure dict to list of forms
                    all_forms = []
                    for forms in structure.values():
                        all_forms.extend(forms[:2])  # Take first 2 from each category
                    structure_str = ', '.join(all_forms[:3])
                else:
                    structure_str = ', '.join(structure[:3])
                print(f"     Structure: {structure_str}")
                about = defn['about']
                if len(about) > 100:
                    about = about[:100] + "..."
                print(f"     About: {about}")
        print()

        # Show progress stats
        stats = progress["stats"]
        total = stats["total"]
        complete = stats["complete"]
        remaining = total - complete - stats.get("skipped", 0)
        print(f"Progress: {complete}/{total} complete, {remaining} remaining")


if __name__ == "__main__":
    main()
