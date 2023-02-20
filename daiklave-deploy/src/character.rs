use serenity::{
    all::{CommandOptionType, CommandType},
    builder::{CreateCommand, CreateCommandOption},
};

pub fn character() -> CreateCommand {
    let concept_delete = CreateCommandOption::new(
        CommandOptionType::SubCommand,
        "delete",
        "Delete your character concept",
    );

    let concept_help = CreateCommandOption::new(
        CommandOptionType::SubCommand,
        "help",
        "Help with character concept commands",
    );

    let concept_set = CreateCommandOption::new(
        CommandOptionType::SubCommand,
        "set",
        "Set your character concept",
    );

    let concept_show = CreateCommandOption::new(
        CommandOptionType::SubCommand,
        "show",
        "Show your character concept",
    );

    let concept = CreateCommandOption::new(
        CommandOptionType::SubCommandGroup,
        "concept",
        "Character concept commands",
    )
    .add_sub_option(concept_delete)
    .add_sub_option(concept_help)
    .add_sub_option(concept_set)
    .add_sub_option(concept_show);

    // TODO
    let create = CreateCommandOption::new(
        CommandOptionType::SubCommand,
        "create",
        "Create a new character",
    );

    // TODO
    let delete = CreateCommandOption::new(
        CommandOptionType::SubCommand,
        "delete",
        "Delete a character",
    )
    .add_sub_option(
        CreateCommandOption::new(
            CommandOptionType::String,
            "name",
            "The name of the character to delete",
        )
        .set_autocomplete(true)
        .required(true),
    );

    let help = CreateCommandOption::new(
        CommandOptionType::SubCommand,
        "help",
        "Help with character top-level commands",
    );

    // TODO
    let rename = CreateCommandOption::new(
        CommandOptionType::SubCommand,
        "rename",
        "Rename a character",
    )
    .add_sub_option(
        CreateCommandOption::new(
            CommandOptionType::String,
            "old_name",
            "The character's old name",
        )
        .set_autocomplete(true)
        .required(true),
    )
    .add_sub_option(
        CreateCommandOption::new(
            CommandOptionType::String,
            "new_name",
            "The character's new name",
        )
        .required(true),
    );

    // TODO
    let switch = CreateCommandOption::new(
        CommandOptionType::SubCommand,
        "switch",
        "Switch your active character in this campaign",
    )
    .add_sub_option(
        CreateCommandOption::new(
            CommandOptionType::String,
            "name",
            "The name of the character to activate",
        )
        .set_autocomplete(true)
        .required(true),
    );

    CreateCommand::new("character")
        .kind(CommandType::ChatInput)
        .description("Character top-level commands")
        .add_option(concept)
        .add_option(create)
        .add_option(delete)
        .add_option(help)
        .add_option(rename)
        .add_option(switch)
}
