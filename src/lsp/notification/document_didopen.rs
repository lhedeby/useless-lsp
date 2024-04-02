use serde::{Deserialize, Serialize};

use crate::lsp::{
    diagnostics::{Diagnostic, DiagnosticSeverity},
    document::TextDocumentItem,
    position::Position,
    range::Range,
};

use super::traits::Notification;

#[derive(Serialize, Deserialize, Debug)]
pub struct DidOpenTextDocumentNotification {
    #[serde(rename = "textDocument")]
    pub text_document: TextDocumentItem,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PublishDiagnosticsParams {
    uri: String,
    diagnostics: Vec<Diagnostic>,
}

impl Notification<PublishDiagnosticsParams> for DidOpenTextDocumentNotification {
    fn handle(
        self,
        state: &mut crate::state::State,
    ) -> anyhow::Result<Option<PublishDiagnosticsParams>> {
        let uri = self.text_document.uri.clone();
        state.update(self.text_document);
        let mut diagnostics: Vec<Diagnostic> = vec![];

        if let Some(doc) = state.get_doc(&uri) {
            for (i, line) in doc.text.lines().enumerate() {
                if let Some(sad) = line.find("sad") {
                    diagnostics.push(Diagnostic {
                        range: Range {
                            start: Position {
                                line: i,
                                character: sad,
                            },
                            end: Position {
                                line: i,
                                character: sad + 3,
                            },
                        },
                        severity: Some(DiagnosticSeverity::Error),
                        code: Some("2305".to_string()),
                        source: Some("Uselesslsp".to_string()),
                        message: "Don't be sad :(".to_string(),
                    })
                }
            }
        }

        let params = PublishDiagnosticsParams { uri, diagnostics };
        Ok(Some(params))
    }
}
