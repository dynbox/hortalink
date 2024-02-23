use axum_test::TestServer;
use sqlx::{Pool, Postgres};
use rest_server::json::auth::LoginCreds;
use rest_server::json::notifications::UpdateNotificationPayload;
use crate::common::{login, test_app};

mod common;

#[sqlx::test(fixtures("users", "notifications"))]
async fn test_notifications(pool: Pool<Postgres>) {
    let server = &mut test_app(pool);

    test_read_notifications(server)
        .await;
    get_notifications(server)
        .await;
}

async fn test_read_notifications(server: &TestServer) {
    login(server, LoginCreds {
        email: "john.doe@gmail.com".to_string(),
        password: "secured123456".to_string(),
    })
        .await;
    
    server.patch("/api/v1/users/@me/notifications/1")
        .json(&UpdateNotificationPayload {
            read: true,
        })
        .expect_success()
        .await;

    server.patch("/api/v1/users/@me/notifications/2")
        .json(&UpdateNotificationPayload {
            read: true,
        })
        .expect_failure()
        .await;
}

async fn get_notifications(server: &TestServer) {
    login(server, LoginCreds {
        email: "john.doe@gmail.com".to_string(),
        password: "secured123456".to_string(),
    })
        .await;

    server.get("/api/v1/users/@me/notifications")
        .expect_success()
        .await;
}