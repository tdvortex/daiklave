use crate::{book_reference::BookReference, weapons::{BaseWeaponId, hearthstone::OwnedHearthstone, base::BaseWeapon}};

use super::base::BaseArtifactWeapon;

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
        self.lore.as_deref()
    }

    pub fn powers(&self) -> Option<&'source str> {
        self.powers.as_deref()
    }

    pub fn hearthstone_slots(&self) -> usize {
        self.hearthstone_slots.len()
    }

    pub fn slotted_heathstones(&'view self) -> impl Iterator<Item = &'view OwnedHearthstone<'source>> + '_ {
        self.hearthstone_slots.iter().filter_map(|maybe_hearthstone| maybe_hearthstone.as_ref())
    }
}