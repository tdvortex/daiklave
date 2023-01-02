use crate::{book_reference::BookReference, weapons::{BaseWeaponId, hearthstone::OwnedHearthstone}};

use super::base::BaseArtifactWeapon;

pub(in crate::weapons::artifact) struct NamedArtifactWeapon<'source> {
    name: &'source str,
    book_reference: Option<BookReference>,
    merit_dots: u8,
    base_weapon_id: BaseWeaponId,
    base_weapon: BaseArtifactWeapon<'source>,
    lore: Option<String>,
    powers: Option<String>,
    hearthstone_slots: Vec<Option<OwnedHearthstone<'source>>>,
}