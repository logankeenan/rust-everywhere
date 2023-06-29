use sqlx::{Pool, Sqlite};
use uuid::Uuid;
use crate::notes_model::Note;


#[derive(Clone)]
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

    pub async fn search_notes(&self, user_id: Uuid, search_text: String) -> Result<Vec<Note>, sqlx::Error> {
        let formatted_search_text = format!("%{}%", search_text);
        let notes = sqlx::query_as!(
            Note,
            r#"
                SELECT id, content, updated_at, created_at, user_id as "user_id: uuid::Uuid"
                FROM notes
                WHERE user_id = ? AND lower(content) LIKE lower(?)
            "#,
            user_id,
            formatted_search_text
        )
            .fetch_all(&self.connection).await?;

        Ok(notes)
    }

    pub async fn delete_by_id(&self, id: i64, user_id: Uuid) -> Result<(), sqlx::Error>  {
        sqlx::query!(
            r#"
                DELETE FROM notes
                WHERE id = ? AND user_id = ?
            "#,
            id,
            user_id
        )
            .execute(&self.connection).await?;

        Ok(())
    }

}

#[cfg(test)]
mod tests {
    use super::*;
    use uuid::Uuid;

    async fn setup_db() -> Pool<Sqlite> {
        let pool = Pool::connect("sqlite::memory:").await.unwrap();
        sqlx::migrate!("./migrations").run(&pool).await.unwrap();

        pool
    }

    #[tokio::test]
    async fn test_all_notes() {
        let pool = setup_db().await;

        let service = NoteService::new(pool.clone());

        // Assume that we have the function `create_test_note` to create a test note.
        let user_id = Uuid::new_v4();
        service.create_note("Test content".to_string(), user_id).await.unwrap();

        let notes = service.all_notes(user_id).await.unwrap();
        assert_eq!(notes.len(), 1);
    }

    #[tokio::test]
    async fn test_by_id() {
        let pool = setup_db().await;

        let service = NoteService::new(pool.clone());

        let user_id = Uuid::new_v4();
        let note = service.create_note("Test content".to_string(), user_id).await.unwrap();

        let fetched_note = service.by_id(note.id, user_id).await.unwrap();
        assert_eq!(fetched_note.id, note.id);
    }

    #[tokio::test]
    async fn test_update_note() {
        let pool = setup_db().await;

        let service = NoteService::new(pool.clone());

        let user_id = Uuid::new_v4();
        let note = service.create_note("Test content".to_string(), user_id).await.unwrap();

        let updated_note = service.update_note("Updated content".to_string(), note.id, user_id).await.unwrap();
        assert_eq!(updated_note.content, "Updated content");
    }

    #[tokio::test]
    async fn test_create_note() {
        let pool = setup_db().await;

        let service = NoteService::new(pool.clone());

        let user_id = Uuid::new_v4();
        let note = service.create_note("Test content".to_string(), user_id).await.unwrap();

        let fetched_note = service.by_id(note.id, user_id).await.unwrap();
        assert_eq!(fetched_note.id, note.id);
        assert_eq!(fetched_note.content, "Test content");
    }

    #[tokio::test]
    async fn test_search_notes() {
        let pool = setup_db().await;

        let service = NoteService::new(pool.clone());

        let user_id = Uuid::new_v4();
        service.create_note("Test content".to_string(), user_id).await.unwrap();
        service.create_note("Another note".to_string(), user_id).await.unwrap();
        service.create_note("A final note".to_string(), user_id).await.unwrap();

        let notes = service.search_notes(user_id, "note".to_string()).await.unwrap();
        assert_eq!(notes.len(), 2);
    }

    #[tokio::test]
    async fn test_delete_note() {
        let pool = setup_db().await;

        let service = NoteService::new(pool.clone());

        let user_id = Uuid::new_v4();
        let note = service.create_note("Test content".to_string(), user_id).await.unwrap();

        assert!(service.delete_by_id(note.id, user_id).await.is_ok());

        let result = service.by_id(note.id, user_id).await;
        assert!(result.is_err());
    }
}