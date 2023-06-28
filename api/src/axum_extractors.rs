use std::str::FromStr;
use axum::{
    http,
    extract::{FromRef, FromRequestParts},
    http::request::Parts,
    async_trait,
};
use uuid::Uuid;
use crate::AppState;
use crate::note_service::NoteService;

#[derive(Debug, Clone, Default)]
pub struct UserId(pub Uuid);


#[async_trait]
impl<S> FromRequestParts<S> for UserId
    where
        S: Send + Sync,
{
    type Rejection = http::StatusCode;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        parts.headers
            .get("user-id")
            .ok_or(http::StatusCode::UNAUTHORIZED)
            .and_then(|user_id| user_id.to_str().map_err(|_| http::StatusCode::UNAUTHORIZED))
            .and_then(|user_id| Uuid::from_str(user_id).map_err(|_| http::StatusCode::UNAUTHORIZED))
            .map(UserId)
    }
}


#[async_trait]
impl<S> FromRequestParts<S> for NoteService
    where
        AppState: FromRef<S>,
        S: Send + Sync,
{
    type Rejection = http::StatusCode;

    async fn from_request_parts(_parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let state = AppState::from_ref(&state);

        Ok(NoteService::new(state.pool))
    }
}

