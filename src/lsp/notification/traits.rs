use serde::Serialize;

use crate::state::State;

pub trait Notification<T: Serialize> {
    fn handle(self, state: &mut State) -> anyhow::Result<Option<T>>;
}
