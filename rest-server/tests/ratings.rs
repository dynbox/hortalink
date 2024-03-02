use axum_test::TestServer;
use sqlx::{Pool, Postgres};
use ::common::entities::StarRating;
use rest_server::json::auth::LoginCreds;
use rest_server::json::ratings::{PatchRatingPayload, PostRatingPayload};
use crate::common::{login, test_app};

mod common;

#[sqlx::test(fixtures("users", "sellers", "customers", "products", "ratings"))]
async fn test_ratings(pool: Pool<Postgres>) {
    let server = &mut test_app(pool);

    login(server, LoginCreds {
        email: "jane.smith@hotmail.com".to_string(),
        password: "secured123456".to_string(),
    })
        .await;

    test_seller_rating(server)
        .await;
}

async fn test_seller_rating(server: &TestServer) {
    let payload = PatchRatingPayload {
        rating: Some(StarRating::VeryBad.into()),
        tags: None,
    };

    server.patch("/api/v1/sellers/8/ratings/1")
        .json(&payload)
        .expect_success()
        .await;

    let payload = PostRatingPayload {
        rating: StarRating::VeryBad.into(),
        tags: None,
    };

    server.post("/api/v1/sellers/8/ratings")
        .json(&payload)
        .expect_success()
        .await;

    server.delete("/api/v1/sellers/8/ratings/1")
        .expect_success()
        .await;
}