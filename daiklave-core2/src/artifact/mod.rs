use crate::weapons::weapon::artifact::ArtifactWeapon;

mod id;
mod magic_material;
mod memo;

pub use id::ArtifactId;
pub use magic_material::MagicMaterial;
pub use memo::ArtifactMemo;

/// A magical item owned by a character.
pub enum Artifact<'source> {
    /// An artifact weapon like a daiklave or direlash.
    Weapon(ArtifactWeapon<'source>),
}

impl<'source> Artifact<'source> {
    /// Creates an owned copy of the artifact.
    pub fn as_memo(&self) -> ArtifactMemo {
        match self {
            Artifact::Weapon(view) => ArtifactMemo::Weapon(view.as_memo()),
        }
    }
}
