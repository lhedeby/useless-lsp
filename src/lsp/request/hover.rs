use crate::lsp::range::Range;
use crate::logger;
use crate::lsp::position::Position;
use crate::{log, lsp::request::traits::Request};

use anyhow::bail;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct HoverRequest {
    #[serde(rename = "textDocument")]
    pub text_document: TextDocumentIdentifier,
    pub position: Position,
}

impl Request<HoverResult> for HoverRequest {
    fn handle(&self, state: &mut crate::state::State) -> anyhow::Result<HoverResult> {
        if let Some(doc) = state.get_doc(&self.text_document.uri) {
            if let Some(line) = doc.text.lines().nth(self.position.line) {
                let chars: Vec<char> = line.chars().collect();
                let mut word_start = self.position.character;
                let mut word_end = self.position.character;
                if chars.len() > 0 && !chars[word_start].is_whitespace() {
                    while word_start > 0 && chars[word_start - 1] != ' ' {
                        word_start -= 1;
                    }
                    while word_end < line.len() && chars[word_end] != ' ' {
                        word_end += 1;
                    }
                }
                let word = String::from_iter(&chars[word_start..word_end]);
                log!(
                    "line: {}, start: {}, end: {}, word: {}",
                    line,
                    word_start,
                    word_end,
                    word
                );

                return Ok(HoverResult {
                    contents: MarkupContent {
                        kind: MarkupKind::Markdown,
                        value: format!("The word you are hovering\n\n---\n\n ```{}```", word),
                    },
                    range: None,
                })
            }
        }
        bail!("Generating initialize response");
    }
}


#[derive(Serialize, Deserialize, Debug)]
pub struct TextDocumentIdentifier {
    pub uri: DocumentUri,
}

type DocumentUri = String;

#[derive(Serialize, Deserialize, Debug)]
pub struct HoverResult {
    pub contents: MarkupContent,
    pub range: Option<Range>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MarkupContent {
    pub kind: MarkupKind,
    pub value: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum MarkupKind {
    #[serde(rename = "plaintext")]
    Plaintext,
    #[serde(rename = "markdown")]
    Markdown,
}

