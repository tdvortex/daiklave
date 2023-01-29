use crate::{
    armor::armor_item::artifact::AddArtifactArmor, weapons::weapon::artifact::ArtifactWeapon, CharacterMutation,
};

/// Builders for Wonders and Warstriders.
pub mod builder;
mod magic_material;
mod name;
mod sonance;

/// Artifacts which are not weapons, armor, or warstriders.
pub mod wonders;
mod attune;

pub use attune::AttuneArtifact;
pub use magic_material::MagicMaterial;
pub use name::{ArtifactName, ArtifactNameMutation};
pub use sonance::Sonance;

use self::{builder::wonder::WonderBuilder, wonders::AddWonder};

/// A magical, Essence-infused object to be added to a character.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AddArtifact {
    /// An artifact weapon.
    Weapon(ArtifactWeapon),
    /// An artifact armor item.
    Armor(AddArtifactArmor),
    /// A catch-all for other artifacts.
    Wonder(AddWonder),
}

impl AddArtifact {
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

impl From<AddArtifactArmor> for AddArtifact {
    fn from(add_artifact_armor: AddArtifactArmor) -> Self {
        Self::Armor(add_artifact_armor)
    }
}

impl From<AddArtifact> for CharacterMutation {
    fn from(add_artifact: AddArtifact) -> Self {
        Self::AddMerit(add_artifact.into())
    }
}