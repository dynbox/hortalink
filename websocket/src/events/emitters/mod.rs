use std::collections::HashMap;
use std::sync::Arc;

use sqlx::{Pool, Postgres};
use sqlx::postgres::PgListener;
use tokio::sync::Mutex;

use common::settings::database::DatabaseSettings;

use crate::events::emitters::notification::NotificationEvent;
use crate::json::error::SocketError;
use crate::socket::session::SocketSession;

pub mod notification;

pub trait EmitterEvent {
    type Response;

    async fn execute(
        connections: &Arc<Mutex<HashMap<i32, SocketSession>>>,
        pool: &Pool<Postgres>,
        data: String,
    ) -> Result<Self::Response, SocketError>;
}

pub async fn listen_postgres(
    database: &DatabaseSettings,
    connections: Arc<Mutex<HashMap<i32, SocketSession>>>,
    pool: Pool<Postgres>
) -> Result<(), sqlx::Error> {
    let mut listener = PgListener::connect(&database.url()).await?;
    listener.listen("notification_insert").await?;

    tokio::spawn(async move {
        loop {
            let notification = listener.recv().await.unwrap();
            NotificationEvent::execute(&connections, &pool, notification.payload().to_string())
                .await
                .expect("TODO: panic message");
        }
    });

    Ok(())
}