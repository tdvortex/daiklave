mod memo;

pub use memo::NamedArtifactWeaponMemo;

use crate::{
    artifact::MagicMaterial,
    book_reference::BookReference,
    hearthstone::OwnedHearthstone,
    weapons::weapon::{base::BaseWeapon, BaseWeaponId},
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NamedArtifactWeapon<'source> {
    pub(crate) name: &'source str,
    pub(crate) book_reference: Option<BookReference>,
    pub(crate) merit_dots: u8,
    pub(crate) magic_material: MagicMaterial,
    pub(crate) base_weapon_id: BaseWeaponId,
    pub(crate) base_weapon: &'source BaseWeapon,
    pub(crate) lore: Option<&'source str>,
    pub(crate) powers: Option<&'source str>,
    pub(crate) hearthstone_slots: Vec<Option<OwnedHearthstone<'source>>>,
}

impl<'view, 'source> NamedArtifactWeapon<'source> {
    pub fn as_memo(&self) -> NamedArtifactWeaponMemo {
        NamedArtifactWeaponMemo {
            name: self.name.to_string(),
            book_reference: self.book_reference,
            merit_dots: self.merit_dots,
            base_weapon_id: self.base_weapon_id,
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

    pub fn name(&self) -> &'source str {
        self.name
    }

    pub fn book_reference(&self) -> Option<BookReference> {
        self.book_reference
    }

    pub fn base_artifact_weapon_id(&self) -> BaseWeaponId {
        self.base_weapon_id
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

    pub fn hearthstone_slots(&self) -> usize {
        self.hearthstone_slots.len()
    }

    pub fn slotted_hearthstones(
        &'view self,
    ) -> impl Iterator<Item = &'view OwnedHearthstone<'source>> + '_ {
        self.hearthstone_slots
            .iter()
            .filter_map(|maybe_hearthstone| maybe_hearthstone.as_ref())
    }
}
