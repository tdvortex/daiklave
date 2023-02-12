use serenity::builder::CreateCommand;

pub fn help() -> CreateCommand {
    CreateCommand::new("help").description("See available Daiklave commands")
}