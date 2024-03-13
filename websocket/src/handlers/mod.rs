use std::collections::HashMap;
use std::sync::Arc;

use tokio::io::BufReader;
use tokio::net::TcpStream;
use tokio::sync::Mutex;
use web_socket::WebSocket;

pub mod handshake;
mod identify;

pub struct GatewayHandler {
    users: Arc<Mutex<HashMap<i32, Arc<WebSocket<Buff>>>>>,
}

impl GatewayHandler {
    pub fn new() -> Self {
        GatewayHandler {
            users: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub async fn add_user(&self, user_id: i32, socket: WebSocket<Buff>) {
        let mut users = self.users.lock().await;
        users.insert(user_id, Arc::new(socket));
    }

    pub async fn get_user(&self, user_id: &i32) -> Option<Arc<WebSocket<Buff>>> {
        let users = self.users.lock().await;
        users.get(user_id).cloned()
    }

    pub async fn remove_user(&self, user_id: &i32) {
        let mut users = self.users.lock().await;
        users.remove(user_id);
    }
}

pub type Buff = BufReader<TcpStream>;
