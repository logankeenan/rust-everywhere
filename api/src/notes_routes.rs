use axum::body::Body;
use axum::extract::{Path};
use axum::response::IntoResponse;
use axum::{Json, Router};
use axum::http::StatusCode;
use axum::routing::{get, post, patch};
use crate::AppState;
use crate::axum_extractors::UserId;
use crate::note_service::NoteService;
use serde::Deserialize;
use validator::{Validate};

pub fn notes_routes(state: AppState) -> Router<AppState, Body> {
    Router::new()
        .route("/notes", get(all))
        .route("/notes", post(create))
        .route("/notes/:id", patch(update))
        .route("/notes/:id", get(by_id))
        .with_state(state)
}

#[derive(Validate, Deserialize)]
struct NoteForm {
    #[validate(length(min = 1, message = "Content is too short. It must be at least 1 characters long."))]
    #[validate(length(max = 1000, message = "Content is too long. It must be no more than 1000 characters long."))]
    content: String,
}

async fn all(note_service: NoteService,
             user_id: UserId) -> Result<impl IntoResponse, StatusCode> {
    match note_service.all_notes(user_id.0).await {
        Ok(notes) => Ok(Json(notes)),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

async fn by_id(note_service: NoteService,
               Path(id): Path<i64>,
               user_id: UserId) -> Result<impl IntoResponse, StatusCode> {
    match note_service.by_id(id, user_id.0).await {
        Ok(note) => Ok(Json(note)),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

async fn create(note_service: NoteService,
                user_id: UserId,
                Json(payload): Json<NoteForm>) -> Result<impl IntoResponse, StatusCode> {
    match payload.validate() {
        Ok(_) => {
            match note_service.create_note(payload.content.clone(), user_id.0).await {
                Ok(note) => Ok(Json(note)),
                Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
            }
        }
        Err(_) => Err(StatusCode::BAD_REQUEST)
    }
}

async fn update(note_service: NoteService,
                Path(id): Path<i64>,
                user_id: UserId,
                Json(payload): Json<NoteForm>) -> Result<impl IntoResponse, StatusCode> {
    match payload.validate() {
        Ok(_) => {
            match note_service.update_note(payload.content.clone(), id, user_id.0).await {
                Ok(note) => Ok(Json(note)),
                Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
            }
        }
        Err(_) => Err(StatusCode::BAD_REQUEST),
    }
}





