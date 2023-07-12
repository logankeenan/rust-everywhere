use std::net::SocketAddr;
use app::create_app;

#[tokio::main]
async fn main() {
    let app = create_app();

    let addr = SocketAddr::from(([0,0,0,0], 3001));

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
