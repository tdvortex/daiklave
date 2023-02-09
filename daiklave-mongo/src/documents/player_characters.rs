use bson::oid::ObjectId;
use serde::{Serialize, Deserialize};

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

/// A subdocument of just the OID and name of a character. For use in dropdown
/// selects where deserializing every character is not required.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CharacterStub {
    /// The Id of the character.
    pub character_id: ObjectId,
    /// The name of the character.
    pub name: String,
}

