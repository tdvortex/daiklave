use crate::{
    armor::armor_item::artifact::{ArtifactArmor, ArtifactArmorId},
    weapons::weapon::{artifact::{ArtifactWeapon}},
};

/// Builders for Wonders and Warstriders.
pub mod builder;
mod id;
mod magic_material;
mod name;
mod sonance;

/// Artifacts which are not weapons, armor, or warstriders.
pub mod wonders;

pub use id::ArtifactId;
pub use magic_material::MagicMaterial;
pub use name::ArtifactName;
pub use sonance::Sonance;

use self::{
    builder::wonder::WonderBuilder,
    wonders::{Wonder, WonderId},
};

/// A magical, Essence-infused object.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Artifact {
    /// An artifact weapon.
    Weapon(ArtifactWeapon),
    /// An artifact armor item.
    Armor(ArtifactArmorId, ArtifactArmor),
    /// A catch-all for other artifacts.
    Wonder(WonderId, Wonder),
}

impl Artifact {
    /// Starts constructing a Wonder artifact.
    pub fn wonder_builder(name: &str) -> WonderBuilder {
        WonderBuilder {
            name: name.to_owned(),
            book_reference: None,
            lore: None,
            magic_material: None,
            hearthstone_slots: 0,
            attunement_cost: None,
        }
    }
}
