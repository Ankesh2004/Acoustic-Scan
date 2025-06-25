mod client;
pub use client::*;
mod mongo;
pub use mongo::*;
mod sqlite;
pub use sqlite::*;

use std::error::Error;
use std::env;

use client::DBClient;

pub async fn new_db_client() -> Result<Box<dyn DBClient>, Box<dyn Error>> {
    // Get database type from environment or use SQLite as default
    let db_type = env::var("DB_TYPE").unwrap_or_else(|_| "sqlite".to_string());
    let db_file = env::var("DB_FILE").unwrap_or_else(|_| "db.sqlite3".to_string());
    
    match db_type.as_str() {
        "mongo" => {
            // MongoDB implementation is not currently available
            #[cfg(feature = "mongodb")]
            {
                let mongo_client = mongo::MongoClient::new(&db_file).await?;
                return Ok(Box::new(mongo_client) as Box<dyn DBClient>);
            }
            
            // Default if MongoDB is not enabled
            #[cfg(not(feature = "mongodb"))]
            return Err(format!("MongoDB client not implemented. db_file: {}", db_file).into());
        }
        "sqlite" => {
            // Use SQLite by default
            let sqlite_client = sqlite::SQLiteClient::new(&db_file)?;
                return Ok(Box::new(sqlite_client) as Box<dyn DBClient>);
            }
            _ => {
                return Err(format!("Unsupported DB type: {}", db_type).into());
            }
        }
    }