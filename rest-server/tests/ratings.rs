use axum_test::TestServer;
use sqlx::{Pool, Postgres};
use ::common::entities::StarRating;
use rest_server::json::auth::LoginCreds;
use rest_server::json::ratings::{PatchSellerRating, PostSellerProductRating, PostSellerRating};
use rest_server::json::utils::Pagination;
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
    
    test_product_rating(server)
        .await;
}

async fn test_product_rating(server: &TestServer) {
    let query = Pagination {
        page: 1,
        per_page: 50,
    };
    
    server.get("/api/v1/sellers/8/products/8/ratings")
        .add_query_params(query)
        .expect_success()
        .await;
    
    let payload = PostSellerProductRating {
        rating: StarRating::VeryBad,
        content: Some("ooooh my god".to_string()),
    };
    
    server.post("/api/v1/sellers/8/products/1/ratings")
        .json(&payload)
        .expect_success()
        .await;

    server.patch("/api/v1/sellers/8/products/1/ratings/2")
        .json(&payload)
        .expect_success()
        .await;
}