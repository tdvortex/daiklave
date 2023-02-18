use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

pub use character_stub::CharacterStub;

mod character_stub;

/// A subdocument representing all of the characters a player possesses for a
/// campaign.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct PlayerCharacters {
    /// The Id of the player's active character. Slash commands in Discord will
    /// use this character.
    pub active_character: Option<ObjectId>,
    /// The Ids and names of the player's characters in this campaign (active
    /// and inactive).
    pub character: Vec<CharacterStub>,
}
