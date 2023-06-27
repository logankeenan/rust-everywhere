use sqlx::{Pool, Sqlite};
use uuid::Uuid;
use crate::notes_model::Note;

pub struct NoteService {
    connection: Pool<Sqlite>
}

impl NoteService {
    pub fn new(connection: Pool<Sqlite>) -> Self {
        NoteService {
            connection,
        }
    }

    pub async fn all_notes(&self, user_id: Uuid) -> Result<Vec<Note>, sqlx::Error> {
        let notes = sqlx::query_as!(
            Note,
            r#" SELECT id, content, updated_at, created_at, user_id as "user_id: uuid::Uuid" FROM notes WHERE user_id = ?"#,
            user_id
        )
            .fetch_all(&self.connection).await?;

        Ok(notes)
    }


    pub async fn by_id(&self, id: i64, user_id: Uuid) -> Result<Note, sqlx::Error> {
        let note = sqlx::query_as!(
            Note,
            r#"
                SELECT id, content, updated_at, created_at, user_id as "user_id: uuid::Uuid" FROM notes
                WHERE id = ? AND user_id = ?
            "#,
            id,
            user_id
        )
            .fetch_one(&self.connection).await?;

        Ok(note)
    }

    pub async fn update_note(&self, content: String, id: i64, user_id: Uuid) -> Result<Note, sqlx::Error>  {
        sqlx::query!(
            r#"
                UPDATE notes
                SET content = ?, updated_at = CURRENT_TIMESTAMP
                WHERE id = ? AND user_id = ?
            "#,
            content,
            id,
            user_id
        )
            .execute(&self.connection).await?;

        let note = sqlx::query_as!(
            Note,
            r#"SELECT id, content, updated_at, created_at, user_id as "user_id: uuid::Uuid" FROM notes WHERE id = ?"#,
            id
        )
            .fetch_one(&self.connection).await?;

        Ok(note)
    }

    pub async fn create_note(&self, content: String, user_id: Uuid) -> Result<Note, sqlx::Error>  {
        let id: i64 = sqlx::query!(
            r#"
                INSERT INTO notes (content, user_id)
                VALUES (?, ?)
            "#,
            content,
            user_id
        )
            .execute(&self.connection)
            .await?
            .last_insert_rowid();

        let note = sqlx::query_as!(
            Note,
            r#"SELECT id, content, updated_at, created_at, user_id as "user_id: uuid::Uuid" FROM notes WHERE id = ?"#,
            id
        )
            .fetch_one(&self.connection).await?;

        Ok(note)
    }
}