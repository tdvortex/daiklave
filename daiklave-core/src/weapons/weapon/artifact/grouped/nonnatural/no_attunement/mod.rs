mod memo;
use std::ops::Deref;

pub(crate) use memo::NonnaturalArtifactWeaponNoAttunementMemo;

use crate::{
    hearthstones::SlottedHearthstone,
    weapons::weapon::artifact::{
        inner::ArtifactWeaponInner,
        newtype::{
            OneHandedArtifactWeaponView, TwoHandedArtifactWeaponView, WornArtifactWeaponView,
        },
    },
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum NonnaturalArtifactWeaponNoAttunement<'source> {
    Worn(WornArtifactWeaponView<'source>),
    OneHanded(OneHandedArtifactWeaponView<'source>),
    TwoHanded(TwoHandedArtifactWeaponView<'source>),
}

impl<'source> Deref for NonnaturalArtifactWeaponNoAttunement<'source> {
    type Target = ArtifactWeaponInner<'source>;

    fn deref(&self) -> &Self::Target {
        match self {
            NonnaturalArtifactWeaponNoAttunement::Worn(deref) => deref,
            NonnaturalArtifactWeaponNoAttunement::OneHanded(deref) => deref,
            NonnaturalArtifactWeaponNoAttunement::TwoHanded(deref) => deref,
        }
    }
}

impl<'source> NonnaturalArtifactWeaponNoAttunement<'source> {
    pub fn hearthstone_slots_mut(&mut self) -> &mut Vec<Option<SlottedHearthstone<'source>>> {
        match self {
            NonnaturalArtifactWeaponNoAttunement::Worn(WornArtifactWeaponView(
                ArtifactWeaponInner {
                    book_reference: _,
                    merit_dots: _,
                    magic_material: _,
                    base_weapon_name: _,
                    base_weapon: _,
                    lore: _,
                    powers: _,
                    hearthstone_slots,
                },
            )) => hearthstone_slots,
            NonnaturalArtifactWeaponNoAttunement::OneHanded(OneHandedArtifactWeaponView(
                ArtifactWeaponInner {
                    book_reference: _,
                    merit_dots: _,
                    magic_material: _,
                    base_weapon_name: _,
                    base_weapon: _,
                    lore: _,
                    powers: _,
                    hearthstone_slots,
                },
            )) => hearthstone_slots,
            NonnaturalArtifactWeaponNoAttunement::TwoHanded(TwoHandedArtifactWeaponView(
                ArtifactWeaponInner {
                    book_reference: _,
                    merit_dots: _,
                    magic_material: _,
                    base_weapon_name: _,
                    base_weapon: _,
                    lore: _,
                    powers: _,
                    hearthstone_slots,
                },
            )) => hearthstone_slots,
        }
    }
}
