use axum_test::TestServer;
use serde_json::Value;
use sqlx::{Pool, Postgres};
use rest_server::json::auth::LoginCreds;
use crate::common::{login, test_app};

mod common;

#[sqlx::test(fixtures("users", "customers", "sellers", "blacklist"))]
async fn test_users(pool: Pool<Postgres>) {
    let server = &mut test_app(pool);

    test_user_me(server)
        .await;
}

async fn test_user_me(server: &TestServer) {
    let file = std::fs::read_to_string("tests/json/user_me.json")
        .unwrap();
    let user_json: Value = serde_json::from_str(&file).unwrap();

    let users = vec![
        ("john.doe@gmail.com", "secured123456"),
        ("jane.smith@hotmail.com", "secured123456"),
        ("bob.johnson@outlook.com", "secured123456"),
        ("alice.williams@yahoo.com", "secured123456"),
        ("charlie.brown@hotmail.com", "secured123456"),
        ("diana.prince@aol.com", "secured123456"),
        ("harry.potter@gmail.com", "secured123456"),
        ("sherlock.holmes@proton.me", "secured123456"),
        ("elizabeth.bennet@gmail.com", "secured123456"),
        ("frodo.baggins@icloud.com", "secured123456"),
    ];

    for (i, (email, password)) in users.iter().enumerate() {
        login(server, LoginCreds {
            email: email.to_string(),
            password: password.to_string(),
        })
            .await;

        let res = server.get("/api/v1/users/@me")
            .expect_success()
            .await;

        assert_eq!(res.json::<Value>(), user_json.get(i.to_string()).unwrap().clone());
    }
}