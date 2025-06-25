use std::error::Error;
use std::fmt;
use crate::db::client::Song;
use tokio::runtime::Runtime;
use crate::db::client::DBClient;

use mongodb::{
    bson::{doc, Bson, Document},
    options::{ClientOptions, IndexOptions},
    IndexModel, Client, Collection,
};
use mongodb::error::ErrorKind;
use mongodb::results::InsertOneResult;

use crate::models;
use crate::utils;

/// MongoClient wraps a MongoDB client.
pub struct MongoClient {
    pub client: Client,
}

impl MongoClient {
    /// Creates a new MongoDB client using the provided URI.
    pub async fn new(uri: &str) -> Result<Self, Box<dyn Error>> {
        let mut client_options = ClientOptions::parse(uri).await?;
        client_options.app_name = Some("song-recognition".to_string());
        let client = Client::with_options(client_options)?;
        Ok(MongoClient { client })
    }

    /// Closes the connection by disconnecting the underlying client.
    pub async fn close(&self) -> Result<(), Box<dyn Error>> {
        // MongoDB Rust driver does not provide an explicit close method.
        // Dropping the client will close connections.
        Ok(())
    }
    pub async fn get_all_songs(&self) -> Result<Vec<crate::db::client::LibrarySong>, Box<dyn Error>> {
        let collection = self.songs_collection();
        let mut cursor = collection.find(doc! {}).await?;
        let mut songs = Vec::new();
        
        while cursor.advance().await? {
            let doc = cursor.current();
            let id = doc.get_i64("_id")? as u32;
            let key = doc.get_str("key")?;
            let youtube_id = doc.get_str("ytID")?;
            
            // Parse title and artist from key
            let parts: Vec<&str> = key.split("---").collect();
            if parts.len() >= 2 {
                songs.push(crate::db::client::LibrarySong {
                    id,
                    name: parts[0].to_string(),
                    artist: parts[1].to_string(),
                    youtube_id: youtube_id.to_string(),
                    thumbnail_url: Some(format!("https://img.youtube.com/vi/{}/mqdefault.jpg", youtube_id)),
                });
            }
        }
        
        Ok(songs)
    }

    /// Returns the fingerprints collection.
    fn fingerprints_collection(&self) -> Collection<Document> {
        self.client.database("song-recognition").collection("fingerprints")
    }

    /// Returns the songs collection.
    fn songs_collection(&self) -> Collection<Document> {
        self.client.database("song-recognition").collection("songs")
    }
}

impl MongoClient {
    /// Stores fingerprints into the "fingerprints" collection.
    pub async fn store_fingerprints(
        &self,
        fingerprints: &std::collections::HashMap<u32, models::Couple>,
    ) -> Result<(), Box<dyn Error>> {
        let collection = self.fingerprints_collection();

        for (&address, couple) in fingerprints.iter() {
            let filter = doc! { "_id": address as i64 };
            let update = doc! {
                "$push": {
                    "couples": {
                        "anchorTimeMs": couple.anchor_time_ms as i64,
                        "songID": couple.song_id as i64,
                    }
                }
            };
            collection.update_one(filter, update)
                .upsert(true)
                .await
                .map_err(|e| {
                    format!("error upserting document: {}", e)
                })?;
        }
        Ok(())
    }

    /// Retrieves fingerprint couples for the given addresses.
    pub async fn get_couples(
        &self,
        addresses: &[u32],
    ) -> Result<std::collections::HashMap<u32, Vec<models::Couple>>, Box<dyn Error>> {
        let collection = self.fingerprints_collection();
        let mut couples_map = std::collections::HashMap::new();

        for &address in addresses {
            let filter = doc! { "_id": address as i64 };
            let result = collection.find_one(filter).await?;
            if let Some(doc) = result {
                // Expect "couples" field to be an array.
                let couples_array = doc.get_array("couples")?;
                let mut couples = Vec::new();
                for item in couples_array {
                    if let Bson::Document(item_doc) = item {
                        let anchor_time_ms = item_doc.get_i64("anchorTimeMs")? as u32;
                        let song_id = item_doc.get_i64("songID")? as u32;
                        couples.push(models::Couple {
                            anchor_time_ms,
                            song_id,
                        });
                    } else {
                        return Err(format!(
                            "invalid couple format in document for address {}",
                            address
                        ).into());
                    }
                }
                couples_map.insert(address, couples);
            }
        }

        Ok(couples_map)
    }

    /// Returns the total number of documents in the "songs" collection.
    pub async fn total_songs(&self) -> Result<i32, Box<dyn Error>> {
        let collection = self.songs_collection();
        let count = collection.count_documents(doc! {}).await?;
        Ok(count as i32)
    }

    /// Registers a new song by inserting it into the "songs" collection.
    /// A unique song ID is generated using `utils::generate_unique_id()`.
    pub async fn register_song(
        &self,
        song_title: &str,
        song_artist: &str,
        yt_id: &str,
    ) -> Result<u32, Box<dyn Error>> {
        let collection = self.songs_collection();

        // Create a compound unique index on "ytID" and "key".
        let index_keys = doc! { "ytID": 1, "key": 1 };
        let index_options = IndexOptions::builder().unique(true).build();
        let index_model = IndexModel::builder().keys(index_keys).options(index_options).build();
        collection
            .create_index(index_model)
            .await
            .map_err(|e| format!("failed to create unique index: {}", e))?;

        let song_id = utils::generate_unique_id();
        let key = utils::generate_song_key(song_title, song_artist);

        let doc = doc! {
            "_id": song_id as i64,
            "key": key,
            "ytID": yt_id,
        };

        match collection.insert_one(doc).await {
            Ok(_result) => Ok(song_id),
            Err(e) => {
                match *e.kind {
                    mongodb::error::ErrorKind::BulkWrite(ref bulk_write_error) => {
                        if bulk_write_error.write_errors.iter().any(|(_, err)| err.code == 11000) {
                            return Err(format!("song with ytID or key already exists: {}", e).into());
                        }
                    },
                    _ => {}
                }
                Err(format!("failed to register song: {}", e).into())
            }
        }
        
    }

    /// Retrieves a song from the "songs" collection using the given filter key and value.
    pub async fn get_song(
        &self,
        filter_key: &str,
        value: BsonValue,
    ) -> Result<(Song, bool), Box<dyn Error>> {
        // Allowed filter keys.
        let allowed_keys = ["_id", "ytID", "key"];
        if !allowed_keys.contains(&filter_key) {
            return Err("invalid filter key".into());
        }

        let collection = self.songs_collection();
        let filter = doc! { filter_key: value };
        let result = collection.find_one(filter).await?;
        if let Some(doc) = result {
            let yt_id = doc.get_str("ytID")?.to_string();
            let key: String = doc.get_str("key")?.to_string();
            let parts: Vec<&str> = key.split("---").collect();
            if parts.len() < 2 {
                return Err("invalid key format".into());
            }
            let song_instance = Song {
                title: parts[0].to_string(),
                artist: parts[1].to_string(),
                youtube_id: yt_id,
            };
            Ok((song_instance, true))
        } else {
            Ok((Song::default(), false))
        }
    }

    pub async fn get_song_by_id(&self, song_id: u32) -> Result<(Song, bool), Box<dyn Error>> {
        self.get_song("_id", BsonValue::Int64(song_id as i64)).await
    }

    pub async fn get_song_by_ytid(&self, yt_id: &str) -> Result<(Song, bool), Box<dyn Error>> {
        self.get_song("ytID", BsonValue::String(yt_id.to_string())).await
    }

    pub async fn get_song_by_key(&self, key: &str) -> Result<(Song, bool), Box<dyn Error>> {
        self.get_song("key", BsonValue::String(key.to_string())).await
    }

    /// Deletes a song from the "songs" collection by its ID.
    pub async fn delete_song_by_id(&self, song_id: u32) -> Result<(), Box<dyn Error>> {
        let collection = self.songs_collection();
        let filter = doc! { "_id": song_id as i64 };
        collection.delete_one(filter).await.map_err(|e| {
            format!("failed to delete song: {}", e)
        })?;
        Ok(())
    }

    /// Drops the specified collection from the "song-recognition" database.
    pub async fn delete_collection(&self, collection_name: &str) -> Result<(), Box<dyn Error>> {
        let collection = self.client.database("song-recognition").collection::<Document>(collection_name);
        collection.drop().await.map_err(|e| {
            format!("error deleting collection: {}", e)
        })?;
        Ok(())
    }
}


impl DBClient for MongoClient {
    fn register_song(&mut self, song_title: &str, song_artist: &str, yt_id: &str) -> Result<u32, Box<dyn Error>> {
        // Create a runtime to run async code in sync context
        let rt = Runtime::new()?;
        // Use fully qualified syntax to call the struct method, not the trait method
        rt.block_on(<MongoClient>::register_song(self, song_title, song_artist, yt_id))
    }
    
    fn store_fingerprints(&mut self, fingerprints: &std::collections::HashMap<u32, models::Couple>) -> Result<(), Box<dyn Error>> {
        let rt = Runtime::new()?;
        rt.block_on(<MongoClient>::store_fingerprints(self, fingerprints))
    }
    
    fn get_couples(&self, addresses: &[u32]) -> Result<std::collections::HashMap<u32, Vec<models::Couple>>, Box<dyn Error>> {
        let rt = Runtime::new()?;
        rt.block_on(<MongoClient>::get_couples(self, addresses))
    }
    
    fn get_all_songs(&self) -> Result<Vec<crate::db::client::LibrarySong>, Box<dyn Error>> {
        let rt = Runtime::new()?;
        rt.block_on(self.get_all_songs())
    }
    
    fn total_songs(&self) -> Result<i32, Box<dyn Error>> {
        let rt = Runtime::new()?;
        rt.block_on(<MongoClient>::total_songs(self))
    }
    
    fn get_song(&self, filter_key: &str, value: &str) -> Result<(Song, bool), Box<dyn Error>> {
        // Convert string value to BsonValue based on filter_key
        let bson_value = match filter_key {
            "_id" => {
                let id = value.parse::<i64>().map_err(|e| format!("invalid id: {}", e))?;
                BsonValue::Int64(id)
            },
            _ => BsonValue::String(value.to_string()),
        };
        
        let rt = Runtime::new()?;
        rt.block_on(<MongoClient>::get_song(self, filter_key, bson_value))
    }
    
    fn get_song_by_id(&self, song_id: u32) -> Result<(Song, bool), Box<dyn Error>> {
        let rt = Runtime::new()?;
        rt.block_on(<MongoClient>::get_song_by_id(self, song_id))
    }
    
    fn get_song_by_ytid(&self, yt_id: &str) -> Result<(Song, bool), Box<dyn Error>> {
        let rt = Runtime::new()?;
        rt.block_on(<MongoClient>::get_song_by_ytid(self, yt_id))
    }
    
    fn get_song_by_key(&self, key: &str) -> Result<(Song, bool), Box<dyn Error>> {
        let rt = Runtime::new()?;
        rt.block_on(<MongoClient>::get_song_by_key(self, key))
    }
    
    fn delete_song_by_id(&mut self, song_id: u32) -> Result<(), Box<dyn Error>> {
        let rt = Runtime::new()?;
        rt.block_on(<MongoClient>::delete_song_by_id(self, song_id))
    }
    
    fn delete_collection(&mut self, collection_name: &str) -> Result<(), Box<dyn Error>> {
        let rt = Runtime::new()?;
        rt.block_on(<MongoClient>::delete_collection(self, collection_name))
    }
    
    fn close(&mut self) -> Result<(), Box<dyn Error>> {
        let rt = Runtime::new()?;
        rt.block_on(<MongoClient>::close(self))
    }
}
/// A helper enum to represent BSON value types for filtering.
pub enum BsonValue {
    Int64(i64),
    String(String),
}

impl From<BsonValue> for mongodb::bson::Bson {
    fn from(val: BsonValue) -> Self {
        match val {
            BsonValue::Int64(i) => mongodb::bson::Bson::Int64(i),
            BsonValue::String(s) => mongodb::bson::Bson::String(s),
        }
    }
}

impl BsonValue {
    pub fn as_bson(&self) -> mongodb::bson::Bson {
        match self {
            BsonValue::Int64(i) => mongodb::bson::Bson::Int64(*i),
            BsonValue::String(s) => mongodb::bson::Bson::String(s.clone()),
        }
    }
}

// Provide a default implementation for Song for error handling.
impl Default for Song {
    fn default() -> Self {
        Song {
            title: "".to_string(),
            artist: "".to_string(),
            youtube_id: "".to_string(),
        }
    }
}
