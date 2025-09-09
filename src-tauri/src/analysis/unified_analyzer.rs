use crate::analysis::morphology::KagomeToken;
use crate::db::grammar_pattern::GrammarPatternCollector;
use crate::error::Error;
use crate::grammar::{
    create_genki_pattern_matcher, pattern_matcher::PatternMatcher, types::ConjugationPattern,
};
use rusqlite::Connection;
use std::collections::{HashMap, HashSet};
use std::sync::{LazyLock, Mutex};

// Static pattern matcher - created once and reused for all transcripts
static GENKI_MATCHER: LazyLock<PatternMatcher<ConjugationPattern>> =
    LazyLock::new(|| create_genki_pattern_matcher());

// POS tag interning cache for memory efficiency
static POS_CACHE: LazyLock<Mutex<HashMap<Vec<String>, Vec<String>>>> =
    LazyLock::new(|| Mutex::new(HashMap::new()));

fn intern_pos_tags(pos: &[String]) -> Vec<String> {
    let mut cache = POS_CACHE.lock().unwrap();

    // Check if we already have this exact POS combination
    if let Some(cached) = cache.get(pos) {
        return cached.clone();
    }

    // Store new POS combination in cache
    let owned_pos = pos.to_vec();
    cache.insert(owned_pos.clone(), owned_pos.clone());
    owned_pos
}

/// Efficient hash-friendly key for word deduplication
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct WordKey {
    pub base_form: String,
    pub reading: String,
    pub pos: Vec<String>,
}

impl WordKey {
    pub fn new(base_form: String, reading: String, pos: Vec<String>) -> Self {
        Self {
            base_form,
            reading,
            pos,
        }
    }
}

/// Unified analyzer that processes Kagome tokens once for all analysis types
pub struct UnifiedAnalyzer;

/// Combined results from unified analysis
#[derive(Debug)]
pub struct UnifiedAnalysisResult {
    pub words: HashMap<WordKey, HashSet<i64>>, // word key -> transcript_ids
    pub grammar_patterns: HashMap<i32, GrammarPatternCollector>, // episode_id -> collector
}

impl UnifiedAnalyzer {
    pub fn new() -> Self {
        UnifiedAnalyzer
    }

    /// Process a batch of transcripts with unified analysis
    pub fn analyze_batch(
        &self,
        batch: &[(i64, i32, String)],
    ) -> Result<UnifiedAnalysisResult, Error> {
        // Combine all transcript texts for single Kagome call
        let combined_text = batch
            .iter()
            .map(|(_, _, text)| text.as_str())
            .collect::<Vec<_>>()
            .join("\n");

        // Single Kagome parsing pass
        let token_arrays = crate::analysis::morphology::process_text_with_kagome(&combined_text)?;

        // Optimized pre-allocation based on realistic estimates
        // Average Japanese transcript has ~15-20 content words, aim for ~70% load factor
        let estimated_word_capacity = (batch.len() * 18) * 10 / 7; // More accurate + load factor
        let mut words = HashMap::with_capacity(estimated_word_capacity);
        // Better estimate for grammar collectors: typical batch spans fewer episodes
        let estimated_episodes = (batch.len() / 20).max(1); // ~20 transcripts per episode
        let mut grammar_collectors = HashMap::with_capacity(estimated_episodes);

        // Process each transcript's tokens
        for (line_idx, &(transcript_id, episode_id, _)) in batch.iter().enumerate() {
            if let Some(tokens) = token_arrays.get(line_idx) {
                if !tokens.is_empty() {
                    // Extract words
                    self.extract_words_from_tokens(tokens, transcript_id, &mut words);

                    // Analyze grammar patterns
                    let collector = grammar_collectors
                        .entry(episode_id)
                        .or_insert_with(GrammarPatternCollector::new);

                    self.analyze_grammar_patterns(tokens, collector, transcript_id);
                }
            }
        }

        Ok(UnifiedAnalysisResult {
            words,
            grammar_patterns: grammar_collectors,
        })
    }

    fn extract_words_from_tokens(
        &self,
        tokens: &[KagomeToken],
        transcript_id: i64,
        word_map: &mut HashMap<WordKey, HashSet<i64>>,
    ) {
        for token in tokens {
            if self.is_content_word(token) {
                // Use HashMap entry API for efficient insertion/update - O(1) instead of O(n)
                // Use interned POS tags to reduce memory usage for common patterns
                let key = WordKey::new(
                    token.base_form.clone(),
                    token.reading.clone(),
                    intern_pos_tags(&token.pos),
                );

                word_map
                    .entry(key)
                    .or_insert_with(HashSet::new)
                    .insert(transcript_id);
            }
        }
    }

    fn analyze_grammar_patterns(
        &self,
        tokens: &[KagomeToken],
        collector: &mut GrammarPatternCollector,
        transcript_id: i64,
    ) {
        // Use static pattern matcher - created once, reused for all transcripts
        let genki_matches = GENKI_MATCHER.match_tokens(tokens);

        for pattern_match in genki_matches {
            // Simply add the pattern occurrence with transcript ID
            collector.add_pattern(
                pattern_match.pattern_name.to_string(),
                transcript_id,
                pattern_match.confidence.into(),
            );
        }
    }

    fn is_content_word(&self, token: &KagomeToken) -> bool {
        if token.pos.is_empty() {
            return false;
        }

        let pos = &token.pos[0];

        // Filter out symbols/punctuation completely
        if pos == "記号" {
            return false;
        }

        matches!(pos.as_str(), "名詞" | "動詞" | "形容詞" | "副詞" | "感動詞")
    }

    /// Stream processing with callback for handling results
    /// Returns collected results instead of processing inline to avoid borrowing issues
    pub fn process_transcripts_streaming_collect(
        &self,
        conn: &Connection,
        batch_size: usize,
    ) -> Result<Vec<UnifiedAnalysisResult>, Error> {
        // First, count total transcripts for progress reporting
        let total_transcripts: i64 =
            conn.query_row("SELECT COUNT(*) FROM transcripts", [], |row| row.get(0))?;

        println!("Processing {} total transcripts...", total_transcripts);

        let mut stmt = conn
            .prepare("SELECT id, episode_id, text FROM transcripts ORDER BY episode_id, line_id")?;

        let transcript_iter = stmt.query_map([], |row| {
            Ok((
                row.get::<_, i64>(0)?,    // transcript_id
                row.get::<_, i32>(1)?,    // episode_id
                row.get::<_, String>(2)?, // text
            ))
        })?;

        let mut batch = Vec::with_capacity(batch_size);
        // Pre-allocate results vector based on estimated number of batches
        let estimated_batches = (total_transcripts as usize / batch_size) + 1;
        let mut all_results = Vec::with_capacity(estimated_batches);
        let mut total_processed = 0;

        for transcript_result in transcript_iter {
            let transcript = transcript_result?;
            batch.push(transcript);

            if batch.len() >= batch_size {
                // Process full batch
                let results = self.analyze_batch(&batch)?;
                all_results.push(results);

                total_processed += batch.len();
                let progress_pct =
                    (total_processed as f64 / total_transcripts as f64 * 100.0) as i32;
                println!(
                    "Processed {} of {} transcripts ({}% complete)",
                    total_processed, total_transcripts, progress_pct
                );

                batch.clear();
            }
        }

        // Process remaining transcripts
        if !batch.is_empty() {
            let results = self.analyze_batch(&batch)?;
            all_results.push(results);
            total_processed += batch.len();
            println!(
                "Processed {} of {} transcripts (100% complete - final batch)",
                total_processed, total_transcripts
            );
        }

        Ok(all_results)
    }
}

impl Default for UnifiedAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}
