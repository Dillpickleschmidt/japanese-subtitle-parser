use crate::error::Error;
use csv;
use rusqlite::{params, Connection, Statement, Transaction};
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::BufReader;

struct WordStatements<'a> {
    get_word_conj: Statement<'a>,
    get_word_no_conj: Statement<'a>,
    get_reading: Statement<'a>,
}

trait GetWord {
    fn get_word<'a>(
        &self,
        statements: &mut WordStatements<'a>,
        word_cache: &mut HashMap<String, String>,
    ) -> String;
    fn get_reading<'a>(
        &self,
        statements: &mut WordStatements<'a>,
        reading_cache: &mut HashMap<String, String>,
    ) -> String;
}

/// Implementation of the GetWord trait for csv::StringRecord.
///
/// This implementation provides methods to retrieve words and readings from a database
/// based on information in a CSV record. It uses a caching mechanism to improve performance
/// for repeated lookups.
///
/// The CSV structure is expected to be:
/// Transcript Number, Word, Reading, Seq, Conjugation Number, Truetext
///
/// Process for get_word:
/// 1. Attempts to use Truetext (index 5), falling back to Word (index 1) if Truetext is "-1"
/// 2. Checks the word cache for a previously retrieved result
/// 3. If not in cache, queries the database:
///    - Uses a conjugation-specific query if Conjugation Number (index 4) is present
///    - Otherwise, uses a non-conjugation query with Seq (index 3)
/// 4. Falls back to the original word from the CSV if the database query fails
/// 5. Caches the result for future use
///
/// Process for get_reading:
/// 1. Uses Seq (index 3) to identify the reading
/// 2. Checks the reading cache for a previously retrieved result
/// 3. If not in cache, queries the database using Seq
/// 4. Falls back to the original Reading (index 2) from the CSV if the database query fails
/// 5. Caches the result for future use
///
/// Both methods use prepared SQL statements for efficient database queries and
/// implement fallback strategies to ensure a result is always returned.
impl GetWord for csv::StringRecord {
    fn get_word<'a>(
        &self,
        statements: &mut WordStatements<'a>,
        word_cache: &mut HashMap<String, String>,
    ) -> String {
        // Get the target word from Truetext (index 5) if it's not "-1", otherwise use Word (index 1)
        let target_column = self
            .get(5)
            .filter(|&s| s != "-1")
            .or_else(|| self.get(1))
            .unwrap_or("");

        // Get the conjugation ID from Conjugation Number (index 4) if it's not "-1"
        let conj_id = self.get(4).filter(|&s| s != "-1");

        // Get the sequence number from Seq (index 3)
        let seq = self.get(3).unwrap_or("");

        // Check if the word is already in the cache
        if let Some(cached_word) = word_cache.get(target_column) {
            return cached_word.clone();
        }

        // Query the database based on whether we have a conjugation ID or not
        let result = if let Some(id) = conj_id {
            // Use the conjugation-specific query
            statements
                .get_word_conj
                .query_row(params![target_column, id], |row| row.get::<_, String>(0))
        } else {
            // Use the non-conjugation query
            statements
                .get_word_no_conj
                .query_row(params![target_column, seq], |row| row.get::<_, String>(0))
        };

        // If the query fails, fall back to the original word from the CSV
        let word = result.unwrap_or_else(|_| target_column.to_string());

        // Cache the result for future use
        word_cache.insert(target_column.to_string(), word.clone());

        word
    }

    /// Retrieves the reading from the database or cache based on the CSV record
    fn get_reading<'a>(
        &self,
        statements: &mut WordStatements<'a>,
        reading_cache: &mut HashMap<String, String>,
    ) -> String {
        // Get the sequence number from Seq (index 3)
        let seq = self.get(3).unwrap_or("");

        // Check if the reading is already in the cache
        if let Some(cached_reading) = reading_cache.get(seq) {
            return cached_reading.clone();
        }

        // Query the database for the reading
        let result = statements
            .get_reading
            .query_row(params![seq], |row| row.get::<_, String>(0));

        // If the query fails, fall back to the original reading from Reading (index 2)
        let reading = match result {
            Ok(r) => r,
            Err(_) => self.get(2).unwrap_or("").to_string(), // Fallback to original reading
        };

        // Cache the result for future use
        reading_cache.insert(seq.to_string(), reading.clone());

        reading
    }
}

pub fn create_reverse_index(
    conn: &mut Connection,
    csv_path: &str,
    jmdict_conn: &mut Connection,
) -> Result<(), Error> {
    // Create indexes on the JMDict database
    create_jmdict_indexes(jmdict_conn)?;

    let mut statements = WordStatements {
        get_word_conj: jmdict_conn.prepare(
            "SELECT source_text FROM conj_source_reading WHERE text = ?1 AND conj_id = ?2",
        )?,
        get_word_no_conj: jmdict_conn.prepare(
            "
            SELECT csr.source_text 
            FROM conj_source_reading csr
            JOIN conjugation c ON csr.conj_id = c.id
            WHERE csr.text = ?1 AND c.seq = ?2
        ",
        )?,
        get_reading: jmdict_conn.prepare(
            "
            SELECT kt.text
            FROM conjugation c
            JOIN kana_text kt ON kt.seq = c.\"from\"
            WHERE c.seq = ?1
        ",
        )?,
    };

    let file = File::open(csv_path)?;
    let reader = BufReader::new(file);
    let mut csv_reader = csv::ReaderBuilder::new()
        .has_headers(true)
        .from_reader(reader);

    let mut word_cache = HashMap::new();
    let mut reading_cache = HashMap::new();
    let mut word_map: HashMap<(String, String), HashSet<i64>> = HashMap::new();

    for (index, result) in csv_reader.records().enumerate() {
        let record = result
            .map_err(|e| Error::InvalidInput(format!("CSV error at line {}: {}", index + 2, e)))?;

        if record.len() < 5 {
            return Err(Error::InvalidInput(format!(
                "Invalid record at line {}: insufficient fields",
                index + 2
            )));
        }

        let transcript_id: i64 = record.get(0).unwrap_or("").parse().map_err(|e| {
            Error::InvalidInput(format!(
                "Invalid transcript_id at line {}: {}",
                index + 2,
                e
            ))
        })?;

        let word = record.get_word(&mut statements, &mut word_cache);
        let reading = record.get_reading(&mut statements, &mut reading_cache);

        word_map
            .entry((word, reading))
            .or_insert_with(HashSet::new)
            .insert(transcript_id);
    }

    // Create indexes on the main database
    create_main_indexes(conn)?;

    // Batch insert into the database
    let tx = conn.transaction()?;
    batch_insert_words_and_occurrences(&tx, word_map)?;
    tx.commit()?;

    Ok(())
}

fn batch_insert_words_and_occurrences(
    tx: &Transaction,
    word_map: HashMap<(String, String), HashSet<i64>>,
) -> Result<(), Error> {
    let mut stmt_word =
        tx.prepare("INSERT OR IGNORE INTO words (word, reading) VALUES (?1, ?2)")?;
    let mut stmt_get_word_id =
        tx.prepare("SELECT id FROM words WHERE word = ?1 AND reading = ?2")?;
    let mut stmt_occurrence = tx.prepare(
        "INSERT OR IGNORE INTO word_occurrences (word_id, transcript_id) VALUES (?1, ?2)",
    )?;

    for ((word, reading), transcript_ids) in word_map {
        stmt_word.execute(params![&word, &reading])?;
        let word_id: i64 =
            stmt_get_word_id.query_row(params![&word, &reading], |row| row.get(0))?;

        for &transcript_id in &transcript_ids {
            stmt_occurrence.execute(params![word_id, transcript_id])?;
        }
    }

    Ok(())
}

fn create_jmdict_indexes(jmdict_conn: &mut Connection) -> Result<(), Error> {
    // Indexes for conj_source_reading table
    jmdict_conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_conj_source_reading_text ON conj_source_reading(text)",
        [],
    )?;
    jmdict_conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_conj_source_reading_conj_id ON conj_source_reading(conj_id)",
        [],
    )?;

    // Indexes for conjugation table
    jmdict_conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_conjugation_id ON conjugation(id)",
        [],
    )?;
    jmdict_conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_conjugation_seq ON conjugation(seq)",
        [],
    )?;
    jmdict_conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_conjugation_from ON conjugation(\"from\")",
        [],
    )?;

    // Indexes for kana_text table
    jmdict_conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_kana_text_seq ON kana_text(seq)",
        [],
    )?;

    // Indexes for kanji_text table
    jmdict_conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_kanji_text_text ON kanji_text(text)",
        [],
    )?;
    jmdict_conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_kanji_text_seq ON kanji_text(seq)",
        [],
    )?;

    Ok(())
}

fn create_main_indexes(conn: &mut Connection) -> Result<(), Error> {
    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_words_word_reading ON words(word, reading)",
        [],
    )?;
    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_word_occurrences_word_id ON word_occurrences(word_id)",
        [],
    )?;
    conn.execute("CREATE INDEX IF NOT EXISTS idx_word_occurrences_transcript_id ON word_occurrences(transcript_id)", [])?;
    Ok(())
}
