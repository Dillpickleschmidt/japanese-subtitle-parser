use crate::db::show::Show;
use crate::db::DbHandler;
use crate::error::Error;
use rusqlite::params;

/// Represents an episode of a TV show in the database
#[derive(Debug)]
pub struct Episode {
    pub id: Option<i64>,
    pub show_id: i64,
    pub name: String,
    pub season: i32,
    pub episode_number: i32,
}

impl Episode {
    /// Creates a new Episode instance
    pub fn new(show_id: i64, name: String, season: i32, episode_number: i32) -> Self {
        Episode {
            id: None,
            show_id,
            name,
            season,
            episode_number,
        }
    }

    /// Inserts the episode into the database
    pub fn insert(&mut self, db: &DbHandler) -> Result<(), Error> {
        let conn = db.get_connection();
        conn.execute(
            "INSERT INTO episodes (show_id, name, season, episode_number) VALUES (?1, ?2, ?3, ?4)",
            params![self.show_id, self.name, self.season, self.episode_number],
        )?;
        self.id = Some(conn.last_insert_rowid());
        Ok(())
    }

    /// Updates the episode in the database
    pub fn update(&self, db: &DbHandler) -> Result<(), Error> {
        let conn = db.get_connection();
        conn.execute(
            "UPDATE episodes SET show_id = ?1, name = ?2, season = ?3, episode_number = ?4 WHERE id = ?5",
            params![self.show_id, self.name, self.season, self.episode_number, self.id],
        )?;
        Ok(())
    }

    /// Deletes the episode from the database
    pub fn delete(&self, db: &DbHandler) -> Result<(), Error> {
        let conn = db.get_connection();
        conn.execute("DELETE FROM episodes WHERE id = ?1", params![self.id])?;
        Ok(())
    }

    /// Retrieves an episode from the database by ID
    pub fn get_by_id(db: &DbHandler, id: i64) -> Result<Episode, Error> {
        let conn = db.get_connection();
        let mut stmt = conn.prepare(
            "SELECT id, show_id, name, season, episode_number FROM episodes WHERE id = ?1",
        )?;
        let episode = stmt.query_row(params![id], |row| {
            Ok(Episode {
                id: Some(row.get(0)?),
                show_id: row.get(1)?,
                name: row.get(2)?,
                season: row.get(3)?,
                episode_number: row.get(4)?,
            })
        })?;
        Ok(episode)
    }

    /// Retrieves all episodes for a specific show
    pub fn get_all_for_show(db: &DbHandler, show_id: i64) -> Result<Vec<Episode>, Error> {
        let conn = db.get_connection();
        let mut stmt = conn.prepare("SELECT id, show_id, name, season, episode_number FROM episodes WHERE show_id = ?1 ORDER BY season, episode_number")?;
        let episodes_iter = stmt.query_map(params![show_id], |row| {
            Ok(Episode {
                id: Some(row.get(0)?),
                show_id: row.get(1)?,
                name: row.get(2)?,
                season: row.get(3)?,
                episode_number: row.get(4)?,
            })
        })?;

        let mut episodes = Vec::new();
        for episode in episodes_iter {
            episodes.push(episode?);
        }
        Ok(episodes)
    }

    /// Searches for episodes by name
    pub fn search_by_name(db: &DbHandler, search_term: &str) -> Result<Vec<Episode>, Error> {
        let conn = db.get_connection();
        let mut stmt = conn.prepare(
            "SELECT id, show_id, name, season, episode_number FROM episodes WHERE name LIKE ?1",
        )?;
        let episodes_iter = stmt.query_map(params![format!("%{}%", search_term)], |row| {
            Ok(Episode {
                id: Some(row.get(0)?),
                show_id: row.get(1)?,
                name: row.get(2)?,
                season: row.get(3)?,
                episode_number: row.get(4)?,
            })
        })?;

        let mut episodes = Vec::new();
        for episode in episodes_iter {
            episodes.push(episode?);
        }
        Ok(episodes)
    }

    /// Retrieves an episode from the database by show_id, season, and episode_number
    pub fn get_by_show_season_episode(
        db: &DbHandler,
        show_id: i64,
        season: i32,
        episode_number: i32,
    ) -> Result<Episode, Error> {
        let conn = db.get_connection();
        let mut stmt = conn.prepare(
            "SELECT id, show_id, name, season, episode_number 
             FROM episodes 
             WHERE show_id = ?1 AND season = ?2 AND episode_number = ?3",
        )?;
        let episode = stmt.query_row(params![show_id, season, episode_number], |row| {
            Ok(Episode {
                id: Some(row.get(0)?),
                show_id: row.get(1)?,
                name: row.get(2)?,
                season: row.get(3)?,
                episode_number: row.get(4)?,
            })
        })?;
        Ok(episode)
    }

    /// Gets the associated Show for this Episode
    pub fn get_show(&self, db: &DbHandler) -> Result<Show, Error> {
        Show::get_by_id(db, self.show_id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::show::Show;
    use tempfile::NamedTempFile;

    fn create_test_db() -> (NamedTempFile, DbHandler) {
        let file = NamedTempFile::new().unwrap();
        let handler = DbHandler::new(file.path().to_str().unwrap()).unwrap();
        handler.create_tables().unwrap();
        (file, handler)
    }

    fn create_test_show(db: &DbHandler) -> Show {
        let mut show = Show::new("Test Show".to_string(), "Anime".to_string());
        show.insert(db).unwrap();
        show
    }

    #[test]
    fn test_insert_and_get_episode() {
        let (_file, db) = create_test_db();
        let show = create_test_show(&db);

        let mut episode = Episode::new(show.id.unwrap(), "Test Episode".to_string(), 1, 1);
        episode.insert(&db).unwrap();

        assert!(episode.id.is_some());

        let retrieved_episode = Episode::get_by_id(&db, episode.id.unwrap()).unwrap();
        assert_eq!(retrieved_episode.name, "Test Episode");
        assert_eq!(retrieved_episode.season, 1);
        assert_eq!(retrieved_episode.episode_number, 1);
    }

    #[test]
    fn test_update_episode() {
        let (_file, db) = create_test_db();
        let show = create_test_show(&db);

        let mut episode = Episode::new(show.id.unwrap(), "Test Episode".to_string(), 1, 1);
        episode.insert(&db).unwrap();

        episode.name = "Updated Episode".to_string();
        episode.update(&db).unwrap();

        let updated_episode = Episode::get_by_id(&db, episode.id.unwrap()).unwrap();
        assert_eq!(updated_episode.name, "Updated Episode");
    }

    #[test]
    fn test_delete_episode() {
        let (_file, db) = create_test_db();
        let show = create_test_show(&db);

        let mut episode = Episode::new(show.id.unwrap(), "Test Episode".to_string(), 1, 1);
        episode.insert(&db).unwrap();

        episode.delete(&db).unwrap();

        let result = Episode::get_by_id(&db, episode.id.unwrap());
        assert!(result.is_err());
    }

    #[test]
    fn test_get_all_for_show() {
        let (_file, db) = create_test_db();
        let show = create_test_show(&db);

        let episodes = vec![
            Episode::new(show.id.unwrap(), "Episode 1".to_string(), 1, 1),
            Episode::new(show.id.unwrap(), "Episode 2".to_string(), 1, 2),
            Episode::new(show.id.unwrap(), "Episode 3".to_string(), 1, 3),
        ];

        for mut episode in episodes {
            episode.insert(&db).unwrap();
        }

        let retrieved_episodes = Episode::get_all_for_show(&db, show.id.unwrap()).unwrap();
        assert_eq!(retrieved_episodes.len(), 3);
        assert_eq!(retrieved_episodes[0].episode_number, 1);
        assert_eq!(retrieved_episodes[1].episode_number, 2);
        assert_eq!(retrieved_episodes[2].episode_number, 3);
    }

    #[test]
    fn test_search_episodes() {
        let (_file, db) = create_test_db();
        let show = create_test_show(&db);

        let episodes = vec![
            Episode::new(show.id.unwrap(), "Pilot Episode".to_string(), 1, 1),
            Episode::new(show.id.unwrap(), "Regular Episode".to_string(), 1, 2),
            Episode::new(show.id.unwrap(), "Finale Episode".to_string(), 1, 3),
        ];

        for mut episode in episodes {
            episode.insert(&db).unwrap();
        }

        let search_results = Episode::search_by_name(&db, "Episode").unwrap();
        assert_eq!(search_results.len(), 3);

        let search_results = Episode::search_by_name(&db, "Pilot").unwrap();
        assert_eq!(search_results.len(), 1);
        assert_eq!(search_results[0].name, "Pilot Episode");
    }

    #[test]
    fn test_get_show_for_episode() {
        let (_file, db) = create_test_db();
        let show = create_test_show(&db);

        let mut episode = Episode::new(show.id.unwrap(), "Test Episode".to_string(), 1, 1);
        episode.insert(&db).unwrap();

        let retrieved_show = episode.get_show(&db).unwrap();
        assert_eq!(retrieved_show.id, show.id);
        assert_eq!(retrieved_show.name, show.name);
    }
}
