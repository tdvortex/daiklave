use serde::{Serialize, Deserialize};

use crate::{book_reference::BookReference, weapons::{BaseWeaponId, hearthstone::{OwnedHearthstone, OwnedHearthstoneMemo}, base::BaseWeapon}};

use super::base::{BaseArtifactWeapon, BaseArtifactWeaponMemo};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(in crate::weapons::artifact) struct NamedArtifactWeapon<'source> {
    name: &'source str,
    book_reference: Option<BookReference>,
    merit_dots: u8,
    base_weapon_id: BaseWeaponId,
    base_weapon: BaseArtifactWeapon<'source>,
    lore: Option<&'source str>,
    powers: Option<&'source str>,
    hearthstone_slots: Vec<Option<OwnedHearthstone<'source>>>,
}

impl<'view, 'source> NamedArtifactWeapon<'source> {
    pub fn as_memo(&self) -> NamedArtifactWeaponMemo {
        NamedArtifactWeaponMemo {
            name: self.name.to_string(),
            book_reference: self.book_reference,
            merit_dots: self.merit_dots,
            base_weapon_id: self.base_weapon_id,
            base_weapon: self.base_weapon.as_memo(),
            lore: self.lore.map(|s| s.to_string()),
            powers: self.powers.map(|s| s.to_string()),
            hearthstone_slots: self.hearthstone_slots.iter().map(|option| option.map(|hearthstone| hearthstone.as_memo())).collect(),
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

    pub fn base_artifact_weapon(&self) -> BaseWeapon<'source> {
        *self.base_weapon
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

    pub fn slotted_heathstones(&'view self) -> impl Iterator<Item = &'view OwnedHearthstone<'source>> + '_ {
        self.hearthstone_slots.iter().filter_map(|maybe_hearthstone| maybe_hearthstone.as_ref())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(in crate::weapons::artifact) struct NamedArtifactWeaponMemo {
    name: String,
    book_reference: Option<BookReference>,
    merit_dots: u8,
    base_weapon_id: BaseWeaponId,
    base_weapon: BaseArtifactWeaponMemo,
    lore: Option<String>,
    powers: Option<String>,
    hearthstone_slots: Vec<Option<OwnedHearthstoneMemo>>,
}

impl<'source> NamedArtifactWeaponMemo {
    pub fn as_ref(&'source self) -> NamedArtifactWeapon<'source> {
        NamedArtifactWeapon {
            name: self.name.as_str(),
            book_reference: self.book_reference,
            merit_dots: self.merit_dots,
            base_weapon_id: self.base_weapon_id,
            base_weapon: self.base_weapon.as_ref(),
            lore: self.lore.as_deref(),
            powers: self.powers.as_deref(),
            hearthstone_slots: self.hearthstone_slots.iter().map(|option| option.as_ref().map(|memo| memo.as_ref())).collect(),
        }
    }
}