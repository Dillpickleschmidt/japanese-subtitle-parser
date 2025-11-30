use rusqlite::Connection;

/// Simple helper to set ID from last insert operation
/// Eliminates the repeated `conn.last_insert_rowid().try_into().unwrap()` pattern
pub fn set_id_from_last_insert(id_field: &mut Option<i32>, conn: &Connection) {
    *id_field = Some(conn.last_insert_rowid().try_into().unwrap());
}
