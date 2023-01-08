use crate::{weapons::weapon::{artifact::ArtifactWeapon, ArtifactWeaponId}, armor::armor_item::artifact::{ArtifactArmorId, ArtifactArmor}};

mod id;
mod magic_material;
mod memo;

pub use id::ArtifactId;
pub use magic_material::MagicMaterial;
pub use memo::ArtifactMemo;

/// A magical item owned by a character.
pub(crate) enum Artifact<'source> {
    /// An artifact weapon like a daiklave or direlash.
    Weapon(ArtifactWeaponId, ArtifactWeapon<'source>),
    /// A piece of artifact armor.
    Armor(ArtifactArmorId, ArtifactArmor<'source>),
}

impl<'source> Artifact<'source> {
    /// Creates an owned copy of the artifact.
    pub fn as_memo(&self) -> ArtifactMemo {
        match self {
            Artifact::Weapon(id, view) => ArtifactMemo::Weapon(*id, view.as_memo()),
            Artifact::Armor(id, view) => ArtifactMemo::Armor(*id, view.as_memo())
        }
    }
}
