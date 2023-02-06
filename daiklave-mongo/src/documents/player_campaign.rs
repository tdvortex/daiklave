use std::collections::HashSet;

use serenity::all::ChannelId;

use crate::PlayerCharacters;

/// A subdocument representing one of the campaigns a player is a part of, 
/// from their perspective.
pub struct PlayerCampaign {
    /// The name of the campaign.
    pub name: String,
    /// Whether the player is the Storyteller of this campaign.
    pub is_storyteller: bool,
    /// Stubs for the characters that the player possesses for this campaign.
    pub characters: PlayerCharacters,
    /// The Id of the channel to which dice rolls are sent when invoked from
    /// the browser. (Slash commands will roll dice in the channel where they 
    /// are invoked.)
    pub dice_channel: ChannelId,
    /// All channels that the campaign claims ownership of (including the dice 
    /// channel)
    pub channels: HashSet<ChannelId>,
}