use axum_test::TestServer;
use sqlx::{Pool, Postgres};
use time::Time;
use ::common::entities::WeekDay;
use rest_server::json::auth::LoginCreds;
use rest_server::json::schedules::{CreateSchedulePayload, UpdateSchedulePayload};
use rest_server::json::utils::Location;
use crate::common::{login, test_app};

mod common;

#[sqlx::test(fixtures("users", "sellers", "customers", "schedules", "products"))]
async fn test_schedules(pool: Pool<Postgres>) {
    let server = &mut test_app(pool);

    login(server, LoginCreds {
        email: "sherlock.holmes@proton.me".to_string(),
        password: "secured123456".to_string(),
    })
        .await;

    test_get_schedules(server)
        .await;
    test_post_schedules(server)
        .await;
    test_patch_schedules(server)
        .await;
    test_delete_schedules(server)
        .await;
}

async fn test_get_schedules(server: &TestServer) {
    server.get("/api/v1/sellers/8/schedules")
        .expect_success()
        .await;
}

async fn test_post_schedules(server: &TestServer) {
    let payload = CreateSchedulePayload {
        location: Location {
            latitude: Some(-11.737770),
            longitude: Some(-49.065820)
        },
        address: "Rua S, 230, 77413-420, Gurupi Tocantins Brasil".to_string(),
        start_time: Time::from_hms(12, 00, 00).unwrap(),
        end_time: Time::from_hms(18, 00, 00).unwrap(),
        day_of_week: WeekDay::Monday,
    };
    
    server.post("/api/v1/sellers/8/schedules")
        .json(&payload)
        .await;
}

async fn test_patch_schedules(server: &TestServer) {
    let payload = UpdateSchedulePayload {
        location: None,
        address: None,
        start_time: None,
        end_time: None,
        day_of_week: Some(WeekDay::Monday),
    };
    
    server.patch("/api/v1/sellers/8/schedules/5")
        .expect_success()
        .json(&payload)
        .await;
}

async fn test_delete_schedules(server: &TestServer) {
    server.delete("/api/v1/sellers/8/schedules/5")
        .await;
}