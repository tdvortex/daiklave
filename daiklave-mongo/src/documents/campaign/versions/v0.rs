use std::collections::{HashMap, HashSet};

use bson::oid::ObjectId;
use serenity::all::{UserId, ChannelId};

use crate::PlayerCharacters;

/// Version 0 of the campaign document schema.
pub struct CampaignV0 {
    /// The MongoDB database ID for the campaign.
    pub _id: ObjectId,
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
