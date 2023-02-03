use serde::{Deserialize, Serialize};

/// A weapon usable by a Martial Arts style.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum MartialArtsStyleWeapon {
    /// The style can be used unarmed.
    Unarmed,
    /// The style is able to use a weapon with this name. This can be either a
    /// mundane weapon (like "sword") or a base artiface weapon (like
    /// "daiklave"), but not a named artifact weapon (like "Spring Razor".)
    BaseWeapon(String),
}
