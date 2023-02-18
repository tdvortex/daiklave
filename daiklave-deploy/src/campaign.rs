use serenity::{
    all::{CommandOptionType, CommandType},
    builder::{CreateCommand, CreateCommandOption},
};

pub fn campaign() -> CreateCommand {
    let channels_help = CreateCommandOption::new(
        CommandOptionType::SubCommand,
        "help",
        "Help with campaign channels commands",
    );

    // TODO
    let channels_set = CreateCommandOption::new(
        CommandOptionType::SubCommand,
        "set",
        "Set the channels used in this campaign",
    );

    // TODO
    let channels_show = CreateCommandOption::new(
        CommandOptionType::SubCommand,
        "show",
        "Show the channels used by this campaign",
    );

    // TODO
    let channels = CreateCommandOption::new(
        CommandOptionType::SubCommandGroup,
        "channels",
        "Campaign channel commands",
    )
    .add_sub_option(channels_help)
    .add_sub_option(channels_set)
    .add_sub_option(channels_show);

    let create = CreateCommandOption::new(
        CommandOptionType::SubCommand,
        "create",
        "Create a new campaign",
    )
    .add_sub_option({
        let mut campaign_name = CreateCommandOption::new(
            CommandOptionType::String,
            "name",
            "The name of the new campaign",
        )
        .required(true);
        campaign_name.min_length(1);
        campaign_name
    });

    let help = CreateCommandOption::new(
        CommandOptionType::SubCommand,
        "help",
        "Help with campaign commands",
    );

    // TODO
    let join =
        CreateCommandOption::new(CommandOptionType::SubCommand, "join", "Join this campaign");

    // TODO
    let kick = CreateCommandOption::new(
        CommandOptionType::SubCommand,
        "kick",
        "Kick a player from the campaign (Storyteller only)",
    )
    .add_sub_option(
        CreateCommandOption::new(CommandOptionType::User, "player", "The player to kick")
            .required(true),
    );

    // TODO
    let leave = CreateCommandOption::new(
        CommandOptionType::SubCommand,
        "leave",
        "Leave this campaign",
    );

    // TODO
    let rename = CreateCommandOption::new(
        CommandOptionType::SubCommand,
        "rename",
        "Rename this campaign (Storyteller only)",
    );

    // TODO
    let storyteller_help = CreateCommandOption::new(
        CommandOptionType::SubCommand,
        "help",
        "Help with Storyteller commands",
    );

    // TODO
    let storyteller_set = CreateCommandOption::new(
        CommandOptionType::SubCommand,
        "set",
        "Reassign the Storyteller for this campaign (Storyteller only)",
    )
    .add_sub_option(
        CreateCommandOption::new(CommandOptionType::User, "player", "The new storyteller")
            .required(true),
    );

    // TODO
    let storyteller = CreateCommandOption::new(
        CommandOptionType::SubCommandGroup,
        "storyteller",
        "Commands related to the Storyteller",
    )
    .add_sub_option(storyteller_help)
    .add_sub_option(storyteller_set);

    CreateCommand::new("campaign")
        .description("Campaign commands")
        .kind(CommandType::ChatInput)
        .add_option(channels)
        .add_option(create)
        .add_option(help)
        .add_option(join)
        .add_option(kick)
        .add_option(leave)
        .add_option(rename)
        .add_option(storyteller)
}
