use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, Instant};

use log::{info, trace, warn};
use sqlx::{Pool, Postgres};
use sqlx::postgres::PgListener;
use tokio::io::{AsyncWriteExt, BufReader};
use tokio::net::TcpListener;
use tokio::sync::{mpsc, Mutex};
use web_socket::{Event, WebSocket};

use app_core::database::SqlxManager;
use common::settings::{AppSettings, Protocol};

use crate::events::emitters::{EmitterEvent};
use crate::events::emitters::notification::NotificationEvent;
use crate::events::receivers::{ReceiverEvent};
use crate::events::receivers::heartbeat::HeartbeatEvent;
use crate::events::receivers::identify::IdentifyEvent;
use crate::handshake::{get_sec_key, response};
use crate::json::event::{EventData, SocketRequest};
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

        let ident = Arc::clone(&self.identified_connections);

        let mut pg_listener = PgListener::connect(&self.settings.database.url()).await.unwrap();
        pg_listener.listen("notification_insert").await.unwrap();

        tokio::spawn(async move {
            loop {
                let notification = pg_listener.recv().await.unwrap();

                match notification.channel() {
                    "notification_insert" => {
                        NotificationEvent::execute(
                            &ident,
                            serde_json::from_str(notification.payload()).unwrap()
                        ).await.expect("TODO: panic message");
                    },
                    _ => {},
                }
            }
        });
        
        let pool = Arc::new(self.pool);

        while let Ok((stream, addr)) = listener.accept().await {
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

                let connections = Arc::clone(&self.connections);
                let ident = Arc::clone(&self.identified_connections);
                let pool = Arc::clone(&pool);
                
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
                                    //trace!("[{addr}] client heartbeat failed, disconnecting!");
                                    //break
                                }
                                
                                if let Err(e) = ws.send_ping("t").await {
                                    warn!("[{addr}] failed to send ping message: {e}")
                                }
                            }
                            Ok(event) = socket.recv() => {
                                    match event {
                                        Event::Data { data, .. } => {
                                            let data = serde_json::from_slice::<SocketRequest>(&data).unwrap();
                                
                                            match data.op {
                                                16 => IdentifyEvent::handle(&ident, &connections, data.d.unwrap(), &mut session, &pool).await,
                                                _ => {},
                                            };
                                        }
                                        Event::Pong(_) => HeartbeatEvent::execute(EventData::Heartbeat, &mut session, &pool).await.unwrap(),
                                        Event::Error(_) | Event::Close { .. } => break,
                                        _ => {}
                                };
                            }
                            Some(event) = rx.recv() => {
                                ws.send(serde_json::to_string(&event).unwrap().as_str()).await.unwrap()
                            }
                        }
                    }
                });
            }
        }

        Ok(())
    }
}