mod base;
mod handless;
mod natural;
mod nonnatural;
mod one_handed;
mod two_handed;
mod worn;

use std::ops::Deref;

pub(in crate::weapons) use handless::{HandlessMundaneWeapon, HandlessMundaneWeaponMemo};
pub(in crate::weapons) use natural::NaturalMundaneWeapon;
pub(in crate::weapons) use nonnatural::{NonnaturalMundaneWeapon, NonnaturalMundaneWeaponMemo};
pub(in crate::weapons) use one_handed::{OneHandedMundaneWeapon, OneHandedMundaneWeaponMemo};
pub(in crate::weapons) use two_handed::{TwoHandedMundaneWeapon, TwoHandedMundaneWeaponMemo};
pub(in crate::weapons) use worn::{WornMundaneWeapon, WornMundaneWeaponMemo};

use super::{base::BaseWeapon, EquipHand, Equipped};

pub(in crate::weapons) enum MundaneWeapon<'source> {
    Natural(NaturalMundaneWeapon<'source>),
    Worn(WornMundaneWeapon<'source>, bool),
    OneHanded(OneHandedMundaneWeapon<'source>, Option<EquipHand>),
    TwoHanded(TwoHandedMundaneWeapon<'source>, bool),
}

impl<'source> Deref for MundaneWeapon<'source> {
    type Target = BaseWeapon<'source>;

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
}
