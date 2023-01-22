mod with_base_weapon;
mod with_heartstone_slots;
mod with_magic_material;
mod with_merit_dots;

pub use with_base_weapon::ArtifactWeaponBuilderWithBaseWeapon;
pub use with_heartstone_slots::ArtifactWeaponBuilderWithHearthstoneSlots;
pub use with_magic_material::ArtifactWeaponBuilderWithMagicMaterial;
pub use with_merit_dots::ArtifactWeaponBuilderWithMeritDots;

use crate::{book_reference::BookReference, weapons::weapon::base::BaseWeaponId};

use super::base::BaseArtifactWeapon;

/// A builder to construct a new artifact weapon. Enforces that required fields
/// are specified in order: name, base artifact, magic material, merit dots,
/// and finally hearthstone slots. Optional fields (lore, powers, and book
/// reference) may be specified at any time prior to the final build().
pub struct ArtifactWeaponBuilder {
    pub(crate) name: String,
    pub(crate) lore: Option<String>,
    pub(crate) powers: Option<String>,
    pub(crate) book_reference: Option<BookReference>,
}

impl ArtifactWeaponBuilder {
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

    /// Specifies the base artifact weapon for the artifact.
    pub fn base_artifact(
        self,
        base_weapon_id: BaseWeaponId,
        base_weapon: BaseArtifactWeapon,
    ) -> ArtifactWeaponBuilderWithBaseWeapon {
        ArtifactWeaponBuilderWithBaseWeapon {
            name: self.name,
            base_weapon_id,
            base_weapon,
            lore: self.lore,
            powers: self.powers,
            book_reference: self.book_reference,
        }
    }
}
