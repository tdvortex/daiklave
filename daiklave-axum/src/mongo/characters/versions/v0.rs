use daiklave_core::CharacterMemo as Character;
use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};
use serenity::all::UserId;

use crate::mongo::characters::CharacterDocument;

/// V0 of the Character document.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(into = "CharacterDocument")]
#[serde(from = "CharacterDocument")]
pub struct CharacterV0 {
    /// The MongoDb database Id for this character.
    pub _id: ObjectId,
    /// The Discord snowflake for this character's player.
    pub player: UserId,
    /// The database Id for the campaign this character belongs to.
    pub campaign_id: ObjectId,
    /// The current Character struct from daiklave-core.
    pub character: Character,
}
