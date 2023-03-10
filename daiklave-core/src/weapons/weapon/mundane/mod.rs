use std::ops::Deref;

use super::equipped::EquipHand;
use super::WeaponTag;
use super::{base::BaseWeapon, equipped::Equipped};

mod add;
mod grouped;
mod memo;
mod name;
mod newtype;
mod remove;

pub use add::AddMundaneWeapon;
pub(crate) use grouped::{
    HandlessMundaneWeapon, HandlessMundaneWeaponMemo, NonnaturalMundaneWeapon,
    NonnaturalMundaneWeaponMemo,
};
pub use memo::MundaneWeapon;
pub(crate) use memo::MundaneWeaponHandedness;
pub use name::MundaneWeaponName;
pub(crate) use newtype::unarmed;
pub(crate) use newtype::{
    NaturalMundaneWeapon, OneHandedMundaneWeaponMemo, TwoHandedMundaneWeapon, WornMundaneWeapon,
};
pub(crate) use newtype::{
    NaturalMundaneWeaponView, OneHandedMundaneWeaponView, TwoHandedMundaneWeaponView,
    WornMundaneWeaponView,
};
pub use remove::RemoveMundaneWeapon;

pub(crate) enum MundaneWeaponView<'source> {
    Natural(NaturalMundaneWeaponView<'source>),
    Worn(WornMundaneWeaponView<'source>, bool),
    OneHanded(OneHandedMundaneWeaponView<'source>, Option<EquipHand>),
    TwoHanded(TwoHandedMundaneWeaponView<'source>, bool),
}

impl<'source> Deref for MundaneWeaponView<'source> {
    type Target = BaseWeapon;

    fn deref(&self) -> &Self::Target {
        match self {
            MundaneWeaponView::Natural(deref) => deref,
            MundaneWeaponView::Worn(deref, _) => deref,
            MundaneWeaponView::OneHanded(deref, _) => deref,
            MundaneWeaponView::TwoHanded(deref, _) => deref,
        }
    }
}

impl<'source> MundaneWeaponView<'source> {
    pub fn is_equipped(&self) -> Option<Equipped> {
        match self {
            MundaneWeaponView::Natural(_) => Some(Equipped::Natural),
            MundaneWeaponView::Worn(_, is_equipped) => {
                if *is_equipped {
                    Some(Equipped::Worn)
                } else {
                    None
                }
            }
            MundaneWeaponView::OneHanded(_, maybe_hand) => match maybe_hand {
                None => None,
                Some(EquipHand::MainHand) => Some(Equipped::MainHand),
                Some(EquipHand::OffHand) => Some(Equipped::OffHand),
            },
            MundaneWeaponView::TwoHanded(_, is_equipped) => {
                if *is_equipped {
                    Some(Equipped::TwoHanded)
                } else {
                    None
                }
            }
        }
    }

    pub fn tags(&self) -> std::vec::IntoIter<WeaponTag> {
        match self {
            MundaneWeaponView::Natural(base) => base.tags(WeaponTag::Natural),
            MundaneWeaponView::Worn(base, _) => base.tags(WeaponTag::Worn),
            MundaneWeaponView::OneHanded(base, _) => base.tags(WeaponTag::OneHanded),
            MundaneWeaponView::TwoHanded(base, _) => base.tags(WeaponTag::TwoHanded),
        }
    }
}
