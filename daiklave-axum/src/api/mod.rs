/// Routes under the campaigns header. Most routes fall under here.
pub mod campaigns;
/// Routes related to managing characters in a campaign. 
pub mod characters;
mod decode_cookie;
/// The login route.
pub mod login;
pub use decode_cookie::decode_user_id_cookie;