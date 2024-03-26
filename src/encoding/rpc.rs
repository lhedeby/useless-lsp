use anyhow::{bail, Context};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::{log, logger};

pub fn encode_message<T: Serialize>(message: T) -> anyhow::Result<String> {
    let msg = serde_json::to_string(&message).context("serialize message")?;
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
    log!("content: {}", content);

    let msg: BaseMessage = serde_json::from_str(content).context("Deserializing message")?;
    Ok(msg)
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BaseMessage {
    pub jsonrpc: String,
    pub method: String,
    pub params: Option<Value>,
    pub id: Option<usize>,
}
