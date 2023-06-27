mod note_service;
mod notes_model;

use sqlx::{
    migrate::MigrateDatabase,
    sqlite::SqlitePoolOptions
};
use crate::note_service::NoteService;

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    let db_location = "./sqlite.db";

    if !sqlx::Sqlite::database_exists(db_location).await? {
        sqlx::Sqlite::create_database(db_location).await?
    }

    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect(db_location).await?;

    sqlx::migrate!()
        .run(&pool)
        .await?;


    let _service = NoteService::new(pool);


    Ok(())
}
