use serde::{Deserialize, Serialize};

use crate::{
    artifact::MagicMaterial, book_reference::BookReference, hearthstones::SlottedHearthstoneMemo,
    weapons::weapon::base::BaseWeapon,
};

use super::NamedArtifactWeapon;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NamedArtifactWeaponMemo {
    pub(crate) name: String,
    pub(crate) book_reference: Option<BookReference>,
    pub(crate) merit_dots: u8,
    pub(crate) magic_material: MagicMaterial,
    pub(crate) base_weapon_name: String,
    pub(crate) base_weapon: BaseWeapon,
    pub(crate) lore: Option<String>,
    pub(crate) powers: Option<String>,
    pub(crate) hearthstone_slots: Vec<Option<SlottedHearthstoneMemo>>,
}

impl<'source> NamedArtifactWeaponMemo {
    pub fn as_ref(&'source self) -> NamedArtifactWeapon<'source> {
        NamedArtifactWeapon {
            name: self.name.as_str(),
            book_reference: self.book_reference,
            merit_dots: self.merit_dots,
            base_weapon_name: self.base_weapon_name.as_str(),
            base_weapon: &self.base_weapon,
            lore: self.lore.as_deref(),
            powers: self.powers.as_deref(),
            hearthstone_slots: self
                .hearthstone_slots
                .iter()
                .map(|option| option.as_ref().map(|memo| memo.as_ref()))
                .collect(),
            magic_material: self.magic_material,
        }
    }
}
