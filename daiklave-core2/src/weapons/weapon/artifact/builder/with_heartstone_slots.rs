use crate::{
    artifact::MagicMaterial,
    book_reference::BookReference,
    weapons::weapon::{
        artifact::{
            base::BaseArtifactWeapon, memo::ArtifactWeaponHandedness,
            named::NamedArtifactWeaponMemo, ArtifactWeapon, NaturalArtifactWeapon,
            OneHandedArtifactWeapon, TwoHandedArtifactWeapon, WornArtifactWeapon,
        },
        handedness::WeaponHandedness,
        BaseWeaponId,
    },
};

/// An artifact builder after having its hearthstone slots specified.
/// The final step is .build() to finish the builder.
pub struct ArtifactWeaponBuilderWithHearthstoneSlots {
    pub(crate) name: String,
    pub(crate) lore: Option<String>,
    pub(crate) powers: Option<String>,
    pub(crate) book_reference: Option<BookReference>,
    pub(crate) base_weapon_id: BaseWeaponId,
    pub(crate) base_weapon: BaseArtifactWeapon,
    pub(crate) magic_material: MagicMaterial,
    pub(crate) merit_dots: u8,
    pub(crate) hearthstone_slots: usize,
}

impl ArtifactWeaponBuilderWithHearthstoneSlots {
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

    /// Completes the builder, returning an Artifact Weapon.
    pub fn build(self) -> ArtifactWeapon {
        let (handedness, base_weapon) = (self.base_weapon.handedness, self.base_weapon.base_weapon);

        let empty_hearthstone_slots = (0..self.hearthstone_slots).fold(Vec::new(), |mut v, _| {
            v.push(None);
            v
        });

        let named_artifact_weapon = NamedArtifactWeaponMemo {
            name: self.name,
            book_reference: self.book_reference,
            merit_dots: self.merit_dots,
            base_weapon_id: self.base_weapon_id,
            base_weapon,
            lore: self.lore,
            powers: self.powers,
            hearthstone_slots: empty_hearthstone_slots,
            magic_material: self.magic_material,
        };

        match handedness {
            WeaponHandedness::Natural => ArtifactWeapon(ArtifactWeaponHandedness::Natural(
                NaturalArtifactWeapon(named_artifact_weapon),
            )),
            WeaponHandedness::Worn => ArtifactWeapon(ArtifactWeaponHandedness::Worn(
                WornArtifactWeapon(named_artifact_weapon),
                false,
            )),
            WeaponHandedness::OneHanded => ArtifactWeapon(ArtifactWeaponHandedness::OneHanded(
                OneHandedArtifactWeapon(named_artifact_weapon),
                None,
            )),
            WeaponHandedness::TwoHanded => ArtifactWeapon(ArtifactWeaponHandedness::TwoHanded(
                TwoHandedArtifactWeapon(named_artifact_weapon),
                false,
            )),
        }
    }
}
