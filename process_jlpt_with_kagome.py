#!/usr/bin/env python3
import csv
import json
import subprocess
from collections import defaultdict

def tokenize_batch(words_with_levels):
    """Send a batch of words to Kagome and get tokenization results."""
    if not words_with_levels:
        return []

    # Create input text (one word per line)
    text = '\n'.join([word for word, _ in words_with_levels])

    try:
        # Run kagome with UTF-8 handling
        result = subprocess.run(
            ['kagome', '-json'],
            input=text.encode('utf-8'),
            capture_output=True,
            check=False
        )

        if result.returncode != 0:
            print(f"Kagome error: {result.stderr.decode('utf-8', errors='ignore')}")
            return words_with_levels

        output = result.stdout.decode('utf-8', errors='ignore').strip()
        if not output:
            return words_with_levels

        tokenized_results = []

        # Split output into JSON arrays (one per input line)
        # Each array spans from '[' to ']', possibly across multiple lines
        json_arrays = []
        bracket_depth = 0
        current = []

        for line in output.split('\n'):
            current.append(line)
            bracket_depth += line.count('[') - line.count(']')
            if bracket_depth == 0 and current:
                json_arrays.append('\n'.join(current))
                current = []

        # Process each word's tokenization results
        for i, json_str in enumerate(json_arrays):
            if i >= len(words_with_levels):
                break

            original_word, level = words_with_levels[i]

            try:
                tokens = json.loads(json_str)

                # Extract non-punctuation tokens, excluding "/"
                valid_tokens = [
                    token['surface']
                    for token in tokens
                    if isinstance(token, dict)
                    and token.get('pos', [None])[0] != '記号'
                    and 'surface' in token
                    and token['surface'] != '/'
                ]

                # Add tokens or keep original if no valid tokens found
                if valid_tokens:
                    for surface in valid_tokens:
                        tokenized_results.append((surface, level))
                else:
                    tokenized_results.append((original_word, level))

            except (json.JSONDecodeError, KeyError):
                tokenized_results.append((original_word, level))

        # Add any remaining unprocessed words
        for i in range(len(json_arrays), len(words_with_levels)):
            tokenized_results.append(words_with_levels[i])

        return tokenized_results

    except Exception as e:
        print(f"Error processing with Kagome: {e}")
        return words_with_levels

def main():
    input_file = 'src-tauri/jlpt_levels.csv'
    output_file = 'src-tauri/jlpt_levels_parsed.csv'

    print(f"Reading {input_file}...")

    # Read all rows at once
    with open(input_file, 'r', encoding='utf-8') as f:
        rows = list(csv.DictReader(f))

    print(f"Total rows: {len(rows)}")

    # Process in chunks of 500
    words_dict = {}
    chunk_size = 500

    for i in range(0, len(rows), chunk_size):
        chunk = rows[i:i+chunk_size]
        chunk_data = [(row['word'], int(row['level'])) for row in chunk]

        print(f"Processing chunk {i//chunk_size + 1} (rows {i+1} to {min(i+chunk_size, len(rows))})...")

        # Tokenize and add to dictionary
        for word, level in tokenize_batch(chunk_data):
            if word:
                # Keep highest level number (5 is easier than 1)
                words_dict[word] = max(words_dict.get(word, 0), level)

        print(f"  Processed {i + len(chunk)} rows, {len(words_dict)} unique words so far")

    print(f"\nTotal unique words: {len(words_dict)}")

    # Sort by level (descending) then alphabetically
    sorted_words = sorted(words_dict.items(), key=lambda x: (-x[1], x[0]))

    # Write output
    print(f"Writing to {output_file}...")
    with open(output_file, 'w', encoding='utf-8', newline='') as f:
        writer = csv.writer(f)
        writer.writerow(['word', 'level'])
        writer.writerows(sorted_words)

    print(f"Done! Output written to {output_file}")

    # Print statistics
    level_counts = defaultdict(int)
    for _, level in sorted_words:
        level_counts[level] += 1

    print("\nWords per level:")
    for level in sorted(level_counts.keys(), reverse=True):
        print(f"  Level {level}: {level_counts[level]} words")

if __name__ == '__main__':
    main()