use sha1::{Digest, Sha1};
use crate::request::HttpRequest;

pub trait Header {
    /// Format a single http header field
    fn fmt(_: &Self) -> String;
}

pub fn response(
    sec_ws_key: impl AsRef<[u8]>,
    headers: impl IntoIterator<Item=impl Header>,
) -> String {
    let key = accept_key_from(sec_ws_key);
    let headers: String = headers.into_iter().map(|f| Header::fmt(&f)).collect();
    format!("HTTP/1.1 101 Switching Protocols\r\nUpgrade: websocket\r\nConnection: Upgrade\r\nSec-WebSocket-Accept: {key}\r\n{headers}\r\n")
}

pub fn get_sec_key(req: &HttpRequest) -> Option<&String> {
    if !req.headers.get("connection")?.eq_ignore_ascii_case("upgrade")
        || !req.headers.get("upgrade")?.eq_ignore_ascii_case("websocket")
    {
        return None;
    }
    req.headers.get("sec-websocket-key")
}

fn accept_key_from(sec_ws_key: impl AsRef<[u8]>) -> String {
    let mut sha1 = Sha1::new();
    sha1.update(sec_ws_key.as_ref());
    sha1.update(b"258EAFA5-E914-47DA-95CA-C5AB0DC85B11");

    base64_encode(sha1.finalize())
}

fn base64_encode(string: impl AsRef<[u8]>) -> String {
    base64::Engine::encode(&base64::prelude::BASE64_STANDARD, string)
}

impl<T: std::fmt::Display> Header for [T; 2] {
    fn fmt([key, value]: &Self) -> String {
        format!("{key}: {value}\r\n")
    }
}

impl<K: std::fmt::Display, V: std::fmt::Display> Header for (K, V) {
    fn fmt((key, value): &Self) -> String {
        format!("{key}: {value}\r\n")
    }
}