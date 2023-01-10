use self::position::HearthstonePosition;

mod details;
mod origin;
mod position;
mod slotted;
mod stability;
mod template;
mod unslotted;

/// A Hearthstone owned by a character.
pub struct Hearthstone<'source>(pub(crate) HearthstonePosition<'source>);