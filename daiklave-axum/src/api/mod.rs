/// Routes under the campaigns header. Most routes fall under here.
pub mod campaigns;
/// Routes related to managing characters in a campaign. 
pub mod characters;
mod auth;

/// The login route.
pub mod login;
mod why_error;

pub use auth::{decode_user_id_cookie, get_auth, validate_player, validate_storyteller};
pub use why_error::{internal_server_error, not_found, not_logged_in, not_storyteller, WhyError};