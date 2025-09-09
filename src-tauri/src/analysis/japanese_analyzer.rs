use crate::analysis::unified_analyzer::UnifiedAnalyzer;
use crate::error::Error;
use rusqlite::{params, Connection, Transaction};
use std::collections::{HashMap, HashSet};
use std::sync::LazyLock;

// Pre-serialized common POS patterns for performance
static POS_CACHE: LazyLock<HashMap<Vec<String>, String>> = LazyLock::new(|| {
    let common_patterns = vec![
        vec!["名詞".to_string(), "一般".to_string()],
        vec!["動詞".to_string(), "自立".to_string()],
        vec!["動詞".to_string(), "非自立".to_string()],
        vec!["形容詞".to_string(), "自立".to_string()],
        vec!["副詞".to_string(), "一般".to_string()],
        vec!["助詞".to_string(), "格助詞".to_string()],
        vec!["助詞".to_string(), "係助詞".to_string()],
        vec!["助動詞".to_string()],
        vec!["記号".to_string(), "句点".to_string()],
        vec!["記号".to_string(), "読点".to_string()],
    ];

    common_patterns
        .into_iter()
        .map(|pos| {
            let serialized = serde_json::to_string(&pos).unwrap();
            (pos, serialized)
        })
        .collect()
});

pub fn create_reverse_index(conn: &mut Connection) -> Result<(), Error> {
    println!("Creating reverse index and analyzing grammar patterns...");

    // Create unified analyzer and process in streaming mode
    let analyzer = UnifiedAnalyzer::new();
    let analysis_results = analyzer.process_transcripts_streaming_collect(conn, 1000)?;

    // Start transaction for database operations
    let tx = conn.transaction()?;

    // Create indexes first (they benefit from being in the same transaction)
    create_main_indexes_tx(&tx)?;

    // Combine all unique words from analysis results
    println!("Combining all unique words...");
    let mut all_words = HashMap::new();

    for results in analysis_results.iter() {
        for (word_key, transcript_ids) in &results.words {
            all_words
                .entry((
                    word_key.base_form.clone(),
                    word_key.reading.clone(),
                    word_key.pos.clone(),
                ))
                .or_insert_with(HashSet::new)
                .extend(transcript_ids);
        }
    }

    println!(
        "Extracted {} unique words from transcripts",
        all_words.len()
    );

    // Apply reading corrections to all words at once
    println!("Correcting base form readings...");
    let unique_base_forms: Vec<&str> = all_words
        .keys()
        .map(|(base_form, _, _)| base_form.as_str())
        .collect();
    let reading_corrections =
        crate::analysis::morphology::get_base_form_readings(&unique_base_forms)?;

    // Apply corrections to all words
    let mut corrected_words = HashMap::new();
    for ((base_form, old_reading, pos), transcript_ids) in all_words {
        let final_reading = reading_corrections
            .get(base_form.as_str())
            .cloned()
            .unwrap_or(old_reading);
        corrected_words.insert((base_form, final_reading, pos), transcript_ids);
    }

    println!(
        "Applied reading corrections to {} words",
        corrected_words.len()
    );

    // Single database insert for all words
    batch_insert_words_and_occurrences(&tx, &corrected_words)?;

    // Process grammar patterns from all batches inside the transaction
    println!("Processing grammar patterns...");
    let mut total_pattern_occurrences = 0;
    for results in analysis_results {
        for (_episode_id, collector) in results.grammar_patterns {
            let occurrences = collector.into_occurrences(&tx)?;
            if !occurrences.is_empty() {
                use crate::db::grammar_pattern::GrammarPatternOccurrence;
                GrammarPatternOccurrence::batch_insert(&occurrences, &tx)?;
                total_pattern_occurrences += occurrences.len();
            }
        }
    }
    println!(
        "Total pattern occurrences inserted: {}",
        total_pattern_occurrences
    );

    // Compute statistics immediately after inserts
    compute_word_statistics(&tx)?;

    // Single commit for all operations
    tx.commit()?;

    println!("Reverse index created successfully!");

    // Import JLPT levels and compute stats if CSV file exists
    process_jlpt_data(conn)?;

    Ok(())
}

fn batch_insert_words_and_occurrences(
    tx: &Transaction,
    word_map: &HashMap<(String, String, Vec<String>), HashSet<i64>>,
) -> Result<(), Error> {
    let mut stmt_word =
        tx.prepare("INSERT OR IGNORE INTO words (word, reading, pos) VALUES (?1, ?2, ?3)")?;
    let mut stmt_get_word_id = tx.prepare("SELECT id FROM words WHERE word = ?1")?;
    let mut stmt_occurrence = tx.prepare(
        "INSERT OR IGNORE INTO word_occurrences (word_id, transcript_id) VALUES (?1, ?2)",
    )?;

    for ((word, reading, pos), transcript_ids) in word_map {
        // Use cached POS serialization if available, otherwise serialize on demand
        let pos_json_owned;
        let pos_json = if let Some(cached) = POS_CACHE.get(pos) {
            cached.as_str()
        } else {
            pos_json_owned = serde_json::to_string(pos).unwrap();
            &pos_json_owned
        };

        stmt_word.execute(params![word, reading, &pos_json])?;
        let word_id: i64 = stmt_get_word_id.query_row(params![word], |row| row.get(0))?;

        for &transcript_id in transcript_ids {
            stmt_occurrence.execute(params![word_id, transcript_id])?;
        }
    }

    Ok(())
}

fn compute_word_statistics(tx: &Transaction) -> Result<(), Error> {
    println!("Computing word statistics...");

    // Check if this is a fresh database or an update
    let existing_stats: i64 = tx
        .query_row("SELECT COUNT(*) FROM word_stats", [], |row| row.get(0))
        .unwrap_or(0);

    if existing_stats == 0 {
        // Fresh database - use bulk insert approach
        compute_word_statistics_bulk(tx)
    } else {
        // Incremental update - only update what changed
        compute_word_statistics_incremental(tx)
    }
}

fn compute_word_statistics_bulk(tx: &Transaction) -> Result<(), Error> {
    // Clear existing statistics (should be empty for fresh DB)
    tx.execute("DELETE FROM word_stats", [])?;
    tx.execute("DELETE FROM word_episodes", [])?;

    // Populate word_stats table with occurrence counts and episode counts
    tx.execute(
        "
        INSERT INTO word_stats (word_id, total_occurrences, episode_count)
        SELECT 
            w.id,
            COUNT(wo.transcript_id) as total_occurrences,
            COUNT(DISTINCT t.episode_id) as episode_count
        FROM words w
        LEFT JOIN word_occurrences wo ON wo.word_id = w.id
        LEFT JOIN transcripts t ON t.id = wo.transcript_id
        GROUP BY w.id
    ",
        [],
    )?;

    // Populate word_episodes table with per-episode occurrence counts
    tx.execute(
        "
        INSERT INTO word_episodes (word_id, episode_id, occurrence_count)
        SELECT 
            wo.word_id,
            t.episode_id,
            COUNT(*) as occurrence_count
        FROM word_occurrences wo
        JOIN transcripts t ON t.id = wo.transcript_id
        GROUP BY wo.word_id, t.episode_id
    ",
        [],
    )?;

    // Compute frequency ranks using window function (100x faster than correlated subquery)
    tx.execute(
        "
        WITH ranked AS (
            SELECT word_id,
                   ROW_NUMBER() OVER (ORDER BY total_occurrences DESC) as rank
            FROM word_stats
        )
        UPDATE word_stats
        SET frequency_rank = (
            SELECT rank FROM ranked WHERE ranked.word_id = word_stats.word_id
        )
    ",
        [],
    )?;

    println!("Word statistics computed successfully (bulk)!");
    Ok(())
}

fn compute_word_statistics_incremental(tx: &Transaction) -> Result<(), Error> {
    // For incremental updates, we need to:
    // 1. Find words that are new (not in word_stats)
    // 2. Update statistics for new words only
    // 3. Recalculate frequency ranks efficiently

    // Insert stats for new words only
    tx.execute(
        "
        INSERT OR IGNORE INTO word_stats (word_id, total_occurrences, episode_count)
        SELECT 
            w.id,
            COUNT(wo.transcript_id) as total_occurrences,
            COUNT(DISTINCT t.episode_id) as episode_count
        FROM words w
        LEFT JOIN word_occurrences wo ON wo.word_id = w.id
        LEFT JOIN transcripts t ON t.id = wo.transcript_id
        LEFT JOIN word_stats ws ON ws.word_id = w.id
        WHERE ws.word_id IS NULL
        GROUP BY w.id
    ",
        [],
    )?;

    // Insert episode stats for new words only
    tx.execute(
        "
        INSERT OR IGNORE INTO word_episodes (word_id, episode_id, occurrence_count)
        SELECT 
            wo.word_id,
            t.episode_id,
            COUNT(*) as occurrence_count
        FROM word_occurrences wo
        JOIN transcripts t ON t.id = wo.transcript_id
        LEFT JOIN word_episodes we ON we.word_id = wo.word_id AND we.episode_id = t.episode_id
        WHERE we.word_id IS NULL
        GROUP BY wo.word_id, t.episode_id
    ",
        [],
    )?;

    // Update frequency ranks using window function (much more efficient)
    tx.execute(
        "
        WITH ranked AS (
            SELECT word_id,
                   ROW_NUMBER() OVER (ORDER BY total_occurrences DESC) as rank
            FROM word_stats
        )
        UPDATE word_stats
        SET frequency_rank = (
            SELECT rank FROM ranked WHERE ranked.word_id = word_stats.word_id
        )
        WHERE frequency_rank IS NULL
    ",
        [],
    )?;

    println!("Word statistics updated successfully (incremental)!");
    Ok(())
}

fn create_main_indexes_tx(tx: &Transaction) -> Result<(), Error> {
    tx.execute(
        "CREATE INDEX IF NOT EXISTS idx_words_word_reading ON words(word, reading)",
        [],
    )?;
    tx.execute(
        "CREATE INDEX IF NOT EXISTS idx_word_occurrences_word_id ON word_occurrences(word_id)",
        [],
    )?;
    tx.execute("CREATE INDEX IF NOT EXISTS idx_word_occurrences_transcript_id ON word_occurrences(transcript_id)", [])?;
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
