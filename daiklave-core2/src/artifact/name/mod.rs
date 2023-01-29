
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
            ArtifactNameMutation::Weapon(artifact_weapon_name) => ArtifactName::Weapon(&***artifact_weapon_name),
            ArtifactNameMutation::Armor(_) => todo!(),
            ArtifactNameMutation::Wonder(_) => todo!(),
        }
    }
}