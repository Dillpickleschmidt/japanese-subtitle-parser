use crate::analysis::kagome_server::KagomeServer;
use crate::analysis::unified_analyzer::analyze_batch;
use crate::error::Error;
use rusqlite::{Connection, Transaction};
use std::collections::{HashMap, HashSet};
use std::sync::LazyLock;

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

    let server = KagomeServer::start()?;

    println!("Processing transcripts and analyzing grammar patterns...");

    let total_transcripts: i64 =
        conn.query_row("SELECT COUNT(*) FROM transcripts", [], |row| row.get(0))?;
    println!(
        "Processing {} total transcripts with streaming...",
        total_transcripts
    );

    let batch_size = 1000;
    let mut all_words = HashMap::new(); // Store raw words first (no corrections yet)
    let mut all_grammar_patterns = HashMap::new();

    let mut stmt =
        conn.prepare("SELECT id, episode_id, text FROM transcripts ORDER BY episode_id, line_id")?;
    let transcript_iter = stmt.query_map([], |row| {
        Ok((
            row.get::<_, i64>(0)?,    // transcript_id
            row.get::<_, i32>(1)?,    // episode_id
            row.get::<_, String>(2)?, // text
        ))
    })?;

    let mut batch = Vec::with_capacity(batch_size);
    let mut batch_count = 0;
    let total_batches = (total_transcripts as usize / batch_size) + 1;

    for transcript_result in transcript_iter {
        let transcript = transcript_result?;
        batch.push(transcript);

        if batch.len() >= batch_size {
            batch_count += 1;
            println!("Processing batch {}/{}", batch_count, total_batches);

            let results = analyze_batch(&batch, &server)?;

            for (word_key, transcript_ids) in results.words {
                all_words
                    .entry((word_key.base_form, word_key.reading, word_key.pos))
                    .or_insert_with(HashSet::new)
                    .extend(transcript_ids);
            }

            for (episode_id, collector) in results.grammar_patterns {
                all_grammar_patterns
                    .entry(episode_id)
                    .or_insert_with(Vec::new)
                    .push(collector);
            }

            batch.clear(); // Free memory of this batch
        }
    }

    if !batch.is_empty() {
        batch_count += 1;
        println!("Processing final batch {}/{}", batch_count, total_batches);

        let results = analyze_batch(&batch, &server)?;

        for (word_key, transcript_ids) in results.words {
            all_words
                .entry((word_key.base_form, word_key.reading, word_key.pos))
                .or_insert_with(HashSet::new)
                .extend(transcript_ids);
        }

        for (episode_id, collector) in results.grammar_patterns {
            all_grammar_patterns
                .entry(episode_id)
                .or_insert_with(Vec::new)
                .push(collector);
        }
    }

    drop(stmt);

    println!(
        "Applying reading corrections to {} unique words...",
        all_words.len()
    );

    let unique_base_forms: HashSet<String> = all_words
        .keys()
        .map(|(base_form, _, _)| base_form.clone())
        .collect();

    let base_forms_vec: Vec<&str> = unique_base_forms.iter().map(|s| s.as_str()).collect();
    let reading_corrections =
        crate::analysis::morphology::get_base_form_readings(&base_forms_vec, &server)?;

    let mut all_corrected_words = HashMap::new();
    for ((base_form, reading, pos), transcript_ids) in all_words {
        let final_reading = reading_corrections
            .get(base_form.as_str())
            .cloned()
            .unwrap_or(reading);

        all_corrected_words
            .entry((base_form, final_reading, pos))
            .or_insert_with(HashSet::new)
            .extend(transcript_ids);
    }

    let tx = conn.transaction()?;

    create_main_indexes_tx(&tx)?;

    println!("Processing grammar pattern occurrences...");

    let mut all_pattern_occurrences = Vec::new();
    let mut pattern_names = std::collections::HashSet::new();

    for (_episode_id, collectors) in all_grammar_patterns {
        for collector in collectors {
            for (pattern_name, transcript_id, confidence, start_char, end_char) in
                collector.occurrences
            {
                pattern_names.insert(pattern_name.clone());
                all_pattern_occurrences.push((
                    pattern_name,
                    transcript_id,
                    confidence,
                    start_char,
                    end_char,
                ));
            }
        }
    }

    // Show a preview of pattern matches
    if !all_pattern_occurrences.is_empty() {
        println!("\nSample grammar pattern matches:");
        let sample_size = 3.min(all_pattern_occurrences.len());

        for i in 0..sample_size {
            let (pattern_name, transcript_id, _confidence, start_char, end_char) =
                &all_pattern_occurrences[i];

            // Query transcript text
            if let Ok(text) = tx.query_row(
                "SELECT text FROM transcripts WHERE id = ?",
                [transcript_id],
                |row| row.get::<_, String>(0),
            ) {
                // Convert character positions to byte positions
                let byte_start = text
                    .char_indices()
                    .nth(*start_char as usize)
                    .map(|(pos, _)| pos)
                    .unwrap_or(0);
                let byte_end = text
                    .char_indices()
                    .nth(*end_char as usize)
                    .map(|(pos, _)| pos)
                    .unwrap_or(text.len());

                let matched_text = &text[byte_start..byte_end];
                println!(
                    "  - Pattern: {}, Transcript ID: {}\n    Matched text: \"{}\"",
                    pattern_name, transcript_id, matched_text
                );
            }
        }
        println!();
    }

    let mut pattern_id_cache = std::collections::HashMap::new();
    for pattern_name in pattern_names {
        let jlpt_level = grammar_lib::get_jlpt_level(&pattern_name);
        let pattern_id =
            crate::db::grammar_pattern::get_or_create_pattern_id(&tx, &pattern_name, jlpt_level)?;
        pattern_id_cache.insert(pattern_name, pattern_id);
    }

    let final_occurrences: Vec<_> = all_pattern_occurrences
        .into_iter()
        .map(
            |(pattern_name, transcript_id, confidence, start_char, end_char)| {
                let pattern_id = pattern_id_cache[&pattern_name];
                crate::db::grammar_pattern::GrammarPatternOccurrence::new(
                    pattern_id,
                    transcript_id,
                    confidence,
                    start_char,
                    end_char,
                )
            },
        )
        .collect();

    let total_pattern_occurrences = final_occurrences.len();

    if !final_occurrences.is_empty() {
        println!(
            "Inserting {} grammar pattern occurrences...",
            total_pattern_occurrences
        );
        crate::db::grammar_pattern::GrammarPatternOccurrence::bulk_insert_optimized(
            &final_occurrences,
            &tx,
        )?;
    }

    batch_insert_words_and_occurrences(&tx, &all_corrected_words)?;

    println!(
        "Total pattern occurrences inserted: {}",
        total_pattern_occurrences
    );

    tx.commit()?;

    server.shutdown()?;

    println!("Reverse index created successfully!");

    process_jlpt_data(conn)?;

    Ok(())
}

fn batch_insert_words_and_occurrences(
    tx: &Transaction,
    word_map: &HashMap<(String, String, Vec<String>), HashSet<i64>>,
) -> Result<(), Error> {
    let word_keys: Vec<_> = word_map.keys().collect();
    for chunk in word_keys.chunks(1000) {
        let placeholders: Vec<String> = chunk.iter().map(|_| "(?, ?, ?)".to_string()).collect();
        let sql = format!(
            "INSERT OR IGNORE INTO words (word, reading, pos) VALUES {}",
            placeholders.join(", ")
        );

        let mut params = Vec::new();
        for (word, reading, pos) in chunk {
            let pos_json = if let Some(cached) = POS_CACHE.get(pos) {
                cached.as_str()
            } else {
                &serde_json::to_string(pos).unwrap()
            };

            params.push(word.clone());
            params.push(reading.clone());
            params.push(pos_json.to_string());
        }

        tx.execute(&sql, rusqlite::params_from_iter(params))?;
    }

    let mut stmt_get_word_id = tx.prepare("SELECT id FROM words WHERE word = ?")?;

    for ((word, _, _), transcript_ids) in word_map {
        let word_id: i64 = stmt_get_word_id.query_row([word], |row| row.get(0))?;

        let transcript_vec: Vec<_> = transcript_ids.iter().collect();
        for chunk in transcript_vec.chunks(1000) {
            let placeholders: Vec<String> = chunk.iter().map(|_| "(?, ?)".to_string()).collect();
            let sql = format!(
                "INSERT OR IGNORE INTO word_occurrences (word_id, transcript_id) VALUES {}",
                placeholders.join(", ")
            );

            let mut params = Vec::new();
            for &&transcript_id in chunk {
                params.push(word_id.to_string());
                params.push(transcript_id.to_string());
            }

            tx.execute(&sql, rusqlite::params_from_iter(params))?;
        }
    }

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

    let reader = BufReader::new(File::open("jlpt_levels.csv")?);
    for line in reader.lines() {
        let line = line?;
        if line.trim().is_empty() || line.starts_with("word,") {
            continue;
        }

        if let Some((word, level)) = line.split_once(',') {
            let level: i32 = level.trim().parse().unwrap_or(0);
            if (1..=5).contains(&level) {
                tx.execute(
                    "INSERT OR REPLACE INTO jlpt_levels (word, level) VALUES (?, ?)",
                    [word.trim(), &level.to_string()],
                )?;
            }
        }
    }

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
