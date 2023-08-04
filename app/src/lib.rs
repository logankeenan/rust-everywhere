use axum::{routing::get, routing::post, Router as AxumRouter, middleware, Router};
use crate::axum_middleware::set_user_id_cookie;
use crate::notes_routes::{create_note, edit_note, index, search_note, show_note, update_note};

mod axum_middleware;
mod axum_extractors;
mod notes_model;
mod notes_routes;
mod notes_service;


pub fn create_app() -> Router {
    let router: AxumRouter = AxumRouter::new()
        .route("/", get(index))
        .route("/create", post(create_note))
        .route("/update", post(update_note))
        .route("/show/:id", get(show_note))
        .route("/edit/:id", get(edit_note))
        .route("/search", get(search_note))
        .layer(middleware::from_fn(set_user_id_cookie));

    router
}
