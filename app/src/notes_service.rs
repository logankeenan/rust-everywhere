use std::env;
use uuid::Uuid;
use crate::notes_model::Note;
use reqwest::Client;
use http::header::HeaderValue;

pub struct NotesService {
    client: Client,
    base_url: String,
}

impl NotesService {
    pub fn new() -> Self {
        let base_url = env::var("BASE_URL")
            .unwrap_or_else(|_| String::from("http://localhost:8000"));

        NotesService {
            client: Client::new(),
            base_url
        }
    }

    async fn fetch_notes(&self, path: &str, user_id: Uuid) -> reqwest::Result<Vec<Note>> {
        let url = format!("{}/{}", self.base_url, path);
        let user_id = user_id.to_string();
        self.client.get(&url)
            .header("user-id", HeaderValue::from_str(&user_id).unwrap())
            .send()
            .await?
            .json::<Vec<Note>>()
            .await
    }

    pub async fn all_notes(&self, user_id: Uuid) -> reqwest::Result<Vec<Note>> {
        self.fetch_notes("notes", user_id).await
    }

    pub async fn by_id(&self, id: Uuid, user_id: Uuid) -> reqwest::Result<Option<Note>> {
        let path = format!("notes/{}", id);
        self.fetch_notes(&path, user_id).await
            .map(|notes| notes.into_iter().next())
    }

    pub async fn update_note(
        &self,
        content: String,
        note_id: i64,
        user_id: Uuid,
    ) -> reqwest::Result<Note> {
        let url = format!("{}/notes/{}", self.base_url, note_id);
        let user_id = user_id.to_string();
        self.client.patch(&url)
            .header("user-id", HeaderValue::from_str(&user_id).unwrap())
            .json(&serde_json::json!({ "content": content }))
            .send()
            .await?
            .json::<Note>()
            .await
    }

    pub async fn create_note(
        &self,
        content: String,
        user_id: Uuid,
    ) -> reqwest::Result<Note> {
        let url = format!("{}/notes", self.base_url);
        let user_id = user_id.to_string();
        self.client.post(&url)
            .header("user-id", HeaderValue::from_str(&user_id).unwrap())
            .json(&serde_json::json!({ "content": content }))
            .send()
            .await?
            .json::<Note>()
            .await
    }

}
