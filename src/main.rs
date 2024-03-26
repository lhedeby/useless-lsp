use std::io::{BufRead, Write};

use encoding::rpc::BaseMessage;
use serde::Serialize;

use crate::{
    encoding::rpc,
    lsp::initialize::{get_init_response, InitializeRequest},
};

pub mod encoding;
pub mod logger;
pub mod lsp;

fn main() -> anyhow::Result<()> {
    log!("Lsp started...");

    let mut stdin = std::io::stdin().lock();

    loop {
        let buffer = stdin.fill_buf()?;

        if let Ok(content) = rpc::decode_message(buffer) {
            if content.method == "shutdown" {
                break;
            }
            match handle(content) {
                Ok(_) => log!("successfully handled message"),
                Err(e) => log!("error handling message: {:#?}", e),
            }

            let length = buffer.len();
            stdin.consume(length);
        }
    }
    log!("Lsp stopped...");
    Ok(())
}

fn handle(message: BaseMessage) -> anyhow::Result<()> {
    log!("Got message - method: {}", message.method);
    log!("Got message - id: {:?}", message.id);

    match message.method.as_str() {
        "initialize" => {
            let req: InitializeRequest = serde_json::from_value(message.params.unwrap())?;
            log!("{:#?}", req);
            let response = get_init_response();
            let response = Response {
                id: message.id.unwrap(),
                jsonrpc: "2.0".to_string(),
                result: response,
            };
            let encoded_message = encoding::rpc::encode_message(response)?;
            let mut stdout = std::io::stdout().lock();
            stdout.write_all(encoded_message.as_bytes())?;
            stdout.flush()?;
            log!("Wrote to stdout response: {}", encoded_message);
        }
        _ => log!("Found implementation for method '{}'", message.method),
    }
    
    Ok(())
}

#[derive(Serialize)]
struct Response<T> {
    id: usize,
    jsonrpc: String,
    result: T,
}
