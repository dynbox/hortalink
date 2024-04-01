use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, Instant};

use log::{info, trace, warn};
use sqlx::{Pool, Postgres};
use sqlx::postgres::PgListener;
use tokio::io::{AsyncWriteExt, BufReader};
use tokio::net::TcpListener;
use tokio::sync::{mpsc, Mutex};
use tokio::time::interval;
use web_socket::{Event, WebSocket};

use app_core::database::SqlxManager;
use common::settings::{AppSettings, Protocol};
use common::settings::database::DatabaseSettings;

use crate::events::emitters::{EmitterEvent, listen_postgres};
use crate::events::emitters::notification::{NotificationEvent};
use crate::events::receivers::handle_event;
use crate::handshake::{get_sec_key, response};
use crate::json::event::SocketRequest;
use crate::request::HttpRequest;
use crate::socket::session::SocketSession;

pub struct SocketServer {
    pub settings: AppSettings,
    pub identified_connections: Arc<Mutex<HashMap<i32, SocketSession>>>,
    pub connections: Arc<Mutex<Vec<SocketSession>>>,
    pub pool: Pool<Postgres>,
}

impl SocketServer {
    pub async fn new() -> Self {
        let settings = AppSettings::new("application.toml");
        let pool = SqlxManager::new(&settings.database).await.pool;

        Self {
            settings,
            identified_connections: Arc::new(Mutex::new(HashMap::new())),
            connections: Arc::new(Mutex::new(Vec::new())),
            pool,
        }
    }

    pub async fn run(self) -> Result<(), std::io::Error> {
        let listener = TcpListener::bind(&self.settings.websocket.url()).await?;
        info!(
            "WebSocket server is listening on {}", 
            format!("ws://{}", self.settings.websocket.url())
        );

        listen_postgres(
            &self.settings.database, 
            Arc::clone(&self.identified_connections), 
            self.pool.clone()
        )
            .await
            .expect("TODO: panic message");

        let connections = Arc::clone(&self.connections);

        loop {
            tokio::select! {
                Ok((stream, addr)) = listener.accept() => {
                    let (reader, mut writer) = stream.into_split();
                    let mut reader = BufReader::new(reader);

                    let req = HttpRequest::parse(&mut reader).await?;
                    
                    if let Some(key) = get_sec_key(&req) {
                        let res = response(key, [("x-agent", "web-socket")]);
                        writer.write_all(res.as_bytes()).await?;

                        trace!("[{addr}] successfully connected");

                        // Channel to send events between sockets
                        let (tx, mut rx) = mpsc::unbounded_channel::<SocketRequest>();

                        let mut ws = WebSocket::server(writer);
                        let mut session = SocketSession::new(addr, tx);
                        
                        connections.lock()
                            .await
                            .push(session.clone());
                        
                        tokio::spawn(async move {
                            let mut socket = WebSocket::server(reader);
                            let mut interval = tokio::time::interval(Duration::from_secs(10));
                            
                            loop {
                                tokio::select! {
                                    _ = interval.tick() => {
                                        if Instant::now().duration_since(session.hb) > Duration::new(10, 0) {
                                            trace!("[{addr}] client heartbeat failed, disconnecting!");
                                            break
                                        }

                                        println!("[{addr}] sending ping");
                                        if let Err(e) = ws.send_ping("t").await {
                                            warn!("[{addr}] failed to send ping message: {e}")
                                        }
                                    }
                                    Ok(event) = socket.recv() => {
                                        handle_event(&mut session, &event).await.expect("TODO: panic message");
                                    }
                                }
                            }
                        });

                        tokio::spawn(async move {
                            while let Some(event) = rx.recv().await {
                                match event {
                                    _ => {},
                                }
                            }
                        });
                    }
                },
            }
        }
    }
}