use rusqlite::{params, Connection, OptionalExtension};
use std::error::Error;
use std::fmt;

use crate::models;
use crate::utils;

use crate::db::client::Song;
use crate::db::client::DBClient;


/// SQLiteClient wraps a rusqlite Connection.
pub struct SQLiteClient {
    pub db: Connection,
}

impl SQLiteClient {
    /// Opens a new SQLite connection using the given data source name and creates the required tables.
    pub fn new(data_source_name: &str) -> Result<Self, Box<dyn Error>> {
        let db = Connection::open(data_source_name)
            .map_err(|e| format!("error connecting to SQLite: {}", e))?;
        create_tables(&db)?;
        Ok(SQLiteClient { db })
    }

    /// Closes the database connection.
    pub fn close(self) -> Result<(), Box<dyn Error>> {
        self.db.close().map_err(|(_, e)| e.into())
    }

    /// Stores fingerprints into the fingerprints table.
    pub fn store_fingerprints(
        &mut self,
        fingerprints: &std::collections::HashMap<u32, models::Couple>,
    ) -> Result<(), Box<dyn Error>> {
        let tx = self.db.transaction()?;
        {
            let mut stmt = tx.prepare(
                "INSERT OR REPLACE INTO fingerprints (address, anchorTimeMs, songID) VALUES (?, ?, ?)",
            )?;
            for (&address, couple) in fingerprints.iter() {
                stmt.execute(params![address as i64, couple.anchor_time_ms as i64, couple.song_id as i64])?;
            }
        }
        tx.commit()?;
        Ok(())
    }

    /// Retrieves fingerprint couples for the given addresses.
    pub fn get_couples(
        &self,
        addresses: &[u32],
    ) -> Result<std::collections::HashMap<u32, Vec<models::Couple>>, Box<dyn Error>> {
        let mut couples_map = std::collections::HashMap::new();

        for &address in addresses {
            let mut stmt = self.db.prepare(
                "SELECT anchorTimeMs, songID FROM fingerprints WHERE address = ?",
            )?;
            let mut rows = stmt.query(params![address as i64])?;

            let mut doc_couples = Vec::new();
            while let Some(row) = rows.next()? {
                let anchor_time_ms: i64 = row.get(0)?;
                let song_id: i64 = row.get(1)?;
                doc_couples.push(models::Couple {
                    anchor_time_ms: anchor_time_ms as u32,
                    song_id: song_id as u32,
                });
            }
            couples_map.insert(address, doc_couples);
        }

        Ok(couples_map)
    }

    /// Returns the total number of songs in the songs table.
    pub fn total_songs(&self) -> Result<i32, Box<dyn Error>> {
        let count: i32 = self.db.query_row("SELECT COUNT(*) FROM songs", [], |row| row.get(0))?;
        Ok(count)
    }

    /// Registers a new song in the songs table.
    pub fn register_song(
        &mut self,
        song_title: &str,
        song_artist: &str,
        yt_id: &str,
    ) -> Result<u32, Box<dyn Error>> {
        let tx = self.db.transaction()?;
        let song_id = utils::generate_unique_id();
        let song_key = utils::generate_song_key(song_title, song_artist);
        let res = tx.execute(
            "INSERT INTO songs (id, title, artist, ytID, key) VALUES (?, ?, ?, ?, ?)",
            params![
                song_id as i64,
                song_title,
                song_artist,
                yt_id,
                song_key
            ],
        );
        match res {
            Ok(_) => {
                tx.commit()?;
                Ok(song_id)
            }
            Err(e) => {
                tx.rollback()?;
                if let rusqlite::Error::SqliteFailure(ref err, _) = e {
                    if err.extended_code == rusqlite::ffi::SQLITE_CONSTRAINT_UNIQUE {
                        return Err(format!("song with ytID or key already exists: {}", e).into());
                    }
                }
                Err(format!("failed to register song: {}", e).into())
            }
        }
    }

    /// Retrieves a song by a filter key.
    pub fn get_song(
        &self,
        filter_key: &str,
        value: &rusqlite::types::Value,
    ) -> Result<(Song, bool), Box<dyn Error>> {
        // Allowed filter keys.
        let allowed_keys = ["id", "ytID", "key"];
        if !allowed_keys.contains(&filter_key) {
            return Err("invalid filter key".into());
        }

        let query = format!("SELECT title, artist, ytID FROM songs WHERE {} = ?", filter_key);
        let mut stmt = self.db.prepare(&query)?;
        let song_opt = stmt.query_row(&[value], |row| {
            Ok(Song {
                title: row.get(0)?,
                artist: row.get(1)?,
                youtube_id: row.get(2)?,
            })
        }).optional()?;

        if let Some(song) = song_opt {
            Ok((song, true))
        } else {
            // Return a default Song if not found.
            Ok((Song::default(), false))
        }
    }

    pub fn get_song_by_id(&self, song_id: u32) -> Result<(Song, bool), Box<dyn Error>> {
        self.get_song("id", &rusqlite::types::Value::Integer(song_id as i64))
    }

    pub fn get_song_by_ytid(&self, yt_id: &str) -> Result<(Song, bool), Box<dyn Error>> {
        self.get_song("ytID", &rusqlite::types::Value::Text(yt_id.to_string()))
    }

    pub fn get_song_by_key(&self, key: &str) -> Result<(Song, bool), Box<dyn Error>> {
        self.get_song("key", &rusqlite::types::Value::Text(key.to_string()))
    }

    /// Deletes a song by its ID.
    pub fn delete_song_by_id(&self, song_id: u32) -> Result<(), Box<dyn Error>> {
        self.db.execute("DELETE FROM songs WHERE id = ?", params![song_id as i64])?;
        Ok(())
    }

    /// Drops a table (collection) from the database.
    pub fn delete_collection(&self, collection_name: &str) -> Result<(), Box<dyn Error>> {
        let query = format!("DROP TABLE IF EXISTS {}", collection_name);
        self.db.execute(&query, [])?;
        Ok(())
    }
    pub fn get_all_songs(&self) -> Result<Vec<crate::db::client::LibrarySong>, Box<dyn Error>> {
        let mut stmt = self.db.prepare("SELECT id, title, artist, ytID FROM songs ORDER BY title")?;
        let song_iter = stmt.query_map([], |row| {
            let id: i64 = row.get(0)?;
            let title: String = row.get(1)?;
            let artist: String = row.get(2)?;
            let youtube_id: String = row.get(3)?;
            
            Ok(crate::db::client::LibrarySong {
                id: id as u32,
                name: title,
                artist,
                youtube_id: youtube_id.clone(),
                thumbnail_url: Some(format!("https://img.youtube.com/vi/{}/mqdefault.jpg", youtube_id)),
            })
        })?;

        let mut songs = Vec::new();
        for song in song_iter {
            songs.push(song?);
        }
        Ok(songs)
    }
}

impl DBClient for SQLiteClient {
    fn register_song(&mut self, song_title: &str, song_artist: &str, yt_id: &str) -> Result<u32, Box<dyn Error>> {
        self.register_song(song_title, song_artist, yt_id)
    }
    fn get_all_songs(&self) -> Result<Vec<crate::db::client::LibrarySong>, Box<dyn Error>> {
        let mut stmt = self.db.prepare("SELECT id, title, artist, ytID FROM songs ORDER BY title")?;
        let song_iter = stmt.query_map([], |row| {
            let id: i64 = row.get(0)?;
            let title: String = row.get(1)?;
            let artist: String = row.get(2)?;
            let youtube_id: String = row.get(3)?;
            
            Ok(crate::db::client::LibrarySong {
                id: id as u32,
                name: title,
                artist,
                youtube_id: youtube_id.clone(),
                thumbnail_url: Some(format!("https://img.youtube.com/vi/{}/mqdefault.jpg", youtube_id)),
            })
        })?;

        let mut songs = Vec::new();
        for song in song_iter {
            songs.push(song?);
        }
        Ok(songs)
    }

    fn store_fingerprints(&mut self, fingerprints: &std::collections::HashMap<u32, models::Couple>) -> Result<(), Box<dyn Error>> {
        self.store_fingerprints(fingerprints)
    }

    fn get_couples(&self, addresses: &[u32]) -> Result<std::collections::HashMap<u32, Vec<models::Couple>>, Box<dyn Error>> {
        self.get_couples(addresses)
    }

    fn total_songs(&self) -> Result<i32, Box<dyn Error>> {
        self.total_songs()
    }

    fn get_song_by_id(&self, song_id: u32) -> Result<(Song, bool), Box<dyn Error>> {
        self.get_song_by_id(song_id)
    }

    fn get_song_by_ytid(&self, yt_id: &str) -> Result<(Song, bool), Box<dyn Error>> {
        self.get_song_by_ytid(yt_id)
    }

    fn get_song_by_key(&self, key: &str) -> Result<(Song, bool), Box<dyn Error>> {
        self.get_song_by_key(key)
    }

    fn delete_song_by_id(&mut self, song_id: u32) -> Result<(), Box<dyn Error>> {
        SQLiteClient::delete_song_by_id(self, song_id)
    }

    fn delete_collection(&mut self, collection_name: &str) -> Result<(), Box<dyn Error>> {
        SQLiteClient::delete_collection(self, collection_name)
    }
    fn close(&mut self) -> Result<(), Box<dyn Error>> {
        // We can't directly call self.close() because it consumes self
        // Instead, we'll handle it differently for the trait implementation
        Ok(()) // This is a workaround - the actual close happens when the struct is dropped
    }

    fn get_song(&self, filter_key: &str, value: &str) -> Result<(Song, bool), Box<dyn Error>> {
        // Convert the string value to the appropriate rusqlite::types::Value based on filter_key
        let sqlite_value = match filter_key {
            "id" => {
                let id = value.parse::<i64>().map_err(|e| format!("invalid id: {}", e))?;
                rusqlite::types::Value::Integer(id)
            }
            _ => rusqlite::types::Value::Text(value.to_string()),
        };
        
        // Call our existing implementation with the converted value
        self.get_song(filter_key, &sqlite_value)
    }
}
/// Creates the required tables if they do not exist.
fn create_tables(db: &Connection) -> Result<(), Box<dyn Error>> {
    let create_songs_table = r#"
        CREATE TABLE IF NOT EXISTS songs (
            id INTEGER PRIMARY KEY,
            title TEXT NOT NULL,
            artist TEXT NOT NULL,
            ytID TEXT UNIQUE,
            key TEXT NOT NULL UNIQUE
        );
    "#;

    let create_fingerprints_table = r#"
        CREATE TABLE IF NOT EXISTS fingerprints (
            address INTEGER NOT NULL,
            anchorTimeMs INTEGER NOT NULL,
            songID INTEGER NOT NULL,
            PRIMARY KEY (address, anchorTimeMs, songID)
        );
    "#;

    db.execute(create_songs_table, [])
        .map_err(|e| format!("error creating songs table: {}", e))?;
    db.execute(create_fingerprints_table, [])
        .map_err(|e| format!("error creating fingerprints table: {}", e))?;

    Ok(())
}
