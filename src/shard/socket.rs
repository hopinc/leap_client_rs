#[cfg(feature = "zlib")]
use std::io::Cursor;

#[cfg(feature = "zlib")]
use async_compression::tokio::bufread::ZlibDecoder;
use async_trait::async_trait;
use async_tungstenite::tokio::ConnectStream;
use async_tungstenite::tungstenite::Message;
use async_tungstenite::WebSocketStream;
use futures::{SinkExt, StreamExt};
use serde_json::{json, to_string, Value};
use tokio::io::AsyncReadExt;
use tokio::time::{timeout, Duration};

use super::error::Error as GatewayError;
use crate::errors::{Error, Result};
use crate::shard::types::OpCode;

pub type WsStream = WebSocketStream<ConnectStream>;

#[async_trait]
pub trait RecieverExt {
    async fn recieve_json(&mut self) -> Result<Option<Value>>;
}

#[async_trait]
pub trait SenderExt {
    async fn send_json(&mut self, value: &Value) -> Result<()>;
}

#[async_trait]
impl RecieverExt for WsStream {
    async fn recieve_json(&mut self) -> Result<Option<Value>> {
        const TIMEOUT: Duration = Duration::from_millis(0);

        match timeout(TIMEOUT, self.next()).await {
            Ok(Some(Ok(message))) => convert_message(message).await,
            Ok(Some(Err(error))) => return Err(error.into()),
            Ok(None) | Err(_) => return Ok(None),
        }
    }
}

async fn convert_message(message: Message) -> Result<Option<Value>> {
    let bytes = match message {
        #[cfg(feature = "zlib")]
        Message::Binary(binary) => {
            let mut compressed = ZlibDecoder::new(Cursor::new(binary));
            let mut buffer = vec![];

            compressed.read_to_end(&mut buffer).await?;

            buffer
        }

        Message::Text(text) => text.into_bytes(),

        Message::Close(frame) => {
            return Err(Error::Gateway(GatewayError::Closed(frame)));
        }

        _ => return Ok(None),
    };

    log::debug!(
        "[Shard] Received raw data: {}",
        String::from_utf8_lossy(&bytes)
    );

    Ok(Some(serde_json::from_slice(&bytes)?))
}

#[async_trait]
impl SenderExt for WsStream {
    async fn send_json(&mut self, value: &Value) -> Result<()> {
        log::debug!("[Shard] Sending: {value}");

        Ok(to_string(value)
            .map(Message::Text)
            .map_err(Error::from)
            .map(|m| self.send(m))?
            .await?)
    }
}

#[async_trait]
pub trait WsStreamExt {
    async fn send_heartbeat(&mut self, tag: Option<&str>) -> Result<()>;
    async fn send_identify(&mut self, project: &str, token: Option<&str>) -> Result<()>;
}

#[async_trait]
impl WsStreamExt for WsStream {
    async fn send_heartbeat(&mut self, tag: Option<&str>) -> Result<()> {
        let payload = if let Some(tag) = tag {
            json!({
                "op": OpCode::Heartbeat.number(),
                "d": {
                    "tag": tag,
                },
            })
        } else {
            json!({
                "op": OpCode::Heartbeat.number(),
            })
        };

        self.send_json(&payload).await
    }

    async fn send_identify(&mut self, project: &str, token: Option<&str>) -> Result<()> {
        let payload = if let Some(token) = token {
            json!({
                "op": OpCode::Identify.number(),
                "d": {
                    "project_id": project,
                    "token": token,
                },
            })
        } else {
            json!({
                "op": OpCode::Identify.number(),
                "d": {
                    "project_id": project,
                },
            })
        };

        self.send_json(&payload).await
    }
}
