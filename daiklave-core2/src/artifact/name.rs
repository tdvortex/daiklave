use serde::{Deserialize, Serialize};

/// The name of a magical creation (weapon, armor, warstrider, or wonder).
/// For use in adding, removing, or otherwise changing a character's state with
/// regards to an artifact.
#[derive(Debug, PartialEq, Eq, Clone, Hash, Serialize, Deserialize)]
pub enum ArtifactNameMutation {
    /// An artifact weapon's name.
    Weapon(String),
    /// An artifact armor item's name.
    Armor(String),
    /// A wonder's name.
    Wonder(String),
}

impl<'source> ArtifactNameMutation {
    pub(crate) fn as_ref(&'source self) -> ArtifactName<'source> {
        match self {
            ArtifactNameMutation::Weapon(name) => ArtifactName::Weapon(name.as_str()),
            ArtifactNameMutation::Armor(name) => ArtifactName::Armor(name.as_str()),
            ArtifactNameMutation::Wonder(name) => ArtifactName::Wonder(name.as_str()),
        }
    }
}

/// The name of a magical creation (weapon, armor, warstrider, or wonder).
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum ArtifactName<'source> {
    /// An artifact weapon's name.
    Weapon(&'source str),
    /// An artifact armor item's name.
    Armor(&'source str),
    /// A wonder's name.
    Wonder(&'source str),
}
