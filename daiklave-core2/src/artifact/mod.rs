use crate::{
    armor::armor_item::artifact::{ArtifactArmorId, ArtifactArmorMemo},
    weapons::weapon::{artifact::ArtifactWeaponMemo, ArtifactWeaponId},
};

mod id;
mod magic_material;

pub use id::ArtifactId;
pub use magic_material::MagicMaterial;

/// An owned copy of an Artifact.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Artifact {
    /// An artifact weapon.
    Weapon(ArtifactWeaponId, ArtifactWeaponMemo),
    /// An artifact armor item.
    Armor(ArtifactArmorId, ArtifactArmorMemo),
}
