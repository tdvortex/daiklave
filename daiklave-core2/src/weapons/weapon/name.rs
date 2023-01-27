use serde::{Deserialize, Serialize};

/// The name of a weapon to be added, removed, equipped, or unequipped.
#[derive(Debug, PartialEq, Eq, Clone, Hash, Serialize, Deserialize)]
pub enum WeaponNameMutation {
    /// All characters have the Unarmed weapon for free, and it cannot
    /// be removed.
    Unarmed,
    /// A mundane weapon without artifact traits.
    Mundane(String),
    /// A unique magical weapon.
    Artifact(String),
}

impl<'source> WeaponNameMutation {
    pub(crate) fn as_ref(&'source self) -> WeaponName<'source> {
        match self {
            WeaponNameMutation::Unarmed => WeaponName::Unarmed,
            WeaponNameMutation::Mundane(name) => WeaponName::Mundane(name.as_str()),
            WeaponNameMutation::Artifact(name) => WeaponName::Artifact(name.as_str()),
        }
    }
}

/// The name of a weapon.
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum WeaponName<'source> {
    /// All characters have the Unarmed weapon for free, and it cannot
    /// be removed.
    Unarmed,
    /// A mundane weapon without artifact traits.
    Mundane(&'source str),
    /// A unique magical weapon.
    Artifact(&'source str),
}
