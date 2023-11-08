use crate::config::Config;
use crate::page::Page;

use sqlx::migrate;
use sqlx::sqlite::SqlitePoolOptions;

pub struct Database {
    pool: sqlx::Pool<sqlx::Sqlite>,
}

impl Database {
    pub async fn new(config: &Config) -> Database {
        let pool = SqlitePoolOptions::new()
            .connect(&format!("{}?mode=rwc", config.db_path))
            .await
            .unwrap();

        let mut conn = pool.acquire().await.unwrap();

        sqlx::migrate!("./migrations/").run(&pool).await.unwrap();

        Database { pool }
    }

    pub async fn get_pages(&self) {
        // sqlx::query_as!(Page, "select * from pages");
    }
}
