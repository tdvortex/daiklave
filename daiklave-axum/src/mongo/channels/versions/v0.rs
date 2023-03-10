use bson::oid::ObjectId;
use mongodb::bson;
use serde::{Deserialize, Serialize};
use serenity::all::ChannelId;

use crate::mongo::channels::ChannelDocument;

/// Version zero of the Channel document.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(into = "ChannelDocument")]
#[serde(from = "ChannelDocument")]
pub struct ChannelV0 {
    /// The MongoDb database id.
    pub _id: ObjectId,
    /// The Discord snowflake for the channel.
    pub channel_id: ChannelId,
    /// The MongoDb OID of the campaign this channel belongs to.
    pub campaign_id: ObjectId,
}
