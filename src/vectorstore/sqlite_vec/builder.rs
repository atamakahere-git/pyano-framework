use std::{ error::Error, str::FromStr, sync::Arc, fs };

use sqlx::{ sqlite::{ SqliteConnectOptions, SqlitePoolOptions }, Pool, Sqlite };
// main.rs
use super::sqlite_vec::Store;
use crate::embedding::embedder_trait::Embedder;
pub struct StoreBuilder {
    pool: Option<Pool<Sqlite>>,
    connection_url: Option<String>,
    table: String,
    embedder: Option<Arc<dyn Embedder>>,
}

impl StoreBuilder {
    pub fn new() -> Self {
        StoreBuilder {
            pool: None,
            connection_url: None,
            table: "documents".to_string(),
            embedder: None,
        }
    }

    pub fn pool(mut self, pool: Pool<Sqlite>) -> Self {
        self.pool = Some(pool);
        self.connection_url = None;
        self
    }

    pub fn in_memory(mut self) -> Self {
        let connection_url = std::env::var("DATABASE_URL").unwrap_or("sqlite::memory:".to_string());

        self.connection_url = Some(connection_url.into());
        self.pool = None;
        self
    }

    pub fn db_name<S: Into<String>>(mut self, db_name: S) -> Self {
        let home_directory = dirs::home_dir().unwrap();
        let root_pyano_dir = home_directory.join(".pyano");
        let pyano_data_dir = root_pyano_dir.join("database");
        if !pyano_data_dir.exists() {
            fs::create_dir_all(&pyano_data_dir).unwrap();
        }
        let file_path = pyano_data_dir.join(format!("{}.db", db_name.into()));
        self.connection_url = Some(format!("sqlite://{}", file_path.display()));
        self.pool = None;
        self
    }

    pub fn table(mut self, table: &str) -> Self {
        self.table = table.into();
        self
    }

    pub fn embedder<E: Embedder + 'static>(mut self, embedder: E) -> Self {
        self.embedder = Some(Arc::new(embedder));
        self
    }

    // Finalize the builder and construct the Store object
    pub async fn build(mut self) -> Result<Store, Box<dyn Error>> {
        if self.connection_url.is_none() {
            self = self.in_memory();
        }

        if self.embedder.is_none() {
            return Err("Embedder is required".into());
        }

        Ok(Store {
            pool: self.get_pool().await?,
            table: self.table,
            embedder: self.embedder.unwrap(),
        })
    }

    async fn get_pool(&self) -> Result<Pool<Sqlite>, Box<dyn Error>> {
        // use sqlite_vec::sqlite3_vec_init;
        // use rusqlite::{ ffi::sqlite3_auto_extension, Result };
        // unsafe {
        //     sqlite3_auto_extension(Some(std::mem::transmute(sqlite3_vec_init as *const ())));
        // }

        match &self.pool {
            Some(pool) => Ok(pool.clone()),
            None => {
                let connection_url = self.connection_url
                    .as_ref()
                    .ok_or("Connection URL or DB is required")?;

                let pool: Pool<Sqlite> = SqlitePoolOptions::new().connect_with(
                    SqliteConnectOptions::from_str(connection_url)?
                        .create_if_missing(true)
                        .extension("/Users/sauravverma/.pyano/bin/vec0")
                ).await?;

                Ok(pool)
            }
        }
    }
}
