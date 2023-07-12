use uuid::Uuid;
use crate::notes_model::Note;
use reqwest::Client;
use http::header::HeaderValue;


#[cfg(base_url = "public")]
const BASE_URL: &'static str = "https://rust-everywhere-api.logankeenan.com";

#[cfg(not(my_var = "public"))]
const BASE_URL: &'static str = "http://localhost:3000";

pub struct NotesService {
    client: Client,
    base_url: String,
}

impl NotesService {
    pub fn new() -> Self {
        NotesService {
            client: Client::new(),
            base_url: String::from(BASE_URL),
        }
    }

    pub async fn all_notes(&self, user_id: Uuid) -> reqwest::Result<Vec<Note>> {
        let url = format!("{}/notes", self.base_url);
        self.client.get(&url)
            .header("user-id", HeaderValue::from_str(&user_id.to_string()).unwrap())
            .send()
            .await?
            .json::<Vec<Note>>()
            .await
    }

    pub async fn by_id(&self, id: i64, user_id: Uuid) -> reqwest::Result<Note> {
        let url = format!("{}/notes/{}", self.base_url, id);
        match self.client.get(&url)
            .header("user-id", HeaderValue::from_str(&user_id.to_string()).unwrap())
            .send()
            .await {
            Ok(x) => {
                let result = x.json::<Note>()
                    .await;
                result
            }
            Err(err) => {
                Err(err)
            }
        }
    }

    pub async fn update_note(
        &self,
        content: String,
        note_id: i64,
        user_id: Uuid,
    ) -> reqwest::Result<()> {
        let url = format!("{}/notes/{}", self.base_url, note_id);
        let user_id = user_id.to_string();
        self.client.patch(&url)
            .header("user-id", HeaderValue::from_str(&user_id).unwrap())
            .json(&serde_json::json!({ "content": content }))
            .send()
            .await?;

        Ok(())
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
