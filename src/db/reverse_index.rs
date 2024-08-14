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

impl GetWord for csv::StringRecord {
    fn get_word<'a>(
        &self,
        statements: &mut WordStatements<'a>,
        word_cache: &mut HashMap<String, String>,
    ) -> String {
        let target_column = self
            .get(5)
            .filter(|&s| s != "-1")
            .or_else(|| self.get(1))
            .unwrap_or("");
        let conj_id = self.get(4).filter(|&s| s != "-1");
        let seq = self.get(6).unwrap_or("");

        if let Some(cached_word) = word_cache.get(target_column) {
            return cached_word.clone();
        }

        let result = if let Some(id) = conj_id {
            statements
                .get_word_conj
                .query_row(params![target_column, id], |row| row.get::<_, String>(0))
        } else {
            statements
                .get_word_no_conj
                .query_row(params![target_column, seq], |row| row.get::<_, String>(0))
        };

        let word = result.unwrap_or_else(|_| target_column.to_string());
        word_cache.insert(target_column.to_string(), word.clone());
        word
    }

    fn get_reading<'a>(
        &self,
        statements: &mut WordStatements<'a>,
        reading_cache: &mut HashMap<String, String>,
    ) -> String {
        let seq = self.get(6).unwrap_or("");

        if let Some(cached_reading) = reading_cache.get(seq) {
            return cached_reading.clone();
        }

        let result = statements
            .get_reading
            .query_row(params![seq], |row| row.get::<_, String>("text"));

        let reading = result.unwrap_or_else(|_| self.get(2).unwrap_or("").to_string());
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
        get_reading: jmdict_conn.prepare("SELECT text FROM kana_text WHERE seq = ?1")?,
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
    jmdict_conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_conj_source_reading_text ON conj_source_reading(text)",
        [],
    )?;
    jmdict_conn.execute("CREATE INDEX IF NOT EXISTS idx_conj_source_reading_conj_id ON conj_source_reading(conj_id)", [])?;
    jmdict_conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_conjugation_id ON conjugation(id)",
        [],
    )?;
    jmdict_conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_conjugation_seq ON conjugation(seq)",
        [],
    )?;
    jmdict_conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_kana_text_seq ON kana_text(seq)",
        [],
    )?;
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
