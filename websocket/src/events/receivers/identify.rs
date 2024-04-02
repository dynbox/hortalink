use std::collections::HashMap;
use std::sync::Arc;
use sqlx::{Pool, Postgres};
use tokio::sync::Mutex;
use crate::events::receivers::ReceiverEvent;
use crate::json::error::SocketError;
use crate::json::event::EventData;
use crate::json::user::{Record};
use crate::socket::session::SocketSession;

pub struct IdentifyEvent;

impl ReceiverEvent for IdentifyEvent {
    type Response = Option<i32>;

    async fn execute(
        data: EventData, 
        _: &mut SocketSession, 
        pool: &Pool<Postgres>
    ) -> Result<Self::Response, SocketError> {
        let session_value = match data {
            EventData::Identify { session } => session,
            _ => return Err(SocketError::Custom("Invalid event data for IdentifyEvent".to_string())),
        };
        
        let session_data: Option<Vec<u8>> = sqlx::query_scalar(
            r#"
                SELECT data FROM "tower_sessions"."sessions"
                WHERE id = $1
            "#
        )
            .bind(session_value)
            .fetch_optional(pool)
            .await
            .unwrap();

        if let Some(data) = session_data {
            Ok(Some(
                rmp_serde::from_slice::<Record>(&data).unwrap()
                    .data["axum-login.data"]["user_id"].as_i64().unwrap() as i32
            ))
        } else {
            Ok(None)
        }
    }
}

impl IdentifyEvent {
    pub async fn handle(
        identified_connections: &Arc<Mutex<HashMap<i32, SocketSession>>>,
        connections: &Arc<Mutex<Vec<SocketSession>>>,
        data: EventData, 
        session: &mut SocketSession,
        pool: &Pool<Postgres>
    ) {
        if let Ok(Some(user_id)) = Self::execute(data, session, pool).await {
            let mut con = identified_connections.lock().await;
            
            if con.iter().any(|(user, _)| user == &user_id) {
                return;
            }
            
            let mut con_vec = connections.lock().await;
            
            if let Some(pos) = con_vec.iter().position(|s| s.addr == session.addr) {
                let session_to_move = con_vec.remove(pos);
                con.insert(user_id, session_to_move);
            }
        }
    }
}