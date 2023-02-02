mod memo;

pub use memo::ArtifactWeaponInnerMemo;

use crate::{
    artifact::{ArtifactName, MagicMaterial},
    book_reference::BookReference,
    hearthstones::{hearthstone::Hearthstone, HearthstonePosition, SlottedHearthstone},
    weapons::weapon::base::BaseWeapon,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ArtifactWeaponInner<'source> {
    pub(crate) book_reference: Option<BookReference>,
    pub(crate) merit_dots: u8,
    pub(crate) magic_material: MagicMaterial,
    pub(crate) base_weapon_name: &'source str,
    pub(crate) base_weapon: &'source BaseWeapon,
    pub(crate) lore: Option<&'source str>,
    pub(crate) powers: Option<&'source str>,
    pub(crate) hearthstone_slots: Vec<Option<SlottedHearthstone<'source>>>,
}

impl<'source> From<&'source ArtifactWeaponInnerMemo> for ArtifactWeaponInner<'source> {
    fn from(memo: &'source ArtifactWeaponInnerMemo) -> Self {
        Self {
            book_reference: memo.book_reference,
            merit_dots: memo.merit_dots,
            magic_material: memo.magic_material,
            base_weapon_name: &memo.base_weapon_name,
            base_weapon: &memo.base_weapon,
            lore: memo.lore.as_deref(),
            powers: memo.powers.as_deref(),
            hearthstone_slots: memo
                .hearthstone_slots
                .iter()
                .map(|maybe_hearthstone| {
                    maybe_hearthstone
                        .as_ref()
                        .map(|hearthstone| hearthstone.into())
                })
                .collect(),
        }
    }
}

impl<'view, 'source> ArtifactWeaponInner<'source> {
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
                        ArtifactName::Weapon(name),
                        *slotted,
                    ))
                })
            })
    }
}
