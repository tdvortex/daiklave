use bson::oid::ObjectId;
use daiklave_core::CharacterMemo as Character;
use serde::{Serialize, Deserialize};
use serenity::all::{UserId, ChannelId};

/// A document to insert a new character into MongoDb.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename = "character")]
#[serde(rename_all = "camelCase")]
pub struct NewCharacter {
    /// The version of the Character document to be inserted.
    pub version: String,
    /// The Discord snowflake for this character's player.
    pub player: UserId,
    /// The database Id for the campaign this character belongs to.
    pub campaign_id: ObjectId,
    /// The name of the campaign this character belongs to.
    pub campaign_name: String,
    /// The dice channel for the campaign this character belongs to.
    pub dice_channel: ChannelId,
    /// The current Character struct from daiklave-core.
    pub character: Character,
}