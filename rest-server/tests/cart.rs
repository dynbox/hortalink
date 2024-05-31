use axum_test::TestServer;
use sqlx::{Pool, Postgres};
use rest_server::json::auth::LoginCreds;
use rest_server::json::cart::{PatchProductCart, PostProductCart};
use crate::common::{login, test_app};

mod common;

#[sqlx::test(fixtures("users", "customers", "sellers", "schedules", "products", "cart"))]
async fn test_cart(pool: Pool<Postgres>) {
    let server = &mut test_app(pool);
    
    login(server, LoginCreds {
        email: "sherlock.holmes@proton.me".to_string(),
        password: "secured123456".to_string(),
    })
        .await;

    test_seller_orders(server)
        .await;

    server.clear_cookies();
    
    login(server, LoginCreds {
        email: "diana.prince@aol.com".to_string(),
        password: "secured123456".to_string(),
    })
        .await;
    
    test_customer_orders(server)
        .await;
}

async fn test_seller_orders(server: &TestServer) {
    server.get("/api/v1/users/@me/orders")
        .expect_success()
        .await;

    server.delete("/api/v1/users/@me/orders/3")
        .expect_success()
        .await;

    server.delete("/api/v1/users/@me/orders/2")
        .expect_failure()
        .await;
}

async fn test_customer_orders(server: &TestServer) {
    server.get("/api/v1/users/@me/cart")
        .expect_success()
        .await;

    server.delete("/api/v1/users/@me/cart/1")
        .expect_success()
        .await;

    let payload = PatchProductCart {
        withdrawn: None,
        amount: Some(2),
    };
    
    server.patch("/api/v1/users/@me/cart/2")
        .json(&payload)
        .expect_success()
        .await;

    let payload = PostProductCart {
        seller_id: 8,
        seller_product_id: 10,
        withdrawn: 2,
        amount: 1,
    };

    server.post("/api/v1/users/@me/cart")
        .json(&payload)
        .expect_success()
        .await;

    let payload = PostProductCart {
        seller_id: 9,
        seller_product_id: 9,
        withdrawn: Default::default(),
        amount: 1,
    };

    server.post("/api/v1/users/@me/cart")
        .expect_failure()
        .json(&payload)
        .await;
}