mod handless;
mod natural;
mod nonnatural;
mod one_handed;
mod two_handed;
mod worn;

use std::ops::Deref;

pub(in crate::weapons) use handless::{HandlessMundaneWeapon, HandlessMundaneWeaponMemo};
pub(in crate::weapons) use natural::{NaturalMundaneWeapon, NaturalMundaneWeaponMemo};
pub(in crate::weapons) use nonnatural::{NonnaturalMundaneWeapon, NonnaturalMundaneWeaponMemo};
pub(in crate::weapons) use one_handed::{OneHandedMundaneWeapon, OneHandedMundaneWeaponMemo};
pub(in crate::weapons) use two_handed::{TwoHandedMundaneWeapon, TwoHandedMundaneWeaponMemo};
pub(in crate::weapons) use worn::{WornMundaneWeapon, WornMundaneWeaponMemo};

use super::{base::{BaseWeaponMemo}, EquipHand, Equipped};

pub enum MundaneWeapon<'source> {
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
    pub fn as_memo(&self) -> MundaneWeaponMemo {
        todo!()
    }

    pub fn name(&self) -> &'source str {
        match self {
            MundaneWeapon::Natural(weapon) => weapon.0.name.as_str(),
            MundaneWeapon::Worn(weapon, _) => weapon.0.name.as_str(),
            MundaneWeapon::OneHanded(weapon, _) => weapon.0.name.as_str(),
            MundaneWeapon::TwoHanded(weapon, _) => weapon.0.name.as_str(),
        }
    }

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
}

/// An owned copy of a Mundane Weapon.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MundaneWeaponMemo {
    /// A Natural weapon.
    Natural(NaturalMundaneWeaponMemo),
    /// A Worn weapon, and whether it is equipped.
    Worn(WornMundaneWeaponMemo, bool),
    /// A OneHanded weapon, and the hand it's equipped in (if any).
    OneHanded(OneHandedMundaneWeaponMemo, Option<EquipHand>),
    /// A TwoHanded weapon, and whether it is equipped.
    TwoHanded(TwoHandedMundaneWeaponMemo, bool),
}