"""
Cleanup Progress Management
===========================

Track progress through grammar patterns for robustness review.
Each pattern maps to one matcher function that may be composed of helpers.
"""

import json
from pathlib import Path
from datetime import datetime

PROGRESS_FILE = "cleanup_progress.json"


def load_progress(project_dir: Path) -> dict | None:
    """Load cleanup progress from JSON file."""
    progress_path = project_dir / PROGRESS_FILE
    if progress_path.exists():
        with open(progress_path) as f:
            return json.load(f)
    return None


def save_progress(project_dir: Path, progress: dict) -> None:
    """Save cleanup progress to JSON file."""
    progress_path = project_dir / PROGRESS_FILE
    with open(progress_path, "w") as f:
        json.dump(progress, f, indent=2, ensure_ascii=False)


def get_next_pattern(project_dir: Path) -> dict | None:
    """Get the next pattern to process."""
    progress = load_progress(project_dir)
    if not progress:
        return None

    for pattern in progress["patterns"]:
        if pattern["status"] in ("pending", "in_progress"):
            return pattern
    return None


def mark_pattern_status(
    project_dir: Path,
    pattern_name: str,
    status: str,
    notes: str = ""
) -> None:
    """Mark a pattern's status."""
    progress = load_progress(project_dir)
    if not progress:
        return

    for pattern in progress["patterns"]:
        if pattern["name"] == pattern_name:
            old_status = pattern["status"]
            pattern["status"] = status
            if notes:
                pattern["notes"] = notes
            if status == "done":
                pattern["completed_at"] = datetime.now().isoformat()

            # Update stats
            if old_status != status:
                if old_status in ("pending", "in_progress"):
                    progress["stats"][old_status] = max(0, progress["stats"].get(old_status, 1) - 1)
                if status in ("pending", "in_progress", "done"):
                    progress["stats"][status] = progress["stats"].get(status, 0) + 1

                # Update by_jlpt stats
                jlpt = pattern["jlpt"]
                if status == "done" and jlpt in progress["stats"]["by_jlpt"]:
                    progress["stats"]["by_jlpt"][jlpt]["done"] += 1
            break

    save_progress(project_dir, progress)


def update_stats(progress: dict) -> None:
    """Recalculate stats from pattern statuses."""
    stats = {"total": len(progress["patterns"]), "done": 0, "in_progress": 0, "pending": 0, "by_jlpt": {}}

    for pattern in progress["patterns"]:
        status = pattern["status"]
        if status in stats:
            stats[status] += 1

        jlpt = pattern["jlpt"]
        if jlpt not in stats["by_jlpt"]:
            stats["by_jlpt"][jlpt] = {"total": 0, "done": 0}
        stats["by_jlpt"][jlpt]["total"] += 1
        if status == "done":
            stats["by_jlpt"][jlpt]["done"] += 1

    progress["stats"] = stats


def print_session_header(iteration: int, is_first_run: bool) -> None:
    """Print session header."""
    print("\n" + "=" * 70)
    if is_first_run:
        print(f"  SESSION {iteration}: INITIALIZATION")
    else:
        print(f"  SESSION {iteration}: CLEANUP")
    print("=" * 70 + "\n")


def print_progress_summary(project_dir: Path) -> None:
    """Print cleanup progress summary."""
    progress = load_progress(project_dir)
    if not progress:
        print("No cleanup progress found.")
        return

    stats = progress["stats"]
    print(f"\nProgress: {stats['done']}/{stats['total']} patterns done")
    print(f"  - Done: {stats['done']}")
    print(f"  - In Progress: {stats.get('in_progress', 0)}")
    print(f"  - Pending: {stats.get('pending', 0)}")

    # Show by JLPT
    print("\nBy JLPT level:")
    for jlpt in sorted(stats.get("by_jlpt", {}).keys()):
        jlpt_stats = stats["by_jlpt"][jlpt]
        print(f"  {jlpt.upper()}: {jlpt_stats['done']}/{jlpt_stats['total']}")

    # Show current pattern
    next_pattern = get_next_pattern(project_dir)
    if next_pattern:
        print(f"\nNext pattern: {next_pattern['name']} ({next_pattern['jlpt'].upper()})")
        print(f"  File: {next_pattern['file']}, Function: {next_pattern['function']}")
