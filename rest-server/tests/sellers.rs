use axum_test::TestServer;
use sqlx::{Pool, Postgres};
use rest_server::json::auth::LoginCreds;
use crate::common::{login, test_app};

mod common;

#[sqlx::test(fixtures("users", "sellers", "customers", "schedules", "products", "ratings"))]
async fn test_sellers(pool: Pool<Postgres>) {
    let server = &mut test_app(pool);

    login(server, LoginCreds {
        email: "john.doe@gmail.com".to_string(),
        password: "secured123456".to_string(),
    })
        .await;

    test_get_seller(&server)
        .await;
}

async fn test_get_seller(server: &TestServer) {
    let res = server.get("/api/v1/users/8")
        .await;

    println!("{}", res.text());
}