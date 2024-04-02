use std::io::{BufRead, Write};

use encoding::rpc::BaseMessage;
use lsp::{notification::traits::Notification, request::traits::Request};
use serde::{de::DeserializeOwned, Serialize};
use state::State;

use crate::{
    encoding::rpc,
    lsp::{
        notification::document_didopen::{DidOpenTextDocumentNotification, PublishDiagnosticsParams}, request::{
            hover::{HoverRequest, HoverResult},
            initialize::{InitializeRequest, InitializeResult},
        }
    },
};

pub mod encoding;
pub mod logger;
pub mod lsp;
pub mod state;

fn main() -> anyhow::Result<()> {
    log!("Lsp started...");

    let mut stdin = std::io::stdin().lock();
    let mut state = State::new();

    loop {
        let buffer = stdin.fill_buf()?;

        if let Ok(content) = rpc::decode_message(buffer) {
            log!("Handling method '{}'", content.method);
            match content.method.as_str() {
                "shutdown" => {
                    break;
                }
                "initialize" => {
                    handle_request::<InitializeRequest, InitializeResult>(content, &mut state)?
                }
                "textDocument/didOpen" => {
                    handle_notification::<DidOpenTextDocumentNotification, PublishDiagnosticsParams>(content, &mut state, "textDocument/publishDiagnostics")?
                }
                "textDocument/hover" => {
                    handle_request::<HoverRequest, HoverResult>(content, &mut state)?
                }
                _ => log!("Method '{}' not implemented", content.method),
            }
            log!("Handling done");
            let length = buffer.len();
            stdin.consume(length);
        }
    }
    log!("Lsp stopped...");
    Ok(())
}

fn handle_request<R, T>(message: BaseMessage, state: &mut State) -> anyhow::Result<()>
where
    R: DeserializeOwned + Request<T>,
    T: Serialize,
{
    let req: R = serde_json::from_value(message.params.unwrap())?;
    let result = req.handle(state)?;

    let mut stdout = std::io::stdout().lock();
    let encoded_message = encoding::rpc::encode_message(result, message.id)?;
    log!("RESPONSE SENT: {}", encoded_message);
    stdout.write_all(encoded_message.as_bytes())?;
    stdout.flush()?;
    Ok(())
}

fn handle_notification<N, T>(message: BaseMessage, state: &mut State, method: &str) -> anyhow::Result<()>
where
    N: DeserializeOwned + Notification<T>,
    T: Serialize,
{
    let notification: N = serde_json::from_value(message.params.unwrap())?;
    if let Some(result) = notification.handle(state)? {
        let mut stdout = std::io::stdout().lock();
        let encoded_message = encoding::rpc::encode_notification(result, method.to_string())?;
        log!("NOTIFICATION SENT: {}", encoded_message);
        stdout.write_all(encoded_message.as_bytes())?;
        stdout.flush()?;
    }
    Ok(())
}
