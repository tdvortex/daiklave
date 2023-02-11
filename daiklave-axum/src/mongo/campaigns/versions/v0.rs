use std::collections::{HashSet};

use bson::oid::ObjectId;
use mongodb::bson;
use serde::{Serialize, Deserialize};
use serenity::all::{UserId, ChannelId};

use crate::mongo::campaigns::CampaignDocument;

/// Version 0 of the campaign document schema.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(into = "CampaignDocument")]
#[serde(from = "CampaignDocument")]
pub struct CampaignV0 {
    /// The MongoDB database ID for the campaign.
    pub _id: ObjectId,
    /// The human-readable name of the campaign.
    pub name: String,
    /// The Discord Snowflake representing the storyteller for the campaign.
    pub storyteller: UserId,
    /// All the Discord snowflakes for the players in the 
    /// campaign (including the storyteller).
    #[serde(default)]
    pub players: HashSet<UserId>,
    /// The Id of the channel to which dice rolls are sent when invoked from
    /// the browser. (Slash commands will roll dice in the channel where they 
    /// are invoked.)
    pub dice_channel: ChannelId,
    /// All channels that the campaign claims ownership of (including the dice 
    /// channel)
    #[serde(default)]
    pub channels: HashSet<ChannelId>,
}


