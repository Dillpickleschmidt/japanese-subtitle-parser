use crate::error::Error;
use rusqlite::{params, Connection};

#[cfg(test)]
use crate::db::show::Show;

/// Represents an episode of a TV show in the database
#[derive(Debug)]
pub struct Episode {
    pub id: Option<i32>,
    pub show_id: i32,
    pub name: String,
    pub episode_number: Option<i32>,
}

impl Episode {
    /// Creates a new Episode instance
    pub fn new(show_id: i32, name: String, episode_number: Option<i32>) -> Self {
        Episode {
            id: None,
            show_id,
            name,
            episode_number,
        }
    }

    /// Inserts the episode into the database
    pub fn insert(&mut self, conn: &Connection) -> Result<(), Error> {
        conn.execute(
            "INSERT OR IGNORE INTO episodes (show_id, name, episode_number) VALUES (?1, ?2, ?3)",
            params![self.show_id, self.name, self.episode_number],
        )?;
        // Convert the last inserted row id to i32 and assign it to the episode's id field
        crate::db::model::set_id_from_last_insert(&mut self.id, conn);
        Ok(())
    }

    /// Updates the episode in the database
    #[cfg(test)]
    pub fn update(&self, conn: &Connection) -> Result<(), Error> {
        conn.execute(
            "UPDATE episodes SET show_id = ?1, name = ?2, episode_number = ?3 WHERE id = ?4",
            params![self.show_id, self.name, self.episode_number, self.id],
        )?;
        Ok(())
    }

    /// Deletes the episode from the database
    #[cfg(test)]
    pub fn delete(&self, conn: &Connection) -> Result<(), Error> {
        conn.execute("DELETE FROM episodes WHERE id = ?1", params![self.id])?;
        Ok(())
    }

    /// Retrieves an episode from the database by ID
    pub fn get_by_id(conn: &Connection, id: i32) -> Result<Episode, Error> {
        let mut stmt =
            conn.prepare("SELECT id, show_id, name, episode_number FROM episodes WHERE id = ?1")?;
        let episode = stmt.query_row(params![id], |row| {
            Ok(Episode {
                id: Some(row.get(0)?),
                show_id: row.get(1)?,
                name: row.get(2)?,
                episode_number: row.get(3)?,
            })
        })?;
        Ok(episode)
    }

    /// Retrieves all episodes for a specific show
    #[cfg(test)]
    pub fn get_all_for_show(conn: &Connection, show_id: i32) -> Result<Vec<Episode>, Error> {
        let mut stmt = conn.prepare("SELECT id, show_id, name, episode_number FROM episodes WHERE show_id = ?1 ORDER BY episode_number NULLS LAST")?;
        let episodes_iter = stmt.query_map(params![show_id], |row| {
            Ok(Episode {
                id: Some(row.get(0)?),
                show_id: row.get(1)?,
                name: row.get(2)?,
                episode_number: row.get(3)?,
            })
        })?;

        episodes_iter
            .collect::<Result<Vec<_>, _>>()
            .map_err(Error::from)
    }

    /// Searches for episodes by name
    #[cfg(test)]
    pub fn search_by_name(conn: &Connection, search_term: &str) -> Result<Vec<Episode>, Error> {
        let mut stmt = conn
            .prepare("SELECT id, show_id, name, episode_number FROM episodes WHERE name LIKE ?1")?;
        let episodes_iter = stmt.query_map(params![format!("%{}%", search_term)], |row| {
            Ok(Episode {
                id: Some(row.get(0)?),
                show_id: row.get(1)?,
                name: row.get(2)?,
                episode_number: row.get(3)?,
            })
        })?;

        episodes_iter
            .collect::<Result<Vec<_>, _>>()
            .map_err(Error::from)
    }

    /// Gets the associated Show for this Episode
    #[cfg(test)]
    pub fn get_show(&self, conn: &Connection) -> Result<Show, Error> {
        Show::get_by_id(conn, self.show_id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::{create_test_db, create_test_show};

    #[test]
    fn test_insert_and_get_episode() {
        let (_file, handler) = create_test_db();
        let conn = &handler.conn;
        let show = create_test_show(&handler, "Test Show", "Anime");

        let mut episode = Episode::new(show.id.unwrap(), "Test Episode".to_string(), Some(1));
        episode.insert(&conn).unwrap();

        assert!(episode.id.is_some());

        let retrieved_episode = Episode::get_by_id(&conn, episode.id.unwrap()).unwrap();
        assert_eq!(retrieved_episode.name, "Test Episode");
        assert_eq!(retrieved_episode.episode_number, Some(1));
    }

    #[test]
    fn test_update_episode() {
        let (_file, handler) = create_test_db();
        let conn = &handler.conn;
        let show = create_test_show(&handler, "Test Show", "Anime");

        let mut episode = Episode::new(show.id.unwrap(), "Test Episode".to_string(), Some(1));
        episode.insert(&conn).unwrap();

        episode.name = "Updated Episode".to_string();
        episode.update(&conn).unwrap();

        let updated_episode = Episode::get_by_id(&conn, episode.id.unwrap()).unwrap();
        assert_eq!(updated_episode.name, "Updated Episode");
    }

    #[test]
    fn test_delete_episode() {
        let (_file, handler) = create_test_db();
        let conn = &handler.conn;
        let show = create_test_show(&handler, "Test Show", "Anime");

        let mut episode = Episode::new(show.id.unwrap(), "Test Episode".to_string(), Some(1));
        episode.insert(&conn).unwrap();

        episode.delete(&conn).unwrap();

        let result = Episode::get_by_id(&conn, episode.id.unwrap());
        assert!(result.is_err());
    }

    #[test]
    fn test_get_all_for_show() {
        let (_file, handler) = create_test_db();
        let conn = &handler.conn;
        let show = create_test_show(&handler, "Test Show", "Anime");

        let episodes = vec![
            Episode::new(show.id.unwrap(), "Episode 1".to_string(), Some(1)),
            Episode::new(show.id.unwrap(), "Episode 2".to_string(), Some(2)),
            Episode::new(show.id.unwrap(), "Episode 3".to_string(), Some(3)),
        ];

        for mut episode in episodes {
            episode.insert(&conn).unwrap();
        }

        let retrieved_episodes = Episode::get_all_for_show(&conn, show.id.unwrap()).unwrap();
        assert_eq!(retrieved_episodes.len(), 3);
        assert_eq!(retrieved_episodes[0].episode_number, Some(1));
        assert_eq!(retrieved_episodes[1].episode_number, Some(2));
        assert_eq!(retrieved_episodes[2].episode_number, Some(3));
    }

    #[test]
    fn test_search_episodes() {
        let (_file, handler) = create_test_db();
        let conn = &handler.conn;
        let show = create_test_show(&handler, "Test Show", "Anime");

        let episodes = vec![
            Episode::new(show.id.unwrap(), "Pilot Episode".to_string(), Some(1)),
            Episode::new(show.id.unwrap(), "Regular Episode".to_string(), Some(2)),
            Episode::new(show.id.unwrap(), "Finale Episode".to_string(), Some(3)),
        ];

        for mut episode in episodes {
            episode.insert(&conn).unwrap();
        }

        let search_results = Episode::search_by_name(&conn, "Episode").unwrap();
        assert_eq!(search_results.len(), 3);

        let search_results = Episode::search_by_name(&conn, "Pilot").unwrap();
        assert_eq!(search_results.len(), 1);
        assert_eq!(search_results[0].name, "Pilot Episode");
    }

    #[test]
    fn test_get_show_for_episode() {
        let (_file, handler) = create_test_db();
        let conn = &handler.conn;
        let show = create_test_show(&handler, "Test Show", "Anime");

        let mut episode = Episode::new(show.id.unwrap(), "Test Episode".to_string(), Some(1));
        episode.insert(&conn).unwrap();

        let retrieved_show = episode.get_show(&conn).unwrap();
        assert_eq!(retrieved_show.id, show.id);
        assert_eq!(retrieved_show.name, show.name);
    }

    #[test]
    fn test_episode_with_no_number() {
        let (_file, handler) = create_test_db();
        let conn = &handler.conn;
        let show = create_test_show(&handler, "Movie Collection", "Anime");

        // Create episode without a number (like a movie)
        let mut movie_episode = Episode::new(show.id.unwrap(), "Movie Title".to_string(), None);
        movie_episode.insert(&conn).unwrap();

        assert!(movie_episode.id.is_some());

        let retrieved_episode = Episode::get_by_id(&conn, movie_episode.id.unwrap()).unwrap();
        assert_eq!(retrieved_episode.name, "Movie Title");
        assert_eq!(retrieved_episode.episode_number, None);

        // Test that we can have multiple episodes with None episode numbers
        let mut special_episode =
            Episode::new(show.id.unwrap(), "Special Episode".to_string(), None);
        special_episode.insert(&conn).unwrap();

        let episodes = Episode::get_all_for_show(&conn, show.id.unwrap()).unwrap();
        assert_eq!(episodes.len(), 2);

        // Both should have None episode numbers and should sort to the end
        let movies: Vec<_> = episodes
            .iter()
            .filter(|e| e.episode_number.is_none())
            .collect();
        assert_eq!(movies.len(), 2);
    }
}
