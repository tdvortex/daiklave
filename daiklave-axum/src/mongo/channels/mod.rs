mod versions;
use mongodb::bson::oid::ObjectId;
use serde::{Serialize, Deserialize};
use serenity::all::ChannelId;
pub use versions::{ChannelV0, ChannelVersion, ChannelCurrent};

/// A versioned Channel document.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename = "channel")]
#[serde(rename_all = "camelCase")]
#[serde(tag = "version")]
pub enum ChannelDocument {
    /// Version zero
    V0(ChannelV0),
}

impl From<ChannelCurrent> for ChannelDocument {
    fn from(value: ChannelCurrent) -> Self {
        Self::V0(value)
    }
}

impl From<ChannelDocument> for ChannelCurrent {
    fn from(value: ChannelDocument) -> Self {
        match value {
            ChannelDocument::V0(value) => value,
        }
    }
}

/// A new channel document, specifying the version but not the _id.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct InsertChannel {
    /// The version of the channel. 
    pub version: ChannelVersion,
    /// The Discord snowflake for the channel.
    pub channel_id: ChannelId,
    /// The MongoDb OID of the campaign this channel belongs to.
    pub campaign_id: ObjectId,
}