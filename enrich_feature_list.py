#!/usr/bin/env python3
"""
Enrich feature_list.json with descriptions from grammar_points_data.json
"""
import json
import re
from html.parser import HTMLParser

class HTMLTextExtractor(HTMLParser):
    """Extract text from HTML, ignoring tags"""
    def __init__(self):
        super().__init__()
        self.text = []

    def handle_data(self, data):
        self.text.append(data.strip())

    def get_text(self):
        return ' '.join(filter(None, self.text))

def extract_first_sentence(html_text):
    """Extract the first meaningful sentence from HTML"""
    parser = HTMLTextExtractor()
    parser.feed(html_text)
    text = parser.get_text()

    # Get first sentence (up to first period or first 100 chars)
    sentences = text.split('.')
    if sentences:
        first = sentences[0].strip()
        if len(first) > 120:
            first = first[:117] + "..."
        return first
    return text[:100] + "..." if len(text) > 100 else text

def main():
    # Load grammar data
    with open("grammar_points_data.json") as f:
        grammar_data = json.load(f)

    # Load feature list
    with open("feature_list.json") as f:
        feature_list = json.load(f)

    print(f"Loaded {len(feature_list)} patterns")
    print(f"Loaded {len(grammar_data)} grammar entries")

    # Enrich each pattern
    enriched = 0
    for pattern in feature_list:
        pattern_name = pattern["pattern_name"]

        # Try to find matching grammar data
        if pattern_name in grammar_data:
            data = grammar_data[pattern_name]

            # Extract description from 'about' field
            if "about" in data and data["about"]:
                desc = extract_first_sentence(data["about"])
                pattern["description"] = desc
                enriched += 1

            # Count structure variants
            if "structure" in data:
                struct = data["structure"]
                standard_count = len(struct.get("standard", []))
                polite_count = len(struct.get("polite", []))

                # Add note about structure variants
                if standard_count > 0 or polite_count > 0:
                    if not pattern["description"]:
                        pattern["description"] = f"[{standard_count} standard, {polite_count} polite forms]"
                    else:
                        pattern["description"] += f" [{standard_count} std, {polite_count} pol]"

    # Fill in missing descriptions with basic info
    for pattern in feature_list:
        if not pattern["description"]:
            pattern["description"] = f"{pattern['jlpt_level'].upper()} pattern"

    # Save enriched feature list
    with open("feature_list.json", "w", encoding="utf-8") as f:
        json.dump(feature_list, f, indent=2, ensure_ascii=False)

    print(f"✓ Enriched {enriched}/{len(feature_list)} patterns with descriptions")

    # Print sample
    print("\nSample patterns:")
    for p in feature_list[:5]:
        print(f"  - {p['pattern_name']}: {p['description'][:80]}")

if __name__ == "__main__":
    main()
