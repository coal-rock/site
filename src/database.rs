use crate::config::Config;
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

        sqlx::query(
            r#"
            DROP TABLE users
        "#,
        )
        .execute(&mut conn)
        .await
        .unwrap();

        sqlx::query!(
            r#"
            CREATE TABLE IF NOT EXISTS users (
                id integer primary key not null,
                password_hash text not null,
                name text not null unique
            )
        "#
        )
        .execute(&mut conn)
        .await
        .unwrap();

        sqlx::query!(
            r#"
            CREATE TABLE IF NOT EXISTS blog_posts (
                title text not null unique,
                description text not null,
                content text not null,
                date text not null,
                serve_path text not null unique
            )
        "#
        )
        .execute(&mut conn)
        .await
        .unwrap();

        sqlx::query!(
            r#"
            CREATE TABLE IF NOT EXISTS notes (
                title text not null unique,
                content text not null,
                date text not null,
                id integer primary key autoincrement
            )
        "#
        )
        .execute(&mut conn)
        .await
        .unwrap();

        sqlx::query!(
            r#"
            CREATE TABLE IF NOT EXISTS requests (
                url text not null,
                user_agent text,
                referer text,
                timestamp text not null
            )
        "#
        )
        .execute(&mut conn)
        .await
        .unwrap();

        sqlx::query(
            r#"
            INSERT INTO users (id, password_hash, name) values ($1, $2, $3)
        "#,
        )
        .bind(0)
        .bind(config.pass)
        .bind(config.user)
        .execute(&mut conn)
        .await
        .unwrap();

        Database { pool }
    }
}
