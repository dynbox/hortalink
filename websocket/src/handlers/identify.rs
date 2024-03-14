use std::io::Error;

use web_socket::WebSocket;

use crate::handlers::Buff;
use crate::handlers::event::WebSocketEvent;
use crate::json::identify::IdentifyPayload;

pub struct IdentifyEvent;

impl WebSocketEvent<IdentifyPayload> for IdentifyEvent {
    async fn handle(
        ws: &WebSocket<Buff>,
        payload: IdentifyPayload,
    ) -> Result<(), Error> {
        /*let context = context.lock().await;
        let data: Option<Vec<u8>> = sqlx::query_scalar("SELECT data FROM tower_sessions.sessions WHERE id = $1")
            .bind(payload.session_id)
            .fetch_optional(&context.pool)
            .await
            .unwrap();

        if let Some(data) = data {
            let user_id = rmp_serde::from_slice(&data).unwrap();
            context.gateway_handler.add_user(user_id, ws).await;
        };
        */
        Ok(())
    }
}