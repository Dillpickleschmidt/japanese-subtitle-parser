use crate::error::Error;
use crate::kagome::extract_words_from_transcripts;
use rusqlite::{params, Connection, Transaction};
use std::collections::{HashMap, HashSet};

pub fn create_reverse_index(conn: &mut Connection) -> Result<(), Error> {
    println!("Creating reverse index using kagome...");

    // Get all transcript data from the database
    let transcript_data = get_all_transcript_data(conn)?;
    println!(
        "Retrieved {} transcripts from database",
        transcript_data.len()
    );

    // Process transcripts with kagome to extract words
    let word_map = extract_words_from_transcripts(&transcript_data)?;
    println!("Extracted {} unique words from transcripts", word_map.len());

    // Create indexes on the main database
    create_main_indexes(conn)?;

    // Batch insert into the database
    let tx = conn.transaction()?;
    batch_insert_words_and_occurrences(&tx, word_map)?;
    tx.commit()?;

    println!("Reverse index created successfully!");

    // Import JLPT levels and compute stats if CSV file exists
    process_jlpt_data(conn)?;

    Ok(())
}

fn get_all_transcript_data(conn: &Connection) -> Result<Vec<(i64, String)>, Error> {
    let mut stmt = conn.prepare("SELECT id, text FROM transcripts")?;
    let transcript_iter = stmt.query_map([], |row| {
        let id: i64 = row.get(0)?;
        let text: String = row.get(1)?;
        Ok((id, text))
    })?;

    let mut transcripts = Vec::new();
    for transcript in transcript_iter {
        transcripts.push(transcript?);
    }

    Ok(transcripts)
}

fn batch_insert_words_and_occurrences(
    tx: &Transaction,
    word_map: HashMap<(String, String, Vec<String>), HashSet<i64>>,
) -> Result<(), Error> {
    let mut stmt_word =
        tx.prepare("INSERT OR IGNORE INTO words (word, reading, pos) VALUES (?1, ?2, ?3)")?;
    let mut stmt_get_word_id = tx.prepare("SELECT id FROM words WHERE word = ?1")?;
    let mut stmt_occurrence = tx.prepare(
        "INSERT OR IGNORE INTO word_occurrences (word_id, transcript_id) VALUES (?1, ?2)",
    )?;

    for ((word, reading, pos), transcript_ids) in word_map {
        let pos_json = serde_json::to_string(&pos)
            .map_err(|e| Error::Other(format!("Failed to serialize POS: {}", e)))?;
        stmt_word.execute(params![&word, &reading, &pos_json])?;
        let word_id: i64 = stmt_get_word_id.query_row(params![&word], |row| row.get(0))?;

        for &transcript_id in &transcript_ids {
            stmt_occurrence.execute(params![word_id, transcript_id])?;
        }
    }

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

fn process_jlpt_data(conn: &mut Connection) -> Result<(), Error> {
    use std::fs::File;
    use std::io::{BufRead, BufReader};

    if !std::path::Path::new("jlpt_levels.csv").exists() {
        println!("JLPT levels CSV not found, skipping JLPT processing");
        return Ok(());
    }

    println!("Processing JLPT data...");
    let tx = conn.transaction()?;

    // Import CSV
    let reader = BufReader::new(File::open("jlpt_levels.csv")?);
    for line in reader.lines() {
        let line = line?;
        if line.trim().is_empty() || line.starts_with("word,") {
            continue;
        }

        if let Some((word, level)) = line.split_once(',') {
            let level: i32 = level.trim().parse().unwrap_or(0);
            if level >= 1 && level <= 5 {
                tx.execute(
                    "INSERT OR REPLACE INTO jlpt_levels (word, level) VALUES (?, ?)",
                    [word.trim(), &level.to_string()],
                )?;
            }
        }
    }

    // Compute stats
    tx.execute("DELETE FROM episode_jlpt_stats", [])?;
    tx.execute(
        "
        INSERT INTO episode_jlpt_stats (episode_id, n5_pct, n4_pct, n3_pct, n2_pct, n1_pct)
        SELECT 
            e.id,
            100.0 * SUM(CASE WHEN jl.level = 5 THEN 1.0 ELSE 0 END) / COUNT(w.id) as n5_pct,
            100.0 * SUM(CASE WHEN jl.level >= 4 THEN 1.0 ELSE 0 END) / COUNT(w.id) as n4_pct,
            100.0 * SUM(CASE WHEN jl.level >= 3 THEN 1.0 ELSE 0 END) / COUNT(w.id) as n3_pct,
            100.0 * SUM(CASE WHEN jl.level >= 2 THEN 1.0 ELSE 0 END) / COUNT(w.id) as n2_pct,
            100.0 * SUM(CASE WHEN jl.level >= 1 THEN 1.0 ELSE 0 END) / COUNT(w.id) as n1_pct
        FROM episodes e
        JOIN transcripts t ON t.episode_id = e.id
        JOIN word_occurrences wo ON wo.transcript_id = t.id
        JOIN words w ON w.id = wo.word_id
        LEFT JOIN jlpt_levels jl ON jl.word = w.word
        WHERE NOT (
            -- Proper nouns
            (JSON_EXTRACT(w.pos, '$[0]') = '名詞' AND JSON_EXTRACT(w.pos, '$[1]') = '固有名詞')
            -- Fillers, others, interjections
            OR JSON_EXTRACT(w.pos, '$[0]') IN ('フィラー', 'その他')
            OR (JSON_EXTRACT(w.pos, '$[0]') = '感動詞' AND JSON_EXTRACT(w.pos, '$[1]') = '間投')
            -- Numbers
            OR (JSON_EXTRACT(w.pos, '$[0]') = '名詞' AND JSON_EXTRACT(w.pos, '$[1]') = '数')
        )
        GROUP BY e.id
    ",
        [],
    )?;

    tx.commit()?;
    println!("JLPT processing completed!");

    // Debug: Show filtering and distribution info
    debug_jlpt_filtering(conn)?;

    Ok(())
}

fn debug_jlpt_filtering(conn: &Connection) -> Result<(), Error> {
    println!("\n=== JLPT Filtering Debug Info ===");

    // Total words before filtering
    let total_words: i32 = conn.query_row("SELECT COUNT(*) FROM words", [], |row| row.get(0))?;
    println!("Total words in database: {}", total_words);

    // Words after POS filtering
    let filtered_words: i32 = conn.query_row(
        "SELECT COUNT(*) FROM words w 
         WHERE NOT (
             (JSON_EXTRACT(w.pos, '$[0]') = '名詞' AND JSON_EXTRACT(w.pos, '$[1]') = '固有名詞')
             OR JSON_EXTRACT(w.pos, '$[0]') IN ('フィラー', 'その他')
             OR (JSON_EXTRACT(w.pos, '$[0]') = '感動詞' AND JSON_EXTRACT(w.pos, '$[1]') = '間投')
             OR (JSON_EXTRACT(w.pos, '$[0]') = '名詞' AND JSON_EXTRACT(w.pos, '$[1]') = '数')
         )",
        [],
        |row| row.get(0),
    )?;
    println!(
        "Words after POS filtering: {} ({:.1}%)",
        filtered_words,
        filtered_words as f64 / total_words as f64 * 100.0
    );

    // Words with JLPT levels
    let jlpt_words: i32 = conn.query_row(
        "SELECT COUNT(*) FROM words w 
         JOIN jlpt_levels jl ON jl.word = w.word
         WHERE NOT (
             (JSON_EXTRACT(w.pos, '$[0]') = '名詞' AND JSON_EXTRACT(w.pos, '$[1]') = '固有名詞')
             OR JSON_EXTRACT(w.pos, '$[0]') IN ('フィラー', 'その他')
             OR (JSON_EXTRACT(w.pos, '$[0]') = '感動詞' AND JSON_EXTRACT(w.pos, '$[1]') = '間投')
             OR (JSON_EXTRACT(w.pos, '$[0]') = '名詞' AND JSON_EXTRACT(w.pos, '$[1]') = '数')
         )",
        [],
        |row| row.get(0),
    )?;
    println!(
        "Words with JLPT levels (after filtering): {} ({:.1}%)",
        jlpt_words,
        jlpt_words as f64 / filtered_words as f64 * 100.0
    );

    // JLPT level distribution
    println!("\nJLPT Level Distribution:");
    let mut stmt = conn.prepare(
        "SELECT jl.level, COUNT(*) as count
         FROM words w 
         JOIN jlpt_levels jl ON jl.word = w.word
         WHERE NOT (
             (JSON_EXTRACT(w.pos, '$[0]') = '名詞' AND JSON_EXTRACT(w.pos, '$[1]') = '固有名詞')
             OR JSON_EXTRACT(w.pos, '$[0]') IN ('フィラー', 'その他')
             OR (JSON_EXTRACT(w.pos, '$[0]') = '感動詞' AND JSON_EXTRACT(w.pos, '$[1]') = '間投')
             OR (JSON_EXTRACT(w.pos, '$[0]') = '名詞' AND JSON_EXTRACT(w.pos, '$[1]') = '数')
         )
         GROUP BY jl.level ORDER BY jl.level DESC",
    )?;

    let level_rows =
        stmt.query_map([], |row| Ok((row.get::<_, i32>(0)?, row.get::<_, i32>(1)?)))?;

    for row in level_rows {
        let (level, count) = row?;
        println!(
            "  N{}: {} words ({:.1}%)",
            level,
            count,
            count as f64 / jlpt_words as f64 * 100.0
        );
    }

    // Sample words that don't have JLPT levels
    println!("\nSample words WITHOUT JLPT levels (after filtering):");
    let mut sample_stmt = conn.prepare(
        "SELECT w.word, JSON_EXTRACT(w.pos, '$[0]') as pos1, JSON_EXTRACT(w.pos, '$[1]') as pos2
         FROM words w 
         LEFT JOIN jlpt_levels jl ON jl.word = w.word
         WHERE jl.word IS NULL
         AND NOT (
             (JSON_EXTRACT(w.pos, '$[0]') = '名詞' AND JSON_EXTRACT(w.pos, '$[1]') = '固有名詞')
             OR JSON_EXTRACT(w.pos, '$[0]') IN ('フィラー', 'その他')
             OR (JSON_EXTRACT(w.pos, '$[0]') = '感動詞' AND JSON_EXTRACT(w.pos, '$[1]') = '間投')
             OR (JSON_EXTRACT(w.pos, '$[0]') = '名詞' AND JSON_EXTRACT(w.pos, '$[1]') = '数')
         )
         LIMIT 20",
    )?;

    let sample_rows = sample_stmt.query_map([], |row| {
        Ok((
            row.get::<_, String>(0)?,
            row.get::<_, String>(1)?,
            row.get::<_, String>(2)?,
        ))
    })?;

    for row in sample_rows {
        let (word, pos1, pos2) = row?;
        println!("  {} ({}, {})", word, pos1, pos2);
    }

    // POS category breakdown for words without JLPT levels
    println!("\nPOS breakdown of words WITHOUT JLPT levels (after filtering):");
    let mut pos_stmt = conn.prepare(
        "SELECT JSON_EXTRACT(w.pos, '$[0]') as pos1, COUNT(*) as count
         FROM words w 
         LEFT JOIN jlpt_levels jl ON jl.word = w.word
         WHERE jl.word IS NULL
         AND NOT (
             (JSON_EXTRACT(w.pos, '$[0]') = '名詞' AND JSON_EXTRACT(w.pos, '$[1]') = '固有名詞')
             OR JSON_EXTRACT(w.pos, '$[0]') IN ('フィラー', 'その他')
             OR (JSON_EXTRACT(w.pos, '$[0]') = '感動詞' AND JSON_EXTRACT(w.pos, '$[1]') = '間投')
             OR (JSON_EXTRACT(w.pos, '$[0]') = '名詞' AND JSON_EXTRACT(w.pos, '$[1]') = '数')
         )
         GROUP BY pos1 ORDER BY count DESC LIMIT 10",
    )?;

    let pos_rows = pos_stmt.query_map([], |row| {
        Ok((row.get::<_, String>(0)?, row.get::<_, i32>(1)?))
    })?;

    for row in pos_rows {
        let (pos, count) = row?;
        println!("  {}: {} words", pos, count);
    }

    println!("=== End Debug Info ===\n");
    Ok(())
}
