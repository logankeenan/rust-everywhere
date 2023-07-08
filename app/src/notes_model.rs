use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Note {
		pub id: i64,
		pub content: String,
		pub created_at: String,
		pub updated_at: Option<String>,
		pub user_id: Uuid,
}
