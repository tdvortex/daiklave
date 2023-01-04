mod base;
mod handless;
mod nonnatural;
mod natural;
mod one_handed;
mod two_handed;
mod worn;

use std::ops::Deref;

pub(in crate::weapons) use handless::{HandlessMundaneWeapon, HandlessMundaneWeaponMemo};
pub(in crate::weapons) use nonnatural::{NonnaturalMundaneWeapon, NonnaturalMundaneWeaponMemo};
pub use one_handed::{OneHandedMundaneWeapon, OneHandedMundaneWeaponMemo};
pub use two_handed::{TwoHandedMundaneWeapon, TwoHandedMundaneWeaponMemo};
pub use natural::{NaturalMundaneWeapon, NaturalMundaneWeaponMemo};
pub use worn::{WornMundaneWeapon, WornMundaneWeaponMemo};

use super::{base::BaseWeapon, Equipped, EquipHand};

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
            MundaneWeapon::Worn(deref, is_worn) => deref,
            MundaneWeapon::OneHanded(deref, maybe_hand) => deref,
            MundaneWeapon::TwoHanded(deref, is_equipped) => deref,
        }
    }
}

impl<'source> MundaneWeapon<'source> {
    pub fn is_equipped(&self) -> Option<Equipped> {
        match self {
            MundaneWeapon::Natural(_) => Some(Equipped::Natural),
            MundaneWeapon::Worn(_, is_equipped) => if *is_equipped {
                Some(Equipped::Worn)
            } else {
                None
            },
            MundaneWeapon::OneHanded(_, maybe_hand) => match maybe_hand {
                None => None,
                Some(EquipHand::MainHand) => Some(Equipped::MainHand),
                Some(EquipHand::OffHand) => Some(Equipped::OffHand),
            }
            MundaneWeapon::TwoHanded(_, is_equipped) => if *is_equipped {
                Some(Equipped::TwoHanded)
            } else {
                None
            }, 
        }
    }
}