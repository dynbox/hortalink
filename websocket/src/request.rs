use std::collections::HashMap;
use tokio::io::{AsyncBufRead, AsyncBufReadExt};

pub struct HttpRequest {
    pub prefix: String,
    pub headers: HashMap<String, String>,
}

impl HttpRequest {
    pub async fn parse<IO>(reader: &mut IO) -> std::io::Result<Self>
        where
            IO: Unpin + AsyncBufRead,
    {
        let mut lines = reader.lines();

        let prefix = lines.next_line().await?.unwrap();
        let mut headers = HashMap::new();

        while let Some(line) = lines.next_line().await? {
            if line.is_empty() {
                break;
            }
            
            let (key, value) = line.split_once(":").unwrap();
            headers.insert(key.to_ascii_lowercase(), value.trim_start().into());
        }
        
        Ok(Self { prefix, headers })
    }
}