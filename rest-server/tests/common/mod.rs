use axum_test::{TestServer, TestServerConfig};
use sqlx::{Pool, Postgres};
use rest_server::app::web::{Server, AppState};

pub fn test_app(pool: Pool<Postgres>) -> TestServer {
    let config = TestServerConfig::builder()
        .save_cookies()
        .mock_transport()
        .build();

    TestServer::new_with_config(
        Server::router(AppState { settings: Default::default(), pool }),
        config
    )
        .unwrap()
}