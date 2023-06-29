use axum::body::Body;
use axum::extract::{Path};
use axum::response::{IntoResponse, Response};
use axum::{Json, Router};
use axum::http::{HeaderMap, StatusCode};
use axum::http::header::LOCATION;
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
             user_id: UserId) -> Response {
    match note_service.all_notes(user_id.0).await {
        Ok(notes) => (StatusCode::OK, Json(notes)).into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR).into_response(),
    }
}

async fn by_id(note_service: NoteService,
               Path(id): Path<i64>,
               user_id: UserId) -> Response {
    match note_service.by_id(id, user_id.0).await {
        Ok(note) => (StatusCode::OK, Json(note)).into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR).into_response(),
    }
}

async fn create(note_service: NoteService,
                user_id: UserId,
                Json(payload): Json<NoteForm>) -> Response {
    match payload.validate() {
        Ok(_) => {
            match note_service.create_note(payload.content.clone(), user_id.0).await {
                Ok(note) => (StatusCode::CREATED, Json(note)).into_response(),
                Err(_) => (StatusCode::INTERNAL_SERVER_ERROR).into_response(),
            }
        }
        Err(errors) => (StatusCode::INTERNAL_SERVER_ERROR, errors.to_string()).into_response()
    }
}

async fn update(note_service: NoteService,
                Path(id): Path<i64>,
                user_id: UserId,
                Json(payload): Json<NoteForm>) -> Response {
    match payload.validate() {
        Ok(_) => {
            match note_service.update_note(payload.content.clone(), id, user_id.0).await {
                Ok(_) => {
                    let mut headers = HeaderMap::new();
                    headers.insert(LOCATION, format!("/notes/{}", id).parse().unwrap());

                    (StatusCode::NO_CONTENT, headers).into_response()
                },
                Err(_) => (StatusCode::INTERNAL_SERVER_ERROR).into_response(),
            }
        }
        Err(errors) => (StatusCode::INTERNAL_SERVER_ERROR, errors.to_string()).into_response()
    }
}





