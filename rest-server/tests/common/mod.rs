use axum_test::{TestServer, TestServerConfig};
use sqlx::{Pool, Postgres};
use rest_server::app::server::{Server, AppState};
use rest_server::json::auth::LoginCreds;

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

pub async fn login(server: &TestServer, payload: LoginCreds) {
    server.post("/api/v1/auth/login")
        .json(&payload)
        .expect_success()
        .do_save_cookies()
        .await;
}