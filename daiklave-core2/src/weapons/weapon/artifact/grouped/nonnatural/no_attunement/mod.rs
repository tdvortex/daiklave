mod memo;
use std::ops::Deref;

pub(crate) use memo::NonnaturalArtifactWeaponNoAttunementMemo;

use crate::{
    hearthstones::SlottedHearthstone,
    weapons::weapon::artifact::{
        newtype::{
            OneHandedArtifactWeaponView, TwoHandedArtifactWeaponView, WornArtifactWeaponView,
        },
        traits::ArtifactWeaponTraits,
    },
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum NonnaturalArtifactWeaponNoAttunement<'source> {
    Worn(WornArtifactWeaponView<'source>),
    OneHanded(OneHandedArtifactWeaponView<'source>),
    TwoHanded(TwoHandedArtifactWeaponView<'source>),
}

impl<'source> Deref for NonnaturalArtifactWeaponNoAttunement<'source> {
    type Target = ArtifactWeaponTraits<'source>;

    fn deref(&self) -> &Self::Target {
        match self {
            NonnaturalArtifactWeaponNoAttunement::Worn(deref) => deref,
            NonnaturalArtifactWeaponNoAttunement::OneHanded(deref) => deref,
            NonnaturalArtifactWeaponNoAttunement::TwoHanded(deref) => deref,
        }
    }
}

impl<'source> NonnaturalArtifactWeaponNoAttunement<'source> {
    pub fn as_memo(&self) -> NonnaturalArtifactWeaponNoAttunementMemo {
        match self {
            NonnaturalArtifactWeaponNoAttunement::Worn(view) => {
                NonnaturalArtifactWeaponNoAttunementMemo::Worn(view.as_memo())
            }
            NonnaturalArtifactWeaponNoAttunement::OneHanded(view) => {
                NonnaturalArtifactWeaponNoAttunementMemo::OneHanded(view.as_memo())
            }
            NonnaturalArtifactWeaponNoAttunement::TwoHanded(view) => {
                NonnaturalArtifactWeaponNoAttunementMemo::TwoHanded(view.as_memo())
            }
        }
    }

    pub fn hearthstone_slots_mut(&mut self) -> &mut Vec<Option<SlottedHearthstone<'source>>> {
        match self {
            NonnaturalArtifactWeaponNoAttunement::Worn(WornArtifactWeaponView(
                ArtifactWeaponTraits {
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
                ArtifactWeaponTraits {
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
                ArtifactWeaponTraits {
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
