use std::ops::Deref;

use super::equipped::EquipHand;
use super::{base::BaseWeaponMemo, equipped::Equipped};

mod grouped;
mod memo;
mod newtype;

pub(crate) use grouped::{
    HandlessMundaneWeapon, HandlessMundaneWeaponMemo, NonnaturalMundaneWeapon,
    NonnaturalMundaneWeaponMemo,
};
pub use memo::MundaneWeaponMemo;
pub(crate) use newtype::unarmed;
pub(crate) use newtype::{
    NaturalMundaneWeapon, OneHandedMundaneWeapon, TwoHandedMundaneWeapon, WornMundaneWeapon,
};
pub use newtype::{
    NaturalMundaneWeaponMemo, OneHandedMundaneWeaponMemo, TwoHandedMundaneWeaponMemo,
    WornMundaneWeaponMemo,
};

pub(crate) enum MundaneWeapon<'source> {
    Natural(NaturalMundaneWeapon<'source>),
    Worn(WornMundaneWeapon<'source>, bool),
    OneHanded(OneHandedMundaneWeapon<'source>, Option<EquipHand>),
    TwoHanded(TwoHandedMundaneWeapon<'source>, bool),
}

impl<'source> Deref for MundaneWeapon<'source> {
    type Target = BaseWeaponMemo;

    fn deref(&self) -> &Self::Target {
        match self {
            MundaneWeapon::Natural(deref) => deref,
            MundaneWeapon::Worn(deref, _) => deref,
            MundaneWeapon::OneHanded(deref, _) => deref,
            MundaneWeapon::TwoHanded(deref, _) => deref,
        }
    }
}

impl<'source> MundaneWeapon<'source> {
    pub fn is_equipped(&self) -> Option<Equipped> {
        match self {
            MundaneWeapon::Natural(_) => Some(Equipped::Natural),
            MundaneWeapon::Worn(_, is_equipped) => {
                if *is_equipped {
                    Some(Equipped::Worn)
                } else {
                    None
                }
            }
            MundaneWeapon::OneHanded(_, maybe_hand) => match maybe_hand {
                None => None,
                Some(EquipHand::MainHand) => Some(Equipped::MainHand),
                Some(EquipHand::OffHand) => Some(Equipped::OffHand),
            },
            MundaneWeapon::TwoHanded(_, is_equipped) => {
                if *is_equipped {
                    Some(Equipped::TwoHanded)
                } else {
                    None
                }
            }
        }
    }

    pub fn name(&self) -> &'source str {
        match self {
            MundaneWeapon::Natural(weapon) => weapon.name(),
            MundaneWeapon::Worn(weapon, _) => weapon.name(),
            MundaneWeapon::OneHanded(weapon, _) => weapon.name(),
            MundaneWeapon::TwoHanded(weapon, _) => weapon.name(),
        }
    }
}
