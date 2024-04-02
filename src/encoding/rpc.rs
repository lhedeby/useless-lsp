use anyhow::{bail, Context};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::{log, logger};

pub fn encode_message<T: Serialize>(result: T, id: Option<usize>) -> anyhow::Result<String> {
    let response = Response {
        id,
        jsonrpc: "2.0".to_string(),
        result,
    };
    let msg = serde_json::to_string(&response).context("serialize message")?;
    Ok(format!("Content-Length: {}\r\n\r\n{}", msg.len(), msg))
}

pub fn encode_notification<T: Serialize>(param: T, method: String) -> anyhow::Result<String> {
    let notification = Notification {
        jsonrpc: "2.0".to_string(),
        method,
        params: Some(param),
    };

    let msg = serde_json::to_string(&notification).context("serialize message")?;
    Ok(format!("Content-Length: {}\r\n\r\n{}", msg.len(), msg))
}

pub fn decode_message(bytes: &[u8]) -> anyhow::Result<BaseMessage> {
    let s = &String::from_utf8(bytes.to_vec())?;
    let (header, content) = s.split_once("\r\n\r\n").unwrap();
    let length: usize = header["Content-Length: ".len()..]
        .parse()
        .context("Parsing length from header.")?;
    if length != content.len() {
        bail!("Length doesn't match.");
    }
    log!("decoded message: {}", content);

    let msg: BaseMessage = serde_json::from_str(content).context("Deserializing message")?;
    Ok(msg)
}

#[derive(Serialize)]
struct Response<T> {
    id: Option<usize>,
    jsonrpc: String,
    result: T,
}

#[derive(Serialize)]
pub struct Notification<T> {
    pub jsonrpc: String,
    pub method: String,
    pub params: Option<T>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BaseMessage {
    pub jsonrpc: String,
    pub method: String,
    pub params: Option<Value>,
    pub id: Option<usize>,
}
