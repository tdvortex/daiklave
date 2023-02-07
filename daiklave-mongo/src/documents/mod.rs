/// The Campaign document interface.
pub mod campaign;

/// The Character document interface.
pub mod character;

/// Errors related to MongoDb documents and collections.
pub mod error;

mod player_campaign;
pub use player_campaign::PlayerCampaign;


mod player_characters;
pub use player_characters::PlayerCharacters;

/// The User document interface.
pub mod user;