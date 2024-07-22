use sqlx::{Pool, Postgres};

use crate::json::Command;
use crate::server::session::SocketSession;

mod users;
mod notifications;

pub async fn handle(
    command: Command,
    connections: &mut Vec<SocketSession>,
    pool: &Pool<Postgres>,
) {
    match command {
        Command::Disconnect(addr) =>
            users::disconnect(addr, connections),
        Command::Identify(data, session_id) =>
            users::identify((data, session_id), connections, pool)
                .await,
        Command::NotificationCreated(data) =>
            notifications::created(connections, data)
    }
}