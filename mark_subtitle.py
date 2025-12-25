#!/usr/bin/env python3
"""
Mark Subtitle Status
====================

Helper script to update subtitle status in validation_progress.json
without loading the file into Claude's context.

Usage:
    python mark_subtitle.py --progress validation_progress.json --id 42 --status complete
    python mark_subtitle.py --progress validation_progress.json --id 42 --status skipped --reason "No patterns"
    python mark_subtitle.py --progress validation_progress.json --id 42 --pattern 0 --review-status valid
    python mark_subtitle.py --progress validation_progress.json --id 42 --pattern 0 --review-status false_positive --notes "Fixed matcher"
"""

import argparse
import json
from datetime import datetime
from pathlib import Path


def main():
    parser = argparse.ArgumentParser(description="Update subtitle status in validation progress")
    parser.add_argument(
        "--progress",
        type=Path,
        required=True,
        help="Path to validation_progress.json"
    )
    parser.add_argument(
        "--id",
        type=str,
        required=True,
        help="Subtitle ID to update"
    )
    parser.add_argument(
        "--status",
        type=str,
        choices=["pending", "in_progress", "complete", "skipped"],
        help="New status for the subtitle"
    )
    parser.add_argument(
        "--reason",
        type=str,
        default="",
        help="Skip reason (when status is skipped)"
    )
    parser.add_argument(
        "--pattern",
        type=int,
        default=None,
        help="Pattern index to update (0-indexed)"
    )
    parser.add_argument(
        "--review-status",
        type=str,
        choices=["pending", "valid", "false_positive", "miscategorized"],
        help="New review status for the pattern"
    )
    parser.add_argument(
        "--notes",
        type=str,
        default="",
        help="Notes for the pattern"
    )
    args = parser.parse_args()

    if not args.progress.exists():
        print(f"Error: Progress file not found: {args.progress}")
        return

    # Load progress
    progress = json.loads(args.progress.read_text())

    # Find subtitle
    if args.id not in progress["subtitles"]:
        print(f"Error: Subtitle ID '{args.id}' not found")
        return

    subtitle = progress["subtitles"][args.id]
    updated = False

    # Update pattern if specified
    if args.pattern is not None:
        if args.pattern < 0 or args.pattern >= len(subtitle["patterns"]):
            print(f"Error: Pattern index {args.pattern} out of range (0-{len(subtitle['patterns'])-1})")
            return

        pattern = subtitle["patterns"][args.pattern]

        if args.review_status:
            old_status = pattern.get("review_status", "pending")
            pattern["review_status"] = args.review_status
            updated = True
            print(f"Updated pattern {args.pattern} review_status: {old_status} -> {args.review_status}")

            # Update stats
            if args.review_status == "false_positive" and old_status != "false_positive":
                progress["stats"]["false_positives_fixed"] = progress["stats"].get("false_positives_fixed", 0) + 1
            elif args.review_status == "miscategorized" and old_status != "miscategorized":
                progress["stats"]["miscategorized"] = progress["stats"].get("miscategorized", 0) + 1

        if args.notes:
            pattern["notes"] = args.notes
            updated = True
            print(f"Added notes to pattern {args.pattern}")

    # Update subtitle status if specified
    if args.status:
        old_status = subtitle["status"]
        subtitle["status"] = args.status
        subtitle["reviewed_at"] = datetime.now().isoformat()
        updated = True
        print(f"Updated subtitle {args.id} status: {old_status} -> {args.status}")

        # Update stats
        if args.status == "complete" and old_status != "complete":
            progress["stats"]["complete"] += 1
        elif args.status == "skipped" and old_status != "skipped":
            progress["stats"]["skipped"] = progress["stats"].get("skipped", 0) + 1
            if args.reason:
                subtitle["skip_reason"] = args.reason

    if updated:
        # Save progress
        args.progress.write_text(json.dumps(progress, indent=2, ensure_ascii=False))
        print(f"Saved {args.progress}")

        # Show updated stats
        stats = progress["stats"]
        total = stats["total"]
        complete = stats["complete"]
        skipped = stats.get("skipped", 0)
        remaining = total - complete - skipped
        print(f"\nProgress: {complete}/{total} complete, {skipped} skipped, {remaining} remaining")
    else:
        print("No changes made. Specify --status and/or --pattern with --review-status")


if __name__ == "__main__":
    main()
