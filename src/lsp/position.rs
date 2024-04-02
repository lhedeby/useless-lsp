use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Position {
    pub line: usize,
    pub character: usize,
}
