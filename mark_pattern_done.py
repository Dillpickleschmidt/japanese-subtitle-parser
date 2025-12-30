#!/usr/bin/env python3
"""
Mark Pattern Status
===================

Helper script to update a pattern's status in cleanup progress.

Usage:
    python mark_pattern_done.py --progress cleanup_progress.json --pattern "てしまう" --status in_progress
    python mark_pattern_done.py --progress cleanup_progress.json --pattern "てしまう" --status done
    python mark_pattern_done.py --progress cleanup_progress.json --pattern "てしまう" --status done --notes "Extracted helper"
"""

import argparse
import json
from pathlib import Path
from datetime import datetime


def main():
    parser = argparse.ArgumentParser(description="Mark pattern status")
    parser.add_argument(
        "--progress",
        type=Path,
        required=True,
        help="Path to cleanup_progress.json"
    )
    parser.add_argument(
        "--pattern",
        type=str,
        required=True,
        help="Pattern name to update"
    )
    parser.add_argument(
        "--status",
        type=str,
        required=True,
        choices=["pending", "in_progress", "done", "skipped"],
        help="New status"
    )
    parser.add_argument(
        "--notes",
        type=str,
        default="",
        help="Optional notes about the change"
    )
    args = parser.parse_args()

    if not args.progress.exists():
        print(f"Error: Progress file not found: {args.progress}")
        return 1

    progress = json.loads(args.progress.read_text())

    # Find and update pattern
    found = False
    for pattern in progress["patterns"]:
        if pattern["name"] == args.pattern:
            old_status = pattern["status"]
            pattern["status"] = args.status
            if args.notes:
                pattern["notes"] = args.notes
            if args.status == "done":
                pattern["completed_at"] = datetime.now().isoformat()

            # Update stats
            if old_status != args.status:
                # Decrement old status
                if old_status in progress["stats"]:
                    progress["stats"][old_status] = max(0, progress["stats"].get(old_status, 1) - 1)
                # Increment new status
                if args.status in ("pending", "in_progress", "done"):
                    progress["stats"][args.status] = progress["stats"].get(args.status, 0) + 1

                # Update by_jlpt
                jlpt = pattern["jlpt"]
                if args.status == "done" and jlpt in progress["stats"].get("by_jlpt", {}):
                    progress["stats"]["by_jlpt"][jlpt]["done"] += 1

            found = True
            break

    if not found:
        print(f"Error: Pattern '{args.pattern}' not found")
        return 1

    # Save
    args.progress.write_text(json.dumps(progress, indent=2, ensure_ascii=False))
    print(f"Marked '{args.pattern}' as {args.status}")

    # Show progress
    stats = progress["stats"]
    print(f"Progress: {stats['done']}/{stats['total']} patterns done")

    return 0


if __name__ == "__main__":
    exit(main())
