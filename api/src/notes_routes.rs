use std::time::Duration;
use axum::body::Body;
use axum::extract::{Path, Query};
use axum::response::{IntoResponse, Response};
use axum::{Json, Router};
use axum::http::{HeaderMap, StatusCode};
use axum::http::header::LOCATION;
use axum::routing::{get, post, patch};
use crate::AppState;
use crate::axum_extractors::UserId;
use crate::note_service::NoteService;
use serde::Deserialize;
use sqlx::Error;
use tokio::time::sleep;
use uuid::Uuid;
use validator::{Validate};
use log::{error};

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

#[derive(Deserialize)]
struct SearchQuery {
    q: Option<String>,
}

async fn all(note_service: NoteService,
             user_id: UserId,
             Query(search_query): Query<SearchQuery>) -> Response {
    if let Some(query) = &search_query.q {
        match note_service.search_notes(user_id.0, query.clone()).await {
            Ok(notes) => (StatusCode::OK, Json(notes)).into_response(),
            Err(error) => {
                error!("line: {:?}, error: {:?}", line!(), error);
                (StatusCode::INTERNAL_SERVER_ERROR).into_response()
            },
        }
    } else {
        match note_service.all_notes(user_id.0).await {
            Ok(notes) => (StatusCode::OK, Json(notes)).into_response(),
            Err(error) => {
                error!("line: {:?}, error: {:?}", line!(), error);
                (StatusCode::INTERNAL_SERVER_ERROR).into_response()
            },
        }
    }
}

async fn by_id(note_service: NoteService,
               Path(id): Path<i64>,
               user_id: UserId) -> Response {
    match note_service.by_id(id, user_id.0).await {
        Ok(note) => (StatusCode::OK, Json(note)).into_response(),
        Err(err) => {
            match err {
                Error::RowNotFound => {
                    (StatusCode::NOT_FOUND).into_response()
                }
                error => {
                    error!("line: {:?}, error: {:?}", line!(), error);
                    (StatusCode::INTERNAL_SERVER_ERROR).into_response()
                }
            }
        },
    }
}

async fn create(note_service: NoteService,
                user_id: UserId,
                Json(payload): Json<NoteForm>) -> Response {
    match payload.validate() {
        Ok(_) => {
            match note_service.create_note(payload.content.clone(), user_id.0).await {
                Ok(note) => {
                    delete_note_after_15_mins(note_service.clone(), note.id, user_id.0);

                    (StatusCode::CREATED, Json(note)).into_response()
                }
                Err(error) => {
                    error!("line: {:?}, error: {:?}", line!(), error);
                    (StatusCode::INTERNAL_SERVER_ERROR).into_response()
                },
            }
        }
        Err(errors) => {
            (StatusCode::BAD_REQUEST, errors.to_string()).into_response()
        }
    }
}

fn delete_note_after_15_mins(cloned_service: NoteService, note_id: i64, user_id: Uuid) {
    tokio::spawn(async move {
        sleep(Duration::from_secs(15 * 60)).await;
        match cloned_service.delete_by_id(note_id, user_id).await {
            Ok(_) => (),
            Err(_) => error!("Error deleting note after 15 minutes"),
        }
    });
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
                }
                Err(error) => {
                    error!("line: {:?}, error: {:?}", line!(), error);
                    (StatusCode::INTERNAL_SERVER_ERROR).into_response()
                },
            }
        }
        Err(errors) => (StatusCode::BAD_REQUEST, errors.to_string()).into_response()
    }
}





