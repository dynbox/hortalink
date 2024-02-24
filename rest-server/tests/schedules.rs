use axum_test::TestServer;
use sqlx::{Pool, Postgres};
use rest_server::json::auth::LoginCreds;
use crate::common::{login, test_app};

mod common;

#[sqlx::test(fixtures("users", "sellers", "schedules"))]
async fn test_schedules(pool: Pool<Postgres>) {
    let server = &mut test_app(pool);

    login(server, LoginCreds {
        email: "sherlock.holmes@proton.me".to_string(),
        password: "secured123456".to_string(),
    })
        .await;

    test_get_schedules(server)
        .await;
}

async fn test_get_schedules(server: &TestServer) {
    server.get("/api/v1/sellers/8/schedules")
        .expect_success()
        .await;
}