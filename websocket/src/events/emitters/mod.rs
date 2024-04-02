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
    type Data;

    async fn execute(
        connections: &Arc<Mutex<HashMap<i32, SocketSession>>>,
        data: Self::Data,
    ) -> Result<Self::Response, SocketError>;
}