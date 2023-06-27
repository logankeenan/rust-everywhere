mod note_service;
mod notes_model;

use sqlx::{
    migrate::MigrateDatabase,
    sqlite::SqlitePoolOptions
};
use uuid::Uuid;
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


    let service = NoteService::new(pool);

    let user_id = Uuid::new_v4();
    service.create_note(String::from("note 1"), user_id).await?;
    service.create_note(String::from("note 2"), user_id).await?;

    let notes = service.all_notes(user_id).await?;

    println!("notes: {:?}", notes);

    let note_1 = notes.get(0).unwrap();

    service.update_note(String::from("note 1 updated"), note_1.id, user_id).await?;


    let note_1_updated = service.by_id(note_1.id, user_id).await?;

    println!("note 1: {:?}", note_1_updated);

    Ok(())
}
