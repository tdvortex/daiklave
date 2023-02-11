use serenity::{builder::{CreateCommand, CreateCommandOption}, all::{CommandOptionType, CommandType}};

pub fn campaign() -> CreateCommand {
    let campaign_create = CreateCommandOption::new(
        CommandOptionType::SubCommand,
        "create",
        "Create a new campaign",
    )
    .add_sub_option({
        let mut campaign_name = CreateCommandOption::new(
            CommandOptionType::String,
            "name",
            "The name of the new campaign",
        ).required(true);
        campaign_name.min_length(1);
        campaign_name
    });    
    
    CreateCommand::new("campaign")
        .description("Campaign management commands")
        .kind(CommandType::ChatInput)
        .add_option(campaign_create)
}