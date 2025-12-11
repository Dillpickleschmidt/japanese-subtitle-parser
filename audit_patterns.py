#!/usr/bin/env python3
"""
Audit script to generate feature_list.json from patterns.rs and test files.
"""
import json
import re
import os
from pathlib import Path

# Read patterns.rs to extract all pattern definitions
def extract_patterns_from_source():
    patterns_file = Path("grammar-lib/src/patterns.rs")
    with open(patterns_file) as f:
        content = f.read()

    # Extract pattern blocks using regex
    pattern_blocks = re.findall(
        r'(\w+) \{\s+name: "([^"]+)",\s+matcher_fn: [^,]+,\s+priority: (\d+),\s+category: [^,]+,\s+jlpt: "([^"]+)"',
        content,
        re.MULTILINE
    )

    patterns = []
    for variant, name, priority, jlpt in pattern_blocks:
        patterns.append({
            "variant": variant,
            "pattern_name": name,
            "jlpt_level": jlpt,
            "priority_in_code": int(priority)
        })

    return patterns

# Check test files for pattern tests
def check_test_files(pattern_name):
    test_dirs = [
        "grammar-lib/src/tests/n5_patterns.rs",
        "grammar-lib/src/tests/n4_patterns.rs",
        "grammar-lib/src/tests/n3_patterns.rs",
        "grammar-lib/src/tests/n2_patterns.rs",
        "grammar-lib/src/tests/n1_patterns.rs",
    ]

    test_info = {
        "exists": False,
        "has_range_assertion": False,
        "has_print_debug": False
    }

    for test_file in test_dirs:
        if not os.path.exists(test_file):
            continue

        with open(test_file) as f:
            content = f.read()

        # Check if pattern is mentioned in tests
        # Look for assert_has_pattern or assert_pattern_range with this pattern name
        if f'"{pattern_name}"' in content:
            test_info["exists"] = True

            # Check for assert_pattern_range
            if f'assert_pattern_range(&patterns, "{pattern_name}"' in content:
                test_info["has_range_assertion"] = True

            # Check for print_debug
            if 'print_debug' in content and pattern_name in content:
                test_info["has_print_debug"] = True

    return test_info

# Main audit
def main():
    patterns = extract_patterns_from_source()
    print(f"Extracted {len(patterns)} patterns from patterns.rs")

    # Audit each pattern
    feature_list = []
    for p in patterns:
        test_info = check_test_files(p["pattern_name"])

        # Determine if tests pass (all criteria met)
        passes = (
            test_info["exists"] and
            test_info["has_range_assertion"] and
            not test_info["has_print_debug"]
        )

        # Determine category - for now, assume all need implementation
        # We'll manually mark some as complete or skipped
        category = "needs_implementation"

        # Calculate priority based on JLPT level
        jlpt_priority_map = {
            "n5": 1,
            "n4": 2,
            "n3": 3,
            "n2": 4,
            "n1": 5,
            "nt": 6  # NT = Not tested level
        }

        priority = jlpt_priority_map.get(p["jlpt_level"], None)

        feature_list.append({
            "pattern_name": p["pattern_name"],
            "jlpt_level": p["jlpt_level"],
            "category": category,
            "tests": test_info,
            "passes": passes,
            "description": "",  # Will be filled in manually
            "priority": priority,
            "skip_reason": None
        })

    # Sort by priority (None values go to end)
    feature_list.sort(key=lambda x: (x["priority"] is None, x["priority"] or 999, x["pattern_name"]))

    # Write to JSON
    output_file = "feature_list.json"
    with open(output_file, "w", encoding="utf-8") as f:
        json.dump(feature_list, f, indent=2, ensure_ascii=False)

    print(f"✓ Written {len(feature_list)} patterns to {output_file}")

    # Print summary
    total = len(feature_list)
    passing = sum(1 for p in feature_list if p["passes"])
    has_tests = sum(1 for p in feature_list if p["tests"]["exists"])
    has_range = sum(1 for p in feature_list if p["tests"]["has_range_assertion"])

    print(f"\nSummary:")
    print(f"  Total patterns: {total}")
    print(f"  Has tests: {has_tests}")
    print(f"  Has range assertions: {has_range}")
    print(f"  Passing all criteria: {passing}")

    # By JLPT level
    for level in ["n5", "n4", "n3", "n2", "n1", "nt"]:
        level_patterns = [p for p in feature_list if p["jlpt_level"] == level]
        level_passing = sum(1 for p in level_patterns if p["passes"])
        print(f"  {level.upper()}: {level_passing}/{len(level_patterns)} passing")

if __name__ == "__main__":
    main()
