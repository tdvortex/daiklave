mod versions;
use mongodb::bson::oid::ObjectId;
use serenity::all::UserId;
pub use versions::{CharacterCurrent, CharacterV0, CharacterVersion};
use serde::{Serialize, Deserialize};

use daiklave_core::CharacterMemo as Character;

/// A versioned Character document.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename = "character")]
#[serde(rename_all = "camelCase")]
#[serde(tag = "version")]
pub enum CharacterDocument {
    /// Version 0
    V0(CharacterV0),
}

impl From<CharacterCurrent> for CharacterDocument {
    fn from(value: CharacterCurrent) -> Self {
        Self::V0(value)
    }
}

impl From<CharacterDocument> for CharacterCurrent {
    fn from(value: CharacterDocument) -> Self {
        match value {
            CharacterDocument::V0(value) => value,
        }
    }
}

/// A new Character document to be inserted. Includes the character version
/// but not the _id as it has not been generated yet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InsertCharacter {
    /// The MongoDB database ID for the character. Should always be the current Id.
    pub version: CharacterVersion,
    /// The Discord snowflake for this character's player.
    pub player: UserId,
    /// The database Id for the campaign this character belongs to.
    pub campaign_id: ObjectId,
    /// The current Character struct from daiklave-core.
    pub character: Character,
}