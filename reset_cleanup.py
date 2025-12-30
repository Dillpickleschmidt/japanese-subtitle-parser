#!/usr/bin/env python3
"""Reset cleanup progress for patterns that still have inline struct definitions."""

import json
import subprocess
from pathlib import Path

PROJECT_DIR = Path.cwd()
PROGRESS_FILE = PROJECT_DIR / "cleanup_progress.json"

# Load progress
progress = json.loads(PROGRESS_FILE.read_text())

# Find functions that still have inline struct definitions
reset_count = 0
for pattern in progress["patterns"]:
    file_path = PROJECT_DIR / "grammar-lib" / "src" / "matchers" / pattern["file"]

    # Read the file and check if this function still has inline structs
    if file_path.exists():
        content = file_path.read_text()

        # Find the function
        func_start = content.find(f"pub fn {pattern['function']}()")
        if func_start != -1:
            # Find the end of the function (next pub fn or end of file)
            next_func = content.find("\npub fn ", func_start + 1)
            if next_func == -1:
                func_content = content[func_start:]
            else:
                func_content = content[func_start:next_func]

            # Check if this function has inline struct definitions
            if "struct " in func_content and "impl Matcher for" in func_content:
                pattern["status"] = "pending"
                if "completed_at" in pattern:
                    del pattern["completed_at"]
                reset_count += 1
                print(f"Reset {pattern['name']} ({pattern['function']}) in {pattern['file']}")

print(f"\nReset {reset_count} patterns back to pending status")

# Save updated progress
PROGRESS_FILE.write_text(json.dumps(progress, indent=2, ensure_ascii=False))
print("Progress file updated")