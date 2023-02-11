use serenity::{builder::CreateCommand, all::CommandType};

pub fn version() -> CreateCommand {
    CreateCommand::new("version")
        .description("Returns the Daiklave version being used")
        .kind(CommandType::ChatInput)
}