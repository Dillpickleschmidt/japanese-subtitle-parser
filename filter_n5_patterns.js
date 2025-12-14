import fs from 'fs';
import path from 'path';
import { fileURLToPath } from 'url';

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);

// Read the CSV file to get N5 patterns
const csvPath = path.join(__dirname, 'grammar_points_jlpt.csv');
const csvContent = fs.readFileSync(csvPath, 'utf-8');

// Parse CSV and filter N5 patterns
const n5Patterns = new Set();
const lines = csvContent.split('\n');

for (let i = 1; i < lines.length; i++) { // Skip header
  const line = lines[i].trim();
  if (!line) continue;

  const [grammarPoint, jlptLevel] = line.split(',');
  if (jlptLevel === 'N5') {
    n5Patterns.add(grammarPoint);
  }
}

console.log(`Found ${n5Patterns.size} N5 patterns`);

// Read the JSON data file
const jsonPath = path.join(__dirname, 'grammar_points_data.json');
const allData = JSON.parse(fs.readFileSync(jsonPath, 'utf-8'));

// Filter to only N5 patterns
const n5Data = {};
for (const [key, value] of Object.entries(allData)) {
  if (n5Patterns.has(key)) {
    n5Data[key] = value;
  }
}

console.log(`Filtered to ${Object.keys(n5Data).length} N5 entries`);

// Write to new file
const outputPath = path.join(__dirname, 'grammar_points_n5_data.json');
fs.writeFileSync(outputPath, JSON.stringify(n5Data, null, 2), 'utf-8');

console.log(`Written to ${outputPath}`);
