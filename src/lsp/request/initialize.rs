use crate::{lsp::request::traits::Request, state::State};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct InitializeRequest {
    #[serde(rename = "clientInfo")]
    client_info: Option<ClientInfo>,
}

impl Request<InitializeResult> for InitializeRequest {
    fn handle(&self, _: &mut State) -> anyhow::Result<InitializeResult> {
        Ok(InitializeResult {
            server_info: Some(ServerInfo {
                name: "uselesslsp".to_string(),
                version: Some("0.0.1".to_string()),
            }),
            capabilities: ServerCapabilities {
                hover_provider: true,
                definition_provider: true,
                text_document_sync: 1,
                code_action_provider: true,
            },
        })
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct ClientInfo {
    name: String,
    version: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct InitializeResult {
    capabilities: ServerCapabilities,
    #[serde(rename = "serverInfo")]
    server_info: Option<ServerInfo>,
}

#[derive(Serialize, Deserialize, Debug)]
struct ServerCapabilities {
    #[serde(rename = "hoverProvider")]
    hover_provider: bool,
    #[serde(rename = "definitionProvider")]
    definition_provider: bool,
    #[serde(rename = "codeActionProvider")]
    code_action_provider: bool,
    #[serde(rename = "textDocumentSync")]
    text_document_sync: usize,
}

#[derive(Serialize, Deserialize, Debug)]
struct ServerInfo {
    name: String,
    version: Option<String>,
}

pub fn get_init_result() -> InitializeResult {
    InitializeResult {
        server_info: Some(ServerInfo {
            name: "uselesslsp".to_string(),
            version: Some("0.0.1".to_string()),
        }),
        capabilities: ServerCapabilities {
            hover_provider: true,
            definition_provider: true,
            text_document_sync: 1,
            code_action_provider: true,
        },
    }
}
