use axum::http::StatusCode;
use axum_test::multipart::{MultipartForm, Part};
use axum_test::TestServer;
use sqlx::{Pool, Postgres};

use rest_server::json::auth::LoginCreds;

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
    let multipart = MultipartForm::new()
        .add_part("name", Part::text("Luis Fernando"))
        .add_part("email", Part::text("baskerbyte@gmail.com"))
        .add_part("password", Part::text("secured123456"))
        .add_part("role", Part::text("4"));

    server.post("/api/v1/auth/sign-in")
        .multipart(multipart)
        .await
        .assert_status(StatusCode::CREATED);

    let multipart = MultipartForm::new()
        .add_part("name", Part::text("Luis Fernando"))
        .add_part("email", Part::text("baskerbyte@gmail.com"))
        .add_part("password", Part::text("secured123456"))
        .add_part("role", Part::text("3"));

    server.post("/api/v1/auth/sign-in")
        .multipart(multipart)
        .await
        .assert_status(StatusCode::CREATED);
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