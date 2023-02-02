use crate::{Character, CharacterEventSource, CharacterMutation, CharacterMutationError};

/// A trait for something which can modify a [CharacterEventSource].
pub trait CharacterEvent<'source> {
    /// Resolves the event into the event source or errors.
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
