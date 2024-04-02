use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct TextDocumentItem {
    pub uri: String,
    #[serde(rename = "languageId")]
    language_id: String,
    version: usize,
    pub text: String,
}
