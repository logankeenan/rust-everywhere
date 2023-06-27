use sqlx::FromRow;
use chrono::NaiveDateTime;
use uuid::Uuid;

#[derive(Debug, Clone, FromRow)]
pub struct Note {
		pub id: i64,
		pub content: String,
		pub created_at: NaiveDateTime,
		pub updated_at: Option<NaiveDateTime>,
		pub user_id: Uuid,
}
