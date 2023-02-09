use bson::oid::ObjectId;
use serde::{Deserialize, Serialize};
use serenity::all::ChannelId;

use super::ChannelVersion;

/// A document to be inserted as a new Channel document.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename = "channel")]
#[serde(rename_all = "camelCase")]
pub struct CreateChannel {
    /// The version of the document.
    pub version: ChannelVersion,
    /// The Discord snowflake for the channel.
    pub channel_id: ChannelId,
    /// The MongoDb OID of the campaign this channel belongs to.
    pub campaign_id: ObjectId,
}
