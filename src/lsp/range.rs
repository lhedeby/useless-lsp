use crate::lsp::position::Position;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Range {
    pub start: Position,
    pub end: Position,
}
