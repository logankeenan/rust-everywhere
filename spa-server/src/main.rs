use std::net::SocketAddr;
use app::create_app;
use tower_http::{
    services::{ServeDir, ServeFile},
};

#[tokio::main]
async fn main() {
    let app = create_app()
        .nest_service("/sw.js", ServeFile::new("sw.js"))
        .nest_service("/dist", ServeDir::new("dist"));

    let addr = SocketAddr::from(([0, 0, 0, 0], 3002));

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
