use std::collections::{HashMap, HashSet};

use serde::{Serialize, Deserialize};
use serenity::all::{UserId, ChannelId};

use crate::PlayerCharacters;

/// A Campaign document to be inserted.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename = "campaign")]
#[serde(rename_all = "camelCase")]
pub struct NewCampaign {
    /// The version of the Campaign document to be inserted.
    pub version: String,
    /// The human-readable name of the campaign.
    pub name: String,
    /// The Discord Snowflake representing the storyteller for the campaign.
    pub storyteller: UserId,
    /// Keys are the Discord snowflakes for all players in the 
    /// campaign (including the storyteller). Values are all of the characters
    /// that player possesses in this campaign.
    pub players: HashMap<UserId, PlayerCharacters>,
    /// The Id of the channel to which dice rolls are sent when invoked from
    /// the browser. (Slash commands will roll dice in the channel where they 
    /// are invoked.)
    pub dice_channel: ChannelId,
    /// All channels that the campaign claims ownership of (including the dice 
    /// channel)
    pub channels: HashSet<ChannelId>,
}