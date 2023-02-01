mod with_name;
mod with_heartstone_slots;
mod with_magic_material;
mod with_merit_dots;

pub use with_name::ArtifactWeaponBuilderWithName;
pub use with_heartstone_slots::ArtifactWeaponBuilderWithHearthstoneSlots;
pub use with_magic_material::ArtifactWeaponBuilderWithMagicMaterial;
pub use with_merit_dots::ArtifactWeaponBuilderWithMeritDots;

use crate::book_reference::BookReference;

use super::{AddBaseArtifactWeapon, ArtifactWeaponName, BaseArtifactWeapon};

/// A builder to construct a new artifact weapon.
pub struct ArtifactWeaponBuilder {
    pub(crate) base_weapon_name: String,
    pub(crate) base_weapon: BaseArtifactWeapon,
    pub(crate) lore: Option<String>,
    pub(crate) powers: Option<String>,
    pub(crate) book_reference: Option<BookReference>,
}

impl From<AddBaseArtifactWeapon> for ArtifactWeaponBuilder {
    fn from(add_base_weapon: AddBaseArtifactWeapon) -> Self {
        Self::base_weapon(add_base_weapon)
    }
}

impl ArtifactWeaponBuilder {
    /// Starts a new builder with this artifact base weapon.
    pub fn base_weapon(add_base_weapon: impl Into<AddBaseArtifactWeapon>) -> Self {
        let add_base_weapon: AddBaseArtifactWeapon = add_base_weapon.into();
        Self {
            base_weapon_name: add_base_weapon.name,
            base_weapon: add_base_weapon.weapon,
            lore: None,
            powers: None,
            book_reference: None
        }
    }


    /// Add flavor text to describe the weapon's forging, history, and prior
    /// wielders.
    pub fn lore(mut self, lore: String) -> Self {
        self.lore = Some(lore);
        self
    }

    /// Add passive or unique magical effects that are not Evocations, such as
    /// Beloved Adorei's emotional bond to her wielder.
    pub fn powers(mut self, powers: String) -> Self {
        self.powers = Some(powers);
        self
    }

    /// Add a book reference for the weapon. Note that this is a reference for
    /// the named instance of the artifact and not the base weapon.
    pub fn book_reference(mut self, book_reference: BookReference) -> Self {
        self.book_reference = Some(book_reference);
        self
    }

    /// Sets the name for this artifact weapon.
    pub fn name(
        self,
        name: impl Into<ArtifactWeaponName>,
    ) -> ArtifactWeaponBuilderWithName {
        ArtifactWeaponBuilderWithName {
            name: name.into(),
            base_weapon_name: self.base_weapon_name,
            base_weapon: self.base_weapon,
            lore: self.lore,
            powers: self.powers,
            book_reference: self.book_reference,
        }
    }
}
