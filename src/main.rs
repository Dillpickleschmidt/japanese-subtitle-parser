/*
1. Read all transcripts_raw files and add entries to sqlite database transcripts table following example.json
2. Create a parser that tokenizes Japanese text.
3. Create a reverse index by extracting all unique words from the transcripts_raw files storing them in a new
table called words. Create another table storing every word (1 each row), with the transcript_id in the second
column as a foreign key to the transcripts table.
4. Create a search function that uses parsed input text and finds transcript lines that contain all parsed
words using the reverse index (match input words to words table -> transcript_id).
  4.1. Get the show_name, season, episode_number, line_id, & transcript_id of the parsed text.
  4.2. Using the show_name, season, episide_number, and line_id, get the text for 5 previous lines (if they
  exist) and 2 next lines. Output a json object with the following structure:
[
  "item 1": [
    {
      "id": "3476"
      "ts_num": "-5",
      "text": "text",
    },
    {
      "id": "3477"
      "ts_num": "-4",
      "text": "text"
    },
    ...
    {
      "id": "3481"
      "ts_num": "0",
      "text": "text"
    },
    {
      "id": "3482"
      "ts_num": "1",
      "text": "text"
    },
    ...
  ],
  ...
]
5. Feed the LLM with the output of the search function. Tell it it's looking for the 10 most interesting/memorable
ts_num 0 lines, returned in descending order of interest/memorability. Use the surrounding lines as context in
determening that. Return just the ids in your output.
6. Search the transcripts table for those 10 ids and return the full text of the matching lines.
*/

mod db;
mod error;
mod srt_parser;

use db::DbHandler;
use error::Error;
use srt_parser::{process_srt_directory, EpisodeNameMethod, EpisodeNumberMethod};
use std::path::Path;
use std::time::Instant;

fn main() -> Result<(), Error> {
    let start_time = Instant::now();

    let mut db = DbHandler::new("transcripts.db")?;
    db.create_tables()?;

    let root_dir = Path::new("data/transcripts_raw");
    let number_method = EpisodeNumberMethod::FromFileOrder;
    let name_method = EpisodeNameMethod::FromEpisodeNumber;

    let show_entries = process_srt_directory(root_dir, &number_method, &name_method);
    println!(
        "Processed {} entries.",
        show_entries.values().flatten().count()
    );

    // Prepare data for batch insertion
    let mut shows = Vec::new();
    let mut episodes = Vec::new();
    let mut transcripts = Vec::new();

    for (show_name, show_episodes) in show_entries {
        shows.push((show_name.clone(), "Anime".to_string()));
        let show_id = shows.len() as i64;

        for episode in show_episodes {
            episodes.push((
                show_id,
                episode.episode_name.clone(),
                1, // Assuming all episodes are in season 1
                episode.episode_number as i32,
            ));
            let episode_id = episodes.len() as i64;

            for subtitle in episode.content.0.iter() {
                transcripts.push((
                    episode_id,
                    subtitle.number as i32,
                    subtitle.start_time.to_string(),
                    subtitle.end_time.to_string(),
                    subtitle.text.clone(),
                ));
            }
        }
    }

    // Perform batch insertions
    db.insert_shows(&shows)?;
    println!("Shows inserted successfully.");

    let episode_ids = db.insert_episodes(&episodes)?;
    println!("Episodes inserted successfully.");

    let transcript_ids = db.insert_transcripts(&transcripts)?;
    println!("Transcripts inserted successfully.");

    // Create reverse index
    db.create_reverse_index("parsed_transcripts.csv")?;
    println!("Reverse index created successfully.");

    // Example of using complex search
    // let results = db.complex_search("一番")?;
    // println!("{:?}", results);

    let duration = start_time.elapsed();
    println!("Total execution time: {:?}", duration);
    Ok(())
}
