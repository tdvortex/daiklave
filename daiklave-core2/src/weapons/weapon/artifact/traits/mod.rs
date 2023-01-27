mod memo;

pub use memo::ArtifactWeaponTraitsMemo;

use crate::{
    artifact::{ArtifactId, MagicMaterial},
    book_reference::BookReference,
    hearthstones::{hearthstone::Hearthstone, HearthstonePosition, SlottedHearthstone},
    weapons::weapon::base::BaseWeapon,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ArtifactWeaponTraits<'source> {
    pub(crate) book_reference: Option<BookReference>,
    pub(crate) merit_dots: u8,
    pub(crate) magic_material: MagicMaterial,
    pub(crate) base_weapon_name: &'source str,
    pub(crate) base_weapon: &'source BaseWeapon,
    pub(crate) lore: Option<&'source str>,
    pub(crate) powers: Option<&'source str>,
    pub(crate) hearthstone_slots: Vec<Option<SlottedHearthstone<'source>>>,
}

impl<'view, 'source> ArtifactWeaponTraits<'source> {
    pub fn as_memo(&self) -> ArtifactWeaponTraitsMemo {
        ArtifactWeaponTraitsMemo {
            book_reference: self.book_reference,
            merit_dots: self.merit_dots,
            base_weapon_name: self.base_weapon_name.to_owned(),
            base_weapon: self.base_weapon.clone(),
            lore: self.lore.map(|s| s.to_string()),
            powers: self.powers.map(|s| s.to_string()),
            hearthstone_slots: self
                .hearthstone_slots
                .iter()
                .map(|option| option.map(|hearthstone| hearthstone.as_memo()))
                .collect(),
            magic_material: self.magic_material,
        }
    }

    pub fn base_artifact_weapon(&self) -> &'source BaseWeapon {
        self.base_weapon
    }

    pub fn lore(&self) -> Option<&'source str> {
        self.lore
    }

    pub fn powers(&self) -> Option<&'source str> {
        self.powers
    }

    pub fn slotted_hearthstones(
        &self,
        name: &'source str,
    ) -> impl Iterator<Item = Hearthstone<'source>> + '_ {
        self.hearthstone_slots
            .iter()
            .filter_map(move |maybe_hearthstone| {
                maybe_hearthstone.as_ref().map(|slotted| {
                    Hearthstone(HearthstonePosition::Slotted(
                        ArtifactId::Weapon(name),
                        *slotted,
                    ))
                })
            })
    }
}
