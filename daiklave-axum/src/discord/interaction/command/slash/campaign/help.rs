use axum::{response::{Response, IntoResponse}, Json};
use serenity::builder::{CreateInteractionResponse, CreateInteractionResponseMessage, CreateEmbed};

pub fn campaign_help() -> Response {
    Json(CreateInteractionResponse::Message(
        CreateInteractionResponseMessage::new()
        .embed(
            CreateEmbed::new()
            .title("/campaign")
            .field("channels", "Subcommands related to this campaign's channels. Use **/campaign channels help** for more.", false)
            .field("create", "Start a new campaign, becoming the storyteller (admin) of that campaign.", false)
            .field("help", "Shows the available campaign subcommands", false)
            .field("join", "Join the campaign in this channel", false)
            .field("kick", "Kick a player from this campaign. Only the storyteller can use this command. The storyteller cannot be kicked.", false)
            .field("leave", "Leave the campaign in this channel. The storyteller cannot leave a campaign until they designate a new storyteller with **/campaign storyteller** set", false)
            .field("rename", "Renames this campaign", false)
            .field("set", "Reassigns the storyteller for the campaign. Only the storyteller can use this command.", false)
            .field("storyteller", "Subcommands related to this campaign's storyteller. Use **/campaign story help** for more.", false)
        )
    )).into_response()
}
