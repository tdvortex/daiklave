use serenity::all::UserId;

/// A Document to find a user given their Discord snowflake.
pub struct FindUser {
    /// The discord ID of the user to find.
    pub discord_id: UserId,
}