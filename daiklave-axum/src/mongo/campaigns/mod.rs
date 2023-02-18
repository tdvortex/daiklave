use std::collections::HashSet;

use serde::{Deserialize, Serialize};
use serenity::all::{ChannelId, UserId};

use self::versions::CampaignV0;

pub use versions::{CampaignCurrent, CampaignVersion};

mod versions;

/// A versioned Campaign document.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename = "campaign")]
#[serde(rename_all = "camelCase")]
#[serde(tag = "version")]
pub enum CampaignDocument {
    /// Version zero.
    V0(CampaignV0),
}

impl From<CampaignCurrent> for CampaignDocument {
    fn from(value: CampaignCurrent) -> Self {
        Self::V0(value.into())
    }
}

impl From<CampaignDocument> for CampaignCurrent {
    fn from(value: CampaignDocument) -> Self {
        match value {
            CampaignDocument::V0(value) => value,
        }
    }
}

/// Current format to insert a new campaign document schema. Identical to
/// [CampaignCurrent] but without the _id field.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InsertCampaign {
    /// The MongoDB database ID for the campaign. Should always be V0.
    pub version: CampaignVersion,
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
