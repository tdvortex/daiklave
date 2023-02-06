use bson::oid::ObjectId;
use daiklave_core::CharacterMemo as Character;
use serde::{Serialize, Deserialize};
use serenity::all::{UserId, ChannelId};

/// V0 of the Character document.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename = "character")]
#[serde(rename_all = "camelCase")]
pub struct CharacterV0 {
    /// The MongoDb database Id for this character.
    pub _id: ObjectId,
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