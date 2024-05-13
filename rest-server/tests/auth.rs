use axum_test::TestServer;
use sqlx::{Pool, Postgres};
use ::common::entities::UserRole;
use rest_server::json::auth::{LoginCreds, SignCreds};
use crate::common::test_app;

mod common;

#[sqlx::test(fixtures("users"))]
async fn test_auth(pool: Pool<Postgres>) {
    let server = &mut test_app(pool);

    test_sign_in(server)
        .await;
    test_login(server)
        .await;
    test_logout(server)
        .await;
}

async fn test_sign_in(server: &TestServer) {
    let payload = SignCreds {
        name: "Luis Fernando".to_string(),
        email: "baskerbyte@gmail.com".to_string(),
        password: Some("secured123456".to_string()),
        avatar: None,
        role: UserRole::Customer,
    };

    server.post("/api/v1/auth/sign-in")
        .json(&payload)
        .expect_success()
        .await;

    server.post("/api/v1/auth/sign-in")
        .json(&payload)
        .expect_failure()
        .await;
}

async fn test_login(server: &TestServer) {
    let payload = LoginCreds {
        email: "baskerbyte@gmail.com".to_string(),
        password: "secured123456".to_string(),
    };

    server.post("/api/v1/auth/login")
        .json(&payload)
        .expect_success()
        .await;

    let payload = LoginCreds {
        email: "aaaaa@gmail.com".to_string(),
        password: "secured123456".to_string(),
    };

    server.post("/api/v1/auth/login")
        .json(&payload)
        .expect_failure()
        .await;
}

async fn test_logout(server: &mut TestServer) {
    server.get("/api/v1/auth/logout")
        .expect_success()
        .await;

    server.clear_cookies();

    server.get("/api/v1/auth/logout")
        .expect_failure()
        .await;
}