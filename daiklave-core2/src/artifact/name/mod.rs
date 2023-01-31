
mod mutation;

pub use mutation::ArtifactNameMutation;

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

impl<'source> From<&'source ArtifactNameMutation> for ArtifactName<'source> {
    fn from(name: &'source ArtifactNameMutation) -> Self {
        match name {
            ArtifactNameMutation::Weapon(artifact_weapon_name) => Self::Weapon(artifact_weapon_name.as_str()),
            ArtifactNameMutation::Armor(artifact_armor_name) => Self::Armor(artifact_armor_name.as_str()),
            ArtifactNameMutation::Wonder(wonder_name) => Self::Wonder(wonder_name.as_str()),
        }
    }
}