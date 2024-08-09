use crate::error::Error;
use csv;
use rusqlite::{params, Connection, Transaction};
use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;

pub fn create_reverse_index(conn: &mut Connection, csv_path: &str) -> Result<(), Error> {
    // Open the CSV file
    let file = File::open(csv_path)?;
    let reader = BufReader::new(file);

    // Create a CSV reader with headers
    let mut csv_reader = csv::ReaderBuilder::new()
        .has_headers(true)
        .from_reader(reader);

    // Initialize a HashMap to store words and their occurrences
    // Key: (word, reading), Value: Vec of transcript IDs
    let mut word_map: HashMap<(String, String), Vec<i64>> = HashMap::new();

    // Iterate over each record in the CSV file
    for (index, record) in csv_reader.records().enumerate() {
        // Handle potential CSV parsing errors
        let record = record.map_err(|e| Error::InvalidInput(format!("CSV error: {}", e)))?;

        // Ensure each record has at least 3 fields
        if record.len() < 3 {
            return Err(Error::InvalidInput(format!(
                "Invalid record at line {}: insufficient fields",
                index + 2 // Add 2 to account for 0-indexing and header row
            )));
        }

        // Parse the transcript ID, handling potential parsing errors
        let transcript_id: i64 = record.get(0).unwrap_or("").parse().map_err(|e| {
            Error::InvalidInput(format!(
                "Invalid transcript_id at line {}: {}",
                index + 2,
                e
            ))
        })?;

        // Insert the word into the hash map, or update the existing entry
        // The key is a tuple of (word, reading)
        // The value is a Vec of transcript IDs where this word appears
        word_map
            .entry((
                record.get(1).unwrap_or("").to_string(), // word
                record.get(2).unwrap_or("").to_string(), // reading
            ))
            .or_insert_with(Vec::new)
            .push(transcript_id);
    }

    // Start a database transaction
    let tx = conn.transaction()?;

    // Insert words and their occurrences into the database
    insert_words_and_occurrences(&tx, word_map)?;

    // Commit the transaction
    tx.commit()?;

    Ok(())
}

fn insert_words_and_occurrences(
    tx: &Transaction,
    word_map: HashMap<(String, String), Vec<i64>>,
) -> Result<(), Error> {
    let mut stmt_word =
        tx.prepare("INSERT OR IGNORE INTO words (word, reading) VALUES (?1, ?2)")?;
    let mut stmt_get_word_id =
        tx.prepare("SELECT id FROM words WHERE word = ?1 AND reading = ?2")?;
    let mut stmt_occurrence = tx.prepare(
        "INSERT OR IGNORE INTO word_occurrences (word_id, transcript_id) VALUES (?1, ?2)",
    )?;

    for ((word, reading), transcript_ids) in word_map {
        // Try to insert the word
        stmt_word.execute(params![&word, &reading])?;

        // Fetch the word_id (whether it was just inserted or already existed)
        let word_id: i64 =
            stmt_get_word_id.query_row(params![&word, &reading], |row| row.get(0))?;

        for &transcript_id in &transcript_ids {
            stmt_occurrence.execute(params![word_id, transcript_id])?;
        }
    }

    Ok(())
}
