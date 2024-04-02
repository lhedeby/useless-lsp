use serde::Serialize;

use crate::state::State;

pub trait Request<T: Serialize> {
    fn handle(&self, state: &mut State) -> anyhow::Result<T>;
}
