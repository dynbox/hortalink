use std::collections::HashMap;
use std::io::Error;
use std::net::SocketAddr;

use log::{error, info, trace, warn};
use sqlx::{Pool, Postgres};
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::{TcpListener, TcpStream};
use web_socket::{CloseCode, DataType, Event, Stream, WebSocket};

use app_core::database::SqlxManager;
use common::settings::{AppSettings, Protocol};

use crate::handlers::Buff;
use crate::handlers::event::handle_event;
use crate::handlers::handshake::response;

pub struct Server {
    settings: AppSettings,
    pool: Pool<Postgres>,
}

impl Server {
    pub async fn new() -> Self {
        let settings = AppSettings::new("application.toml");
        let pool = SqlxManager::new(&settings.database).await.pool;

        Self {
            settings,
            pool,
        }
    }

    pub async fn run(&self) -> Result<(), Error> {
        let listener = TcpListener::bind(&self.settings.websocket.url()).await?;
        info!("Listening at {}", &self.settings.websocket.url());

        loop {
            match listener.accept().await {
                Ok((stream, addr)) => {
                    handle_connection(stream, addr).await;
                }
                Err(e) => {
                    error!("Error accepting connection: {}", e);
                }
            }
        }
    }
}

async fn handle_connection(stream: TcpStream, addr: SocketAddr) {
    let mut stream = BufReader::new(stream);

    match parse_headers(&mut stream).await {
        Ok(headers) => {
            if let Some(key) = get_sec_key(&headers) {
                if let Err(e) = stream
                    .write_all(response(key, [("x-agent", "web-socket")]).as_bytes())
                    .await {
                    error!("[{addr}] failed to send response {e}");
                } else {
                    tokio::spawn(async move {
                        let mut ws = WebSocket::server(stream);
                        let mut buf = Vec::with_capacity(4096);

                        trace!("[{addr}] connection successfully established");

                        loop {
                            match ws.recv_event().await {
                                Ok(event) => match event {
                                    Event::Data { ty, data } => match ty {
                                        DataType::Complete(_) =>
                                            handle_event(&ws, &data).await,
                                        DataType::Stream(stream) => {
                                            buf.extend_from_slice(&data);

                                            if let Stream::End(_) = stream {
                                                handle_event(&ws, &buf).await;
                                                buf.clear();
                                            }
                                        }
                                    }
                                    Event::Ping(data) => ws.send_pong(data).await?,
                                    Event::Pong(_) => {}
                                    Event::Error(_) => return ws.close(CloseCode::ProtocolError).await,
                                    Event::Close { .. } => return ws.close(()).await,
                                }
                                Err(e) => {
                                    error!("[{addr}] WebSocket error: {e}");

                                    return ws.close(CloseCode::ProtocolError).await;
                                }
                            }
                        }
                    });
                }
            } else {
                warn!("[{addr}] expected websocket upgrade request");
            }
        }
        Err(e) => {
            warn!("[{addr}] failed to parse connection headers: {e}");
        }
    }
}

async fn parse_headers(reader: &mut Buff) -> Result<HashMap<String, String>, Error> {
    let mut headers = HashMap::new();
    let mut lines = reader.lines();

    lines.next_line().await?;

    while let Some(line) = lines.next_line().await? {
        if line == "" {
            break;
        }

        let (key, value) = line.split_once(":").unwrap();
        headers.insert(key.to_ascii_lowercase(), value.trim_start().into());
    }

    Ok(headers)
}

fn get_sec_key(headers: &HashMap<String, String>) -> Option<&String> {
    if !headers.get("connection")?.eq_ignore_ascii_case("upgrade")
        || !headers.get("upgrade")?.eq_ignore_ascii_case("websocket")
    {
        return None;
    }

    headers.get("sec-websocket-key")
}