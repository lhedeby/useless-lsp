use std::collections::HashMap;

use crate::lsp::document::TextDocumentItem;

pub struct State {
    pub documents: HashMap<String, TextDocumentItem>,
}

impl State {
    pub fn new() -> State {
        State {
            documents: HashMap::new()
        }
    }

    pub fn update(&mut self, document: TextDocumentItem) {
        self.documents.insert(document.uri.to_string(), document);
    }

    pub fn get_doc(&self, s: &str) -> Option<&TextDocumentItem> {
        self.documents.get(s)
    }
}
