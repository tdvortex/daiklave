use std::collections::HashMap;

use crate::CharacterMutation;

use super::WoundPenalty;

/// A mutation to set the character's health track to have the specified 
/// boxes.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SetHealthTrack(pub HashMap<WoundPenalty, u8>);

impl From<SetHealthTrack> for CharacterMutation {
    fn from(set_health_track: SetHealthTrack) -> Self {
        CharacterMutation::SetHealthTrack(set_health_track)
    }
}