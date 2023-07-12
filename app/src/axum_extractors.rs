use cookie::{Cookie, CookieJar};
use axum::{
	http,
	extract::{FromRequestParts},
	http::header::COOKIE,
	http::request::Parts,
	async_trait,
};
use uuid::Uuid;
use crate::notes_service::NotesService;

#[derive(Debug, Clone, Default)]
pub struct UserId(pub Uuid);


#[async_trait]
impl<S> FromRequestParts<S> for UserId
    where
        S: Send + Sync,
{
    type Rejection = http::StatusCode;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        if let Some(cookie_header) = parts.headers.get(COOKIE) {
            let mut cookie_jar = CookieJar::new();
            let cookie_str = cookie_header.to_str().unwrap_or_default();

            for cookie in cookie_str.split(';').map(|c| c.trim()) {
                if let Ok(parsed_cookie) = Cookie::parse(cookie.to_string()) {
                    cookie_jar.add_original(parsed_cookie);
                }
            }

            if let Some(cookie) = cookie_jar.get("user_id") {
                Ok(UserId(cookie.value().to_string().parse().unwrap()))
            } else {
                Err(http::StatusCode::BAD_REQUEST)
            }
        } else {
            Err(http::StatusCode::BAD_REQUEST)
        }
    }
}

#[async_trait]
impl<S> FromRequestParts<S> for NotesService
    where
        S: Send + Sync,
{
    type Rejection = http::StatusCode;

    async fn from_request_parts(_parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        Ok(NotesService::new())
    }
}


