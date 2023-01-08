use crate::{
    armor::armor_item::artifact::{ArtifactArmorId, ArtifactArmor},
    weapons::weapon::{artifact::ArtifactWeaponMemo, ArtifactWeaponId},
};

mod id;
mod magic_material;

pub use id::ArtifactId;
pub use magic_material::MagicMaterial;

/// A magica;, Essence-infused object.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Artifact {
    /// An artifact weapon.
    Weapon(ArtifactWeaponId, ArtifactWeaponMemo),
    /// An artifact armor item.
    Armor(ArtifactArmorId, ArtifactArmor),
}
