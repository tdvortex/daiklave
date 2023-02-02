use crate::{Character, CharacterEventSource, CharacterMutationError};

use super::CharacterEvent;

pub struct Undo;

impl<'source> CharacterEvent<'source> for Undo {
    fn apply_event(
        self,
        event_source: &'source mut CharacterEventSource,
    ) -> Result<Character<'source>, CharacterMutationError> {
        event_source.undo()
    }
}
