use crate::{
    artifact::MagicMaterial, book_reference::BookReference,
    weapons::weapon::artifact::base::BaseArtifactWeapon,
};

use super::with_merit_dots::ArtifactWeaponBuilderWithMeritDots;

/// An artifact weapon after specifying its Magic Material. The next
/// step is .merit_dots().
pub struct ArtifactWeaponBuilderWithMagicMaterial {
    pub(crate) name: String,
    pub(crate) lore: Option<String>,
    pub(crate) powers: Option<String>,
    pub(crate) book_reference: Option<BookReference>,
    pub(crate) base_weapon_name: String,
    pub(crate) base_weapon: BaseArtifactWeapon,
    pub(crate) magic_material: MagicMaterial,
}

impl ArtifactWeaponBuilderWithMagicMaterial {
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

    /// Specifies the dot rating of the artifact. Officially, all artifact
    /// weapons should be rated 3+, but this is not enforced. Dot ratings
    /// of 6+ are treatedas N/A artifacts.
    pub fn merit_dots(self, dots: u8) -> ArtifactWeaponBuilderWithMeritDots {
        ArtifactWeaponBuilderWithMeritDots {
            name: self.name,
            base_weapon_name: self.base_weapon_name,
            base_weapon: self.base_weapon,
            magic_material: self.magic_material,
            merit_dots: dots.min(6),
            lore: self.lore,
            powers: self.powers,
            book_reference: self.book_reference,
        }
    }
}
