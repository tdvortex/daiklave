use crate::{
    armor::armor_item::artifact::{ArtifactArmor, ArtifactArmorId},
    weapons::weapon::{artifact::ArtifactWeapon, ArtifactWeaponId},
};

mod id;
mod magic_material;

/// Artifacts which are not weapons, armor, or warstriders.
pub mod wonders;

pub use id::ArtifactId;
pub use magic_material::MagicMaterial;

use self::wonders::{WonderId, Wonder};

/// A magical, Essence-infused object.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Artifact {
    /// An artifact weapon.
    Weapon(ArtifactWeaponId, ArtifactWeapon),
    /// An artifact armor item.
    Armor(ArtifactArmorId, ArtifactArmor),
    /// A catch-all for other artifacts.
    Wonder(WonderId, Wonder),
}
