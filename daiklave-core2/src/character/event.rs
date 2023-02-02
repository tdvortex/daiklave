use crate::{Character, CharacterEventSource, CharacterMutation, CharacterMutationError};

pub trait CharacterEvent<'source> {
    fn apply_event(
        self,
        event_source: &'source mut CharacterEventSource,
    ) -> Result<Character<'source>, CharacterMutationError>;
}

impl<'source, T> CharacterEvent<'source> for T
where
    T: Into<CharacterMutation>,
{
    fn apply_event(
        self,
        event_source: &'source mut CharacterEventSource,
    ) -> Result<Character<'source>, CharacterMutationError> {
        event_source.apply_mutation(self)
    }
}
