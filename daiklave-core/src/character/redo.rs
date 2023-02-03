use crate::{Character, CharacterEventSource, CharacterMutationError};

use super::CharacterEvent;

pub struct Redo;

impl<'source> CharacterEvent<'source> for Redo {
    fn apply_event(
        self,
        event_source: &'source mut CharacterEventSource,
    ) -> Result<Character<'source>, CharacterMutationError> {
        event_source.redo()
    }
}
