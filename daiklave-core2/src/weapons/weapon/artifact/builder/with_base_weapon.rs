use crate::{
    artifact::MagicMaterial,
    book_reference::BookReference,
    weapons::weapon::{artifact::base::BaseArtifactWeapon, BaseWeaponId},
};

use super::with_magic_material::ArtifactWeaponBuilderWithMagicMaterial;

/// An artifact builder after the base weapon has been specified.
/// The next stage is .material().
pub struct ArtifactWeaponBuilderWithBaseWeapon {
    pub(crate) name: String,
    pub(crate) lore: Option<String>,
    pub(crate) powers: Option<String>,
    pub(crate) book_reference: Option<BookReference>,
    pub(crate) base_weapon_id: BaseWeaponId,
    pub(crate) base_weapon: BaseArtifactWeapon,
}

impl ArtifactWeaponBuilderWithBaseWeapon {
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

    /// Specifies the magic material from which the weapon is constructed. If
    /// a weapon is built with more than one, only the primary material is
    /// recorded and the accents can be listed under Lore.
    pub fn material(self, magic_material: MagicMaterial) -> ArtifactWeaponBuilderWithMagicMaterial {
        ArtifactWeaponBuilderWithMagicMaterial {
            name: self.name,
            base_weapon_id: self.base_weapon_id,
            base_weapon: self.base_weapon,
            magic_material,
            lore: self.lore,
            powers: self.powers,
            book_reference: self.book_reference,
        }
    }
}
