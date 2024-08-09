use crate::error::Error;
use rusqlite::{params, Connection};

#[derive(Debug)]
pub struct Show {
    pub id: Option<i64>,
    pub name: String,
    pub show_type: String,
}

impl Show {
    pub fn new(name: String, show_type: String) -> Self {
        Show {
            id: None,
            name,
            show_type,
        }
    }

    pub fn insert(&mut self, conn: &Connection) -> Result<(), Error> {
        conn.execute(
            "INSERT OR IGNORE INTO shows (name, show_type) VALUES (?1, ?2)",
            params![self.name, self.show_type],
        )?;
        self.id = Some(conn.last_insert_rowid());
        Ok(())
    }

    pub fn update(&self, conn: &Connection) -> Result<(), Error> {
        conn.execute(
            "UPDATE shows SET name = ?1, show_type = ?2 WHERE id = ?3",
            params![self.name, self.show_type, self.id],
        )?;
        Ok(())
    }

    pub fn delete(&self, conn: &Connection) -> Result<(), Error> {
        conn.execute("DELETE FROM shows WHERE id = ?1", params![self.id])?;
        Ok(())
    }

    pub fn get_by_id(conn: &Connection, id: i64) -> Result<Show, Error> {
        let mut stmt = conn.prepare("SELECT id, name, show_type FROM shows WHERE id = ?1")?;
        let show = stmt.query_row(params![id], |row| {
            Ok(Show {
                id: Some(row.get(0)?),
                name: row.get(1)?,
                show_type: row.get(2)?,
            })
        })?;
        Ok(show)
    }

    pub fn get_all(conn: &Connection) -> Result<Vec<Show>, Error> {
        let mut stmt = conn.prepare("SELECT id, name, show_type FROM shows")?;
        let shows_iter = stmt.query_map([], |row| {
            Ok(Show {
                id: Some(row.get(0)?),
                name: row.get(1)?,
                show_type: row.get(2)?,
            })
        })?;

        let mut shows = Vec::new();
        for show in shows_iter {
            shows.push(show?);
        }
        Ok(shows)
    }

    pub fn get_by_name(conn: &Connection, name: &str) -> Result<Show, Error> {
        let mut stmt = conn.prepare("SELECT id, name, show_type FROM shows WHERE name = ?1")?;
        let show = stmt.query_row(params![name], |row| {
            Ok(Show {
                id: Some(row.get(0)?),
                name: row.get(1)?,
                show_type: row.get(2)?,
            })
        })?;
        Ok(show)
    }

    pub fn search_by_name(conn: &Connection, search_term: &str) -> Result<Vec<Show>, Error> {
        let mut stmt = conn.prepare("SELECT id, name, show_type FROM shows WHERE name LIKE ?1")?;
        let shows_iter = stmt.query_map(params![format!("%{}%", search_term)], |row| {
            Ok(Show {
                id: Some(row.get(0)?),
                name: row.get(1)?,
                show_type: row.get(2)?,
            })
        })?;

        let mut shows = Vec::new();
        for show in shows_iter {
            shows.push(show?);
        }
        Ok(shows)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::DbHandler;
    use rusqlite::Connection;
    use tempfile::NamedTempFile;

    fn create_test_db() -> (NamedTempFile, Connection) {
        let file = NamedTempFile::new().unwrap();
        let conn = Connection::open(file.path()).unwrap();
        let db_handler = DbHandler::new(file.path().to_str().unwrap()).unwrap();
        db_handler.create_tables().unwrap();
        (file, conn)
    }

    #[test]
    fn test_insert_and_get_show() {
        let (_file, conn) = create_test_db();
        let mut show = Show::new("Test Show".to_string(), "Anime".to_string());
        show.insert(&conn).unwrap();

        assert!(show.id.is_some());

        let retrieved_show = Show::get_by_id(&conn, show.id.unwrap()).unwrap();
        assert_eq!(retrieved_show.name, "Test Show");
        assert_eq!(retrieved_show.show_type, "Anime");
    }

    #[test]
    fn test_update_show() {
        let (_file, conn) = create_test_db();
        let mut show = Show::new("Test Show".to_string(), "Anime".to_string());
        show.insert(&conn).unwrap();

        show.name = "Updated Show".to_string();
        show.update(&conn).unwrap();

        let updated_show = Show::get_by_id(&conn, show.id.unwrap()).unwrap();
        assert_eq!(updated_show.name, "Updated Show");
    }

    #[test]
    fn test_delete_show() {
        let (_file, conn) = create_test_db();
        let mut show = Show::new("Test Show".to_string(), "Anime".to_string());
        show.insert(&conn).unwrap();

        show.delete(&conn).unwrap();

        let result = Show::get_by_id(&conn, show.id.unwrap());
        assert!(result.is_err());
    }

    #[test]
    fn test_search_shows() {
        let (_file, conn) = create_test_db();
        let shows = vec![
            Show::new("Anime Show 1".to_string(), "Anime".to_string()),
            Show::new("Drama Show".to_string(), "Drama".to_string()),
            Show::new("Anime Show 2".to_string(), "Anime".to_string()),
        ];

        for mut show in shows {
            show.insert(&conn).unwrap();
        }

        let search_results = Show::search_by_name(&conn, "Anime").unwrap();
        assert_eq!(search_results.len(), 2);
        assert!(search_results.iter().any(|s| s.name == "Anime Show 1"));
        assert!(search_results.iter().any(|s| s.name == "Anime Show 2"));
    }
}
