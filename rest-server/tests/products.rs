use axum_test::TestServer;
use sqlx::{Pool, Postgres};
use sqlx::types::Decimal;

use rest_server::json::auth::LoginCreds;
use rest_server::json::products::{FilterProducts, PatchSellerProduct, PostSellerProduct};

use crate::common::{login, test_app};

mod common;

#[sqlx::test(fixtures("users", "sellers", "customers", "products", "ratings", "schedules"))]
async fn test_products(pool: Pool<Postgres>) {
    let server = &mut test_app(pool);

    login(server, LoginCreds {
        email: "sherlock.holmes@proton.me".to_string(),
        password: "secured123456".to_string(),
    })
        .await;

    test_get_product(server)
        .await;
    test_patch_product(server)
        .await;
    test_post_product(server)
        .await;
}

async fn test_get_product(server: &TestServer) {
    server.get("/api/v1/sellers/8/products/8")
        .expect_success()
        .await;

    let query = FilterProducts {
        max_price: None,
        min_price: None,
        min_stars: None,
        product_type: None,
        start_time: None,
        day_of_week: None,
        page: 1,
        per_page: 10,
        latitude: None,
        longitude: None,
    };

    server.get("/api/v1/products")
        .add_query_params(query)
        .await;
}

async fn test_patch_product(server: &TestServer) {
    let payload = PatchSellerProduct {
        price: Some(Decimal::new(11, 1)),
        quantity: None,
        photos: None,
        remove_schedules: Some(vec![5]),
        add_schedules: Some(vec![1, 2]),
    };

    server.patch("/api/v1/sellers/8/products/8")
        .json(&payload)
        .expect_success()
        .await;
}

async fn test_post_product(server: &TestServer) {
    let payload = PostSellerProduct {
        product_id: 1,
        price: Some(Decimal::new(11, 1)),
        quantity: None,
        photos: vec![String::new()],
        schedules: None,
    };

    server.post("/api/v1/sellers/8/products")
        .json(&payload)
        .await;
}